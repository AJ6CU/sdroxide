//! Callsign-lookup result, shared by the native lookup clients (QRZ/HamQTH in
//! `sdroxide-net`), the wire protocol, and the UI. Pure data + serde.

use serde::{Deserialize, Serialize};

/// Biographical / location data resolved for a callsign. Every field is
/// optional: providers return different subsets and some require a paid
/// subscription for the full record. The UI merges the populated fields into
/// the active log entry without overwriting what the operator already typed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallsignInfo {
    /// The callsign this record is for (uppercased), echoed back so a late
    /// result can be matched to the right log entry.
    pub call: String,
    /// Operator name (first + last, as the provider formats it).
    pub name: Option<String>,
    /// City / town (QTH).
    pub qth: Option<String>,
    /// Maidenhead grid locator.
    pub grid: Option<String>,
    /// US state (2-letter) or other primary administrative subdivision.
    pub state: Option<String>,
    /// US county, if provided.
    pub county: Option<String>,
    /// Country / DXCC entity name.
    pub country: Option<String>,
    /// DXCC entity number, if the provider returns it.
    pub dxcc: Option<u16>,
    /// CQ zone (1..40).
    pub cq_zone: Option<u8>,
    /// ITU zone (1..90).
    pub itu_zone: Option<u8>,
    /// QSL routing hint / manager, if any.
    pub qsl_via: Option<String>,
    // ── The QRZ card. Appended last, as the wire requires: `CallsignInfo`
    // rides `ServerMsg::CallsignResult`, and postcard is positional. ──
    /// Street address (QRZ `addr1`).
    #[serde(default)]
    pub addr1: Option<String>,
    /// Contact e-mail, when QRZ gives one (subscribers only).
    #[serde(default)]
    pub email: Option<String>,
    /// Age in years, worked out **on the station** from the date of birth,
    /// which itself never leaves it.
    #[serde(default)]
    pub age: Option<u8>,
    /// Licence class, as QRZ writes it.
    #[serde(default)]
    pub class: Option<String>,
    /// The operator answers QSL by LoTW / eQSL / paper.
    #[serde(default)]
    pub qsl_lotw: bool,
    #[serde(default)]
    pub qsl_eqsl: bool,
    #[serde(default)]
    pub qsl_paper: bool,
    /// The operator's picture, reduced on the station to a small JPEG so that
    /// it crosses the link cheaply. `None` when QRZ has none or the card is off.
    #[serde(default)]
    pub photo: Option<Vec<u8>>,
}

impl CallsignInfo {
    pub fn for_call(call: &str) -> Self {
        CallsignInfo { call: call.trim().to_ascii_uppercase(), ..Default::default() }
    }

    /// True when the lookup produced nothing usable.
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.qth.is_none()
            && self.grid.is_none()
            && self.state.is_none()
            && self.county.is_none()
            && self.country.is_none()
            && self.dxcc.is_none()
            && self.cq_zone.is_none()
            && self.itu_zone.is_none()
            && self.qsl_via.is_none()
            && self.addr1.is_none()
            && self.email.is_none()
            && self.age.is_none()
            && self.class.is_none()
            && !self.qsl_lotw
            && !self.qsl_eqsl
            && !self.qsl_paper
            && self.photo.is_none()
    }
}

/// Where to send a one-click QSO upload. Matches the credentials in
/// [`crate` network config] and the upload clients in `sdroxide-net`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UploadTarget {
    Eqsl,
    QrzLogbook,
    ClubLog,
    /// The HamQTH logbook, via its real-time QSO endpoint. Appended last:
    /// postcard numbers variants by declaration index, so inserting it next to
    /// QRZ (where it belongs on screen) would renumber Club Log on the wire.
    HamQth,
    /// The World Radio League logbook, via its developer API (issue #337).
    /// Appended for the reason [`UploadTarget::HamQth`] gives.
    Wrl,
    /// The LOG11DX 11 m logbook, via the same JSON API its WSJT-X bridge uses.
    /// Appended for the reason [`UploadTarget::HamQth`] gives.
    Log11Dx,
    /// The operator's **own** Wavelog, reached at whatever address they gave
    /// it. Appended for the reason [`UploadTarget::HamQth`] gives.
    Wavelog,
}

