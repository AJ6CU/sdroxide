//! Spoken-announcement preferences (persisted in `config.toml` under
//! `[speech]`). Kept wasm-safe (no I/O), like every other settings struct here.
//!
//! This is a *client-side* preference, in the same sense as `[ui]`: it
//! describes what the operator sitting at this screen wants to hear, not how
//! the station is configured. It never travels as a [`crate::Command`], and no
//! tab has to wait for the engine to seed it.

use serde::{Deserialize, Serialize};

/// How much to say.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verbosity {
    /// Only what changed, in as few words as carry it: "twenty meters".
    Terse,
    /// The default. Adds the numbers an operator wants with the change — the
    /// frequency after a band change, the signal report with a call.
    #[default]
    Normal,
    /// Adds units, band segments, and the settings that normally stay quiet.
    Full,
}

impl Verbosity {
    pub const ALL: [Verbosity; 3] = [Verbosity::Terse, Verbosity::Normal, Verbosity::Full];

    pub fn label(self) -> &'static str {
        match self {
            Verbosity::Terse => "Terse",
            Verbosity::Normal => "Normal",
            Verbosity::Full => "Full",
        }
    }
}

/// How to read a frequency out.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FreqStyle {
    /// How an operator reads a dial: "fourteen point zero seven four".
    #[default]
    Dial,
    /// Every digit, for when there must be no doubt at all:
    /// "one four zero seven four zero zero zero". Slower, and immune to
    /// mishearing a decimal point.
    Digits,
    /// `Dial` plus the unit: "... megahertz".
    Full,
}

impl FreqStyle {
    pub const ALL: [FreqStyle; 3] = [FreqStyle::Dial, FreqStyle::Digits, FreqStyle::Full];

    pub fn label(self) -> &'static str {
        match self {
            FreqStyle::Dial => "Dial (14 point 074)",
            FreqStyle::Digits => "Every digit",
            FreqStyle::Full => "Dial with units",
        }
    }
}

/// How to read a callsign.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallsignStyle {
    /// NATO phonetics: "kilo one alpha bravo charlie". The default, because a
    /// callsign is the one token that must not be misheard.
    #[default]
    Phonetic,
    /// Letter names: "kay one ay bee see". Quicker, if you are used to it.
    Letters,
    /// Hand the raw text to the voice and let it do what it will. Fastest, and
    /// usually unintelligible.
    AsIs,
}

impl CallsignStyle {
    pub const ALL: [CallsignStyle; 3] =
        [CallsignStyle::Phonetic, CallsignStyle::Letters, CallsignStyle::AsIs];

    pub fn label(self) -> &'static str {
        match self {
            CallsignStyle::Phonetic => "Phonetic (kilo one alpha)",
            CallsignStyle::Letters => "Letters (kay one ay)",
            CallsignStyle::AsIs => "As written",
        }
    }
}

/// Which changes to the radio get announced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CategoryFlags {
    /// The VFO frequency, once the dial stops moving.
    pub frequency: bool,
    /// Mode and band.
    pub mode_band: bool,
    /// A/B, swap, and split.
    pub vfo_split: bool,
    /// AGC setting, AGC ceiling, and the manual gain behind it.
    pub agc_gain: bool,
    /// Drive, tune level, mic gain, AF volume.
    pub levels: bool,
    /// Filter width and shift, squelch, noise blanker, noise reduction, notch.
    /// Off by default: these move constantly while chasing a signal.
    pub filters: bool,
    /// Going to transmit and coming back.
    pub ptt: bool,
    pub rit_xit: bool,
    /// Memory recall, and the scanner stopping on a channel.
    pub memory_scan: bool,
    /// The engine's own messages — the banner text, and losing the connection.
    pub notices: bool,
    /// Crossing out of, or back into, an amateur allocation.
    pub band_edge: bool,
}

impl Default for CategoryFlags {
    fn default() -> Self {
        CategoryFlags {
            frequency: true,
            mode_band: true,
            vfo_split: true,
            agc_gain: true,
            levels: true,
            filters: false,
            ptt: true,
            rit_xit: true,
            memory_scan: true,
            notices: true,
            band_edge: true,
        }
    }
}

/// Which decoded messages get read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DecodeSpeech {
    /// FT8/FT4 decodes addressed to your callsign.
    pub ft8_to_me: bool,
    /// Directed CQs you could answer. Off by default: a busy evening on
    /// twenty metres is a hundred of these a minute.
    pub ft8_cq_for_me: bool,
    /// JS8 messages addressed to you personally.
    pub js8: bool,
    /// JS8 `@ALLCALL` traffic as well.
    pub js8_allcall: bool,
    /// Read a JS8 message before every frame has arrived.
    pub js8_partial: bool,
    /// FSQ messages addressed to you.
    pub fsq: bool,
    /// Your own side of an FT8 exchange: what the sequencer is about to send,
    /// and the contact ending. Without it the operator hears half a
    /// conversation — every message the other station sends, and no sign that
    /// the QSO was ever finished and logged.
    pub ft8_qso: bool,
    /// Append the signal report.
    ///
    /// Only where the message carries no report of its own: a decode that says
    /// "reports minus twelve" is already a number, and ours belongs on the call
    /// that started the contact, not on every message of it.
    pub include_snr: bool,
    /// Append the grid square.
    pub include_grid: bool,
}

impl Default for DecodeSpeech {
    fn default() -> Self {
        DecodeSpeech {
            ft8_to_me: true,
            ft8_cq_for_me: false,
            js8: true,
            js8_allcall: false,
            js8_partial: false,
            fsq: true,
            ft8_qso: true,
            include_snr: true,
            include_grid: false,
        }
    }
}

