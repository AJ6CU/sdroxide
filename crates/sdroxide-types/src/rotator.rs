//! The antenna-rotator client's configuration.
//!
//! sdroxide points a motorized antenna by talking *to* a Hamlib `rotctld`
//! daemon over TCP — the opposite direction from the built-in rigctld
//! *server*. One text protocol reaches every rotator Hamlib can drive, which
//! is essentially all of them, without this codebase growing a serial driver
//! per controller.
//!
//! Stations that would rather not run a daemon can speak a controller directly
//! over a serial port instead — [`RotatorTransport::EasyComm2`] (SatNOGS-style
//! homebrew) and [`RotatorTransport::Gs232`] (Yaesu's own interfaces). Both are
//! line protocols simpler than rotctld's and slot in behind the same
//! engine-side interface — "here is az/el, go" — without the engine noticing.
//!
//! # Who may move the antenna
//!
//! [`RotatorAuthority`] decides. `Auto` lets the satellite lock drive the
//! antenna and parks it with no lock; `Manual` holds an operator's own target
//! and a satellite lock does not override it. Without that distinction a manual
//! point would be undone by the next tracking tick — a control that silently
//! does nothing, which this fork treats as a bug in its own right.

use serde::{Deserialize, Serialize};

/// How the antenna is reached.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotatorTransport {
    /// A Hamlib `rotctld` daemon over TCP. One protocol reaches every rotator
    /// Hamlib drives — GS-232, EasyComm, SPID, AlfaSpid and the rest — so this
    /// is the default and the route that needs no hardware in hand to trust.
    Rotctld,
    /// EasyComm II over a serial port — the SatNOGS-style homebrew controllers.
    /// `AZnnn.n ELnnn.n` to point, `AZ` / `EL` to read back, `STOP` to halt.
    EasyComm2,
    /// Yaesu GS-232A/B over a serial port. `Mnnn` (A) or `Mnnn eee` (B) to
    /// point, `C` to read back, `S` to halt.
    Gs232,
    /// **PSTRotator's own UDP control port**, spoken directly rather than
    /// through a rotctld daemon (issue #638).
    ///
    /// Added to this enum rather than as a separate one beside it: a second
    /// enum holding the same choice would leave the config with two fields
    /// answering "how is the antenna reached", and only one of them wired.
    /// PSTRotator answers commands on `port` and replies on `port + 1`, which
    /// is why `serial_port` and `baud` are unused here.
    PsRotator,
}

impl Default for RotatorTransport {
    fn default() -> Self {
        RotatorTransport::Rotctld
    }
}

/// Who is allowed to move the antenna.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotatorAuthority {
    /// The satellite lock drives the antenna; with no lock it parks. This is
    /// the default, and the state a satellite lock takes the moment it is
    /// armed — locking a bird is the operator asking the antenna to follow it.
    Auto,
    /// The operator's own target drives the antenna and a satellite lock does
    /// not override it. Set by [`Command::PointRotator`], cleared by
    /// [`Command::SetRotatorAuthority`].
    Manual,
}

impl Default for RotatorAuthority {
    fn default() -> Self {
        RotatorAuthority::Auto
    }
}

/// Where the rotator lives and how eagerly to chase the satellite.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RotatorConfig {
    /// Master switch. Off, the client thread does not exist and nothing
    /// connects anywhere.
    pub enabled: bool,
    pub host: String,
    /// Hamlib's registered rotctld port.
    pub port: u16,
    /// How the antenna is reached. [`RotatorTransport::Rotctld`] (the default)
    /// dials a daemon; the serial transports talk straight to a controller.
    pub transport: RotatorTransport,
    /// Serial port for the serial transports, e.g. `/dev/ttyUSB0`. Unused by
    /// [`RotatorTransport::Rotctld`].
    pub serial_port: String,
    /// Baud rate for the serial transports. Unused by
    /// [`RotatorTransport::Rotctld`].
    pub baud: u32,
    /// Below this elevation the rotator parks instead of tracking: no point
    /// grinding the motors at a satellite behind the neighbour's roofline.
    pub min_el_deg: f64,
    /// Added to every commanded azimuth — for a rotator whose north calibration
    /// is off by a known amount.
    pub az_offset_deg: f64,
    /// Movements smaller than this are not sent. Chasing tenths of a degree
    /// wears hardware for nothing; a degree is well inside any amateur
    /// antenna's beamwidth.
    pub min_move_deg: f64,
    /// Where to point when idle — `None` leaves the rotator wherever it was.
    pub park: Option<(f64, f64)>,
    /// Which program is at `host:port`. Appended last: this rides the remote
    /// link positionally. `Rotctld` in a `rotator.json` written before it.
    pub protocol: RotatorProtocol,
}

/// The rotator program sdroxide talks to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotatorProtocol {
    /// Hamlib `rotctld`, over TCP.
    #[default]
    Rotctld,
    /// PSTRotator (YO3DMU), over its UDP control port. It answers position
    /// queries on that port plus one.
    PstRotator,
}

impl RotatorProtocol {
    pub const ALL: [RotatorProtocol; 2] = [RotatorProtocol::Rotctld, RotatorProtocol::PstRotator];

    pub fn label(self) -> &'static str {
        match self {
            RotatorProtocol::Rotctld => "Hamlib rotctld (TCP)",
            RotatorProtocol::PstRotator => "PSTRotator (UDP)",
        }
    }

    /// The port each program listens on out of the box.
    pub fn default_port(self) -> u16 {
        match self {
            RotatorProtocol::Rotctld => 4533,
            RotatorProtocol::PstRotator => 12000,
        }
    }
}

impl Default for RotatorConfig {
    fn default() -> Self {
        RotatorConfig {
            enabled: false,
            host: "127.0.0.1".into(),
            port: 4533,
            transport: RotatorTransport::Rotctld,
            serial_port: String::new(),
            baud: 9600,
            min_el_deg: 0.0,
            az_offset_deg: 0.0,
            min_move_deg: 1.0,
            park: None,
            protocol: RotatorProtocol::Rotctld,
        }
    }
}

impl RotatorConfig {
    /// What to dial, as one string.
    pub fn address(&self) -> String {
        format!("{}:{}", self.host.trim(), self.port)
    }
}