impl UploadTarget {
    pub fn label(self) -> &'static str {
        match self {
            UploadTarget::Eqsl => "eQSL",
            UploadTarget::QrzLogbook => "QRZ",
            UploadTarget::ClubLog => "Club Log",
            UploadTarget::HamQth => "HamQTH",
            UploadTarget::Wrl => "WRL",
            UploadTarget::Log11Dx => "LOG11DX",
            UploadTarget::Wavelog => "Wavelog",
        }
    }

    /// The service whose stored credentials this target's are, for a
    /// "Test this login" button beside it.
    ///
    /// One-way on purpose. Every upload target is testable, so this mapping is
    /// total; the reverse is not — LoTW can be tested and cannot be uploaded to
    /// — which is why [`LoginTarget`] stays an enum of its own rather than
    /// becoming a method on this one.
    pub fn login_target(self) -> LoginTarget {
        match self {
            UploadTarget::Eqsl => LoginTarget::Eqsl,
            UploadTarget::QrzLogbook => LoginTarget::QrzLogbook,
            UploadTarget::ClubLog => LoginTarget::ClubLog,
            UploadTarget::HamQth => LoginTarget::HamQth,
            UploadTarget::Wrl => LoginTarget::Wrl,
            UploadTarget::Log11Dx => LoginTarget::Log11Dx,
            UploadTarget::Wavelog => LoginTarget::Wavelog,
        }
    }

    /// Display order, which is deliberately not the declaration order above:
    /// the variants are numbered by the wire and appended to, while this is
    /// what the operator reads along a tab strip.
    pub const ALL: [UploadTarget; 7] = [
        // The operator's own server first: it is the one they chose rather than
        // signed up to, and the one whose address and key they had to type.
        UploadTarget::Wavelog,
        UploadTarget::QrzLogbook,
        UploadTarget::Eqsl,
        UploadTarget::HamQth,
        UploadTarget::ClubLog,
        UploadTarget::Wrl,
        UploadTarget::Log11Dx,
    ];
}

/// A service whose stored credentials can be checked without logging a QSO.
///
/// Deliberately a separate enum from [`UploadTarget`] rather than an extension
/// of it. LoTW is testable but is not an upload target (its upload needs TQSL
/// signing), and the two lists would drift apart the moment a download-only
/// service was added.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoginTarget {
    Eqsl,
    QrzLogbook,
    ClubLog,
    Lotw,
    /// The HamQTH account — the *same* username and password the callsign
    /// lookup uses, because HamQTH has one account per operator and the
    /// real-time logbook endpoint authenticates with it directly.
    HamQth,
    /// The World Radio League developer API key (issue #337). Appended for the
    /// reason [`UploadTarget::HamQth`] gives.
    Wrl,
    /// The LOG11DX API token. Appended for the reason [`UploadTarget::HamQth`]
    /// gives.
    Log11Dx,
    /// The operator's own Wavelog: its address and an API key from its own API
    /// page — not a username and password. Appended for the reason
    /// [`UploadTarget::HamQth`] gives.
    Wavelog,
}

impl LoginTarget {
    pub fn label(self) -> &'static str {
        match self {
            LoginTarget::Eqsl => "eQSL",
            LoginTarget::QrzLogbook => "QRZ Logbook",
            LoginTarget::ClubLog => "Club Log",
            LoginTarget::Lotw => "LoTW",
            LoginTarget::HamQth => "HamQTH",
            LoginTarget::Wrl => "World Radio League",
            LoginTarget::Log11Dx => "LOG11DX",
            LoginTarget::Wavelog => "Wavelog",
        }
    }

    pub const ALL: [LoginTarget; 8] = [
        LoginTarget::Eqsl,
        LoginTarget::QrzLogbook,
        LoginTarget::HamQth,
        LoginTarget::ClubLog,
        LoginTarget::Lotw,
        LoginTarget::Wrl,
        LoginTarget::Log11Dx,
        LoginTarget::Wavelog,
    ];
}

/// The outcome of checking one service's stored credentials.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginTestResult {
    pub target: LoginTarget,
    pub ok: bool,
    /// What the service said, or why the check could not be made.
    pub message: String,
}

/// The outcome of an upload attempt for one QSO to one target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadResult {
    /// The logbook id of the QSO this result is for.
    pub qso_id: u64,
    pub target: UploadTarget,
    pub ok: bool,
    /// Server message or error detail.
    pub message: String,
    /// On a failure, whether trying again later could succeed: the server was
    /// unreachable or busy, not refusing this QSO. Only Wavelog says, since
    /// only Wavelog keeps a retry queue; the other services leave it false.
    /// Appended last, as the wire requires.
    #[serde(default)]
    pub retryable: bool,
}