/// Reading the keyboard modes' decoded text aloud.
///
/// Both streams default to off. A CW or RTTY decoder produces text faster than
/// speech can read it, and an operator who wants a running commentary should
/// ask for one rather than be given it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TextSpeech {
    pub cw: bool,
    /// RTTY, PSK31, Olivia, THOR and FSQ share one decoded-text stream.
    pub rtty_psk: bool,
    /// Only read CW while the decoder reports a stable timing fit. Reading an
    /// unlocked CW decoder's output is worse than silence.
    pub cw_only_when_locked: bool,
    /// Characters of undelivered text to hold before dropping the oldest.
    /// Roughly thirty seconds of speech at the default.
    pub max_backlog_chars: usize,
}

impl Default for TextSpeech {
    fn default() -> Self {
        TextSpeech { cw: false, rtty_psk: false, cw_only_when_locked: true, max_backlog_chars: 400 }
    }
}

/// Reading SWR out while tuning up.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TuneSpeech {
    /// Read the SWR out periodically while TUNE is held.
    pub swr_while_tuning: bool,
    /// How often, in seconds.
    pub period_s: f32,
    /// Interrupt with a warning at or above this SWR. Three to one is where a
    /// solid-state PA starts folding back.
    pub alarm_swr: f32,
    /// Warn during an ordinary transmission too, not only while tuning.
    pub alarm_always: bool,
    /// Report the best match reached, once TUNE is released.
    pub summary_after_tune: bool,
}

impl Default for TuneSpeech {
    fn default() -> Self {
        TuneSpeech {
            swr_while_tuning: true,
            period_s: 2.0,
            alarm_swr: 3.0,
            alarm_always: true,
            summary_after_tune: true,
        }
    }
}

/// Everything the speech layer is configured by.
///
/// **Field order matters.** `sdroxide-config` writes this as a TOML table and
/// serde emits fields in declaration order, so every scalar must precede every
/// sub-table — a table declared above a scalar swallows it on the next write.
/// The same rule applies to where `speech` itself sits in `Settings`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SpeechSettings {
    // ── scalars ─────────────────────────────────────────────────────────
    /// Master switch. Off until the operator asks for it: a radio that starts
    /// talking on first run is a bug, not a feature.
    pub enabled: bool,
    /// Voice basename, e.g. `en_US-hfc_female-medium`. Empty means the shipped
    /// voice, or the first one found.
    pub voice: String,
    /// Speech rate multiplier. Divides the voice's own `length_scale`, so the
    /// model stretches the phoneme durations itself — no resampling and no
    /// pitch shift.
    ///
    /// The upper end is 2.0 rather than something higher because VITS rounds
    /// each phoneme up to a whole frame: past about 2× the durations stop
    /// shrinking and the slider stops doing anything.
    pub rate: f32,
    /// Speech volume, independent of the radio's AF gain.
    pub volume: f32,
    /// Sound device for speech. `None` is the system default. Speech has its
    /// own output stream, so this can be a different device from the radio's —
    /// announcements in the room, the band in the headphones.
    pub device: Option<String>,
    /// Stay quiet below an alert while the transmitter is keyed. Speech goes
    /// to the operator's speakers, and therefore into the microphone.
    pub duck_on_ptt: bool,
    /// Attenuate receiver audio while an announcement plays.
    pub duck_rx: bool,
    /// What to attenuate it to. 0.25 is about 12 dB down.
    pub duck_level: f32,
    pub verbosity: Verbosity,
    pub freq_style: FreqStyle,
    pub callsign_style: CallsignStyle,

    // ── sub-tables: everything below emits as a TOML table ───────────────
    pub cat: CategoryFlags,
    pub decodes: DecodeSpeech,
    pub text: TextSpeech,
    pub tune: TuneSpeech,
}

impl Default for SpeechSettings {
    fn default() -> Self {
        SpeechSettings {
            enabled: false,
            voice: String::new(),
            rate: 1.0,
            volume: 0.8,
            device: None,
            duck_on_ptt: true,
            duck_rx: true,
            duck_level: 0.25,
            verbosity: Verbosity::Normal,
            freq_style: FreqStyle::Dial,
            callsign_style: CallsignStyle::Phonetic,
            cat: CategoryFlags::default(),
            decodes: DecodeSpeech::default(),
            text: TextSpeech::default(),
            tune: TuneSpeech::default(),
        }
    }
}

impl SpeechSettings {
    /// Selectable speech rates for the settings combo.
    pub const RATE_RANGE: std::ops::RangeInclusive<f32> = 0.5..=2.0;

    /// Rate clamped to a sane range (guards a hand-edited config).
    pub fn rate(&self) -> f32 {
        self.rate.clamp(0.5, 2.0)
    }

    /// Volume clamped to a sane range.
    pub fn volume(&self) -> f32 {
        self.volume.clamp(0.0, 1.0)
    }

    /// Receiver gain to apply while speaking, or 1.0 when ducking is off.
    pub fn duck_gain(&self) -> f32 {
        if self.duck_rx { self.duck_level.clamp(0.0, 1.0) } else { 1.0 }
    }

    /// How often to read SWR while tuning, clamped.
    pub fn swr_period_s(&self) -> f64 {
        self.tune.period_s.clamp(1.0, 10.0) as f64
    }

    pub fn is_verbose(&self) -> bool {
        self.verbosity == Verbosity::Full
    }

    pub fn is_terse(&self) -> bool {
        self.verbosity == Verbosity::Terse
    }
}
