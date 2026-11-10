//! Configuration for the network "cockpit" features: spot feeds (DX cluster,
//! POTA, SOTA, PSK Reporter, FreeDV Reporter), callsign lookup, and QSO upload.
//! Pure data + serde, persisted by `sdroxide-config` as `net.json` and carried
//! to the engine by [`crate::Command::SetNetworkConfig`].
//!
//! Deliberately **not** here: the operator's callsign and grid. Those live in
//! [`crate::DigiConfig`] and are the same identity everywhere in the app; the
//! engine hands them to the network workers separately. A second copy would
//! only give the two a way to disagree.
//!
//! Credentials are stored in plaintext (matching the existing config
//! convention). Every field defaults so an older/absent file always loads.

use serde::{Deserialize, Serialize};

/// A DX-cluster telnet node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ClusterConfig {
    pub enabled: bool,
    /// Host name of the telnet cluster node.
    pub host: String,
    /// TCP port (commonly 7300/7373/8000).
    pub port: u16,
    /// Login callsign sent at the node's `login:` prompt. Falls back to the
    /// operator callsign when empty.
    pub login: String,
    /// Extra commands sent after login (e.g. `SET/FT8`, band/spotter filters),
    /// one per line.
    pub commands: Vec<String>,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        ClusterConfig {
            enabled: false,
            host: String::new(),
            port: 7373,
            login: String::new(),
            commands: Vec::new(),
        }
    }
}

/// The Reverse Beacon Network's aggregated skimmer feed.
///
/// Speaks the same telnet protocol as a DX cluster node and is deliberately a
/// separate config anyway, because it is not one: RBN carries every signal
/// every skimmer hears, several times each, and the spots go to the
/// propagation map rather than into the spot list. Merging the two settings
/// would invite pointing a cluster at RBN's port, and RBN asks specifically
/// that its feed never be re-injected into the cluster network.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RbnConfig {
    pub enabled: bool,
    pub host: String,
    /// 7000 for the CW/RTTY skimmers, 7001 for the FT8/FT4 ones.
    pub port: u16,
    /// Login callsign. RBN wants a real one; falls back to the operator
    /// callsign when empty, as the cluster does.
    pub login: String,
    /// Commands sent after login, one per line. The place to narrow the
    /// firehose — a DX Spider node accepts `set/filter` lines, and an operator
    /// who only cares about their own continent should say so here rather than
    /// download the world and discard it.
    pub commands: Vec<String>,
}

impl Default for RbnConfig {
    fn default() -> Self {
        RbnConfig {
            // On by default, unlike every other feed here, because this one
            // costs the operator nothing and answers a question they always
            // have. It puts nothing on the air, needs no account, and is the
            // only thing that can tell a station monitoring one band what the
            // other nine are doing. A callsign is still required to connect —
            // see `SpotManager::rebuild_rbn` — so a fresh install with no
            // operator identity set opens no socket.
            enabled: true,
            host: "telnet.reversebeacon.net".into(),
            port: 7000,
            login: String::new(),
            commands: Vec::new(),
        }
    }
}

impl RbnConfig {
    /// The second skimmer connection: the FT8/FT4 feed on 7001. Off by default,
    /// unlike the first, because it opens a second telnet session and adds a
    /// large number of spots to the propagation map.
    pub fn second() -> Self {
        RbnConfig { enabled: false, port: 7001, ..RbnConfig::default() }
    }

    /// Whether `self` and `other` would open the same connection, which sends
    /// every path twice. Compared on host and port only: a different login or
    /// filter on the same node is still the same feed.
    pub fn same_feed(&self, other: &RbnConfig) -> bool {
        self.port == other.port && self.host.trim().eq_ignore_ascii_case(other.host.trim())
    }
}

/// A polled HTTP spot feed (POTA / SOTA).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FeedConfig {
    pub enabled: bool,
    /// Poll interval in seconds (clamped to a sane minimum by the client).
    pub interval_secs: u32,
}

impl Default for FeedConfig {
    fn default() -> Self {
        FeedConfig { enabled: false, interval_secs: 60 }
    }
}

/// PSK Reporter reception-report retrieval, for a "who is active on this band"
/// overlay.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PskConfig {
    pub enabled: bool,
    /// Poll interval (PSK Reporter asks for ≥ 300 s between queries).
    pub interval_secs: u32,
    /// Upload what *we* decode as reception reports, so this station appears as
    /// a receiver on the map. Independent of the retrieval half above.
    pub report: bool,
    /// Collector host. Empty falls back to the public one. The project also
    /// runs a test collector on port 14739 that never reaches the live map.
    pub host: String,
    pub port: u16,
    /// Free-text antenna description sent with the receiver record ("dipole at
    /// 10 m", "vertical"). Optional.
    pub antenna: String,
}

impl Default for PskConfig {
    fn default() -> Self {
        PskConfig {
            enabled: false,
            interval_secs: 300,
            report: false,
            host: "report.pskreporter.info".to_string(),
            port: 4739,
            antenna: String::new(),
        }
    }
}

/// FreeDV Reporter (<https://qso.freedv.org/>): announce our station and
/// surface everyone else's as spots. Both halves ride one Socket.IO connection.
///
/// The reported identity is the operator's callsign and grid, set once on the
/// General tab, rather than a copy of its own. With either unset we connect
/// read-only and never appear on the site.
///
/// We are only *shown* to others while the radio is in RADE — in any other mode
/// the connection stays up but the station is hidden, matching FreeDV GUI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FreeDvReporterConfig {
    pub enabled: bool,
    /// Reporter host name. Empty falls back to the public server.
    pub host: String,
    pub port: u16,
    /// Reserved: the public server also answers TLS on 443, but FreeDV GUI
    /// speaks plain `ws://` and so do we. Not yet implemented.
    pub tls: bool,
    /// Advertise this station as receive-only.
    pub rx_only: bool,
    /// Free-text status shown beside our callsign on the site.
    pub message: String,
    /// Send an `rx_report` for every callsign decoded from a RADE End-of-Over
    /// frame.
    pub report_rx: bool,
    /// Surface other reporter stations as spots on the panadapter and map.
    pub show_spots: bool,
}

impl Default for FreeDvReporterConfig {
    fn default() -> Self {
        FreeDvReporterConfig {
            enabled: false,
            host: "qso.freedv.org".to_string(),
            port: 80,
            tls: false,
            rx_only: false,
            message: String::new(),
            report_rx: true,
            show_spots: true,
        }
    }
}

/// Which callsign-lookup provider to use for auto-fill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LookupProvider {
    #[default]
    None,
    Qrz,
    HamQth,
}

impl LookupProvider {
    pub fn label(self) -> &'static str {
        match self {
            LookupProvider::None => "Off",
            LookupProvider::Qrz => "QRZ.com",
            LookupProvider::HamQth => "HamQTH",
        }
    }
    pub const ALL: [LookupProvider; 3] =
        [LookupProvider::None, LookupProvider::Qrz, LookupProvider::HamQth];
}

/// Username/password for a lookup or upload service (plaintext).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Credentials {
    pub user: String,
    pub password: String,
}

/// WSPRnet: what we tell it, and what we ask it.
///
/// Uploading is on by default and downloading is not, which looks backwards
/// until you notice which one is the point of the mode. Reporting what you hear
/// is what makes a WSPR receiver a member of the network rather than a private
/// curiosity; it puts nothing on the air and costs one HTTP request per spot.
/// Downloading is a poll of somebody else's server on a timer, so it waits to
/// be asked for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WsprNetConfig {
    /// Upload spots decoded here to wsprnet.org.
    pub upload: bool,
    /// Poll wsprnet.org for reports of our own callsign — who heard us.
    ///
    /// The only way a beacon learns anything about its own reach: WSPR has no
    /// acknowledgement, so a transmitting station is otherwise talking into a
    /// void.
    pub download_heard_us: bool,
    /// How often to ask, in seconds. Nothing is gained by asking faster than a
    /// slot goes by, and the server would rather we did not.
    pub download_interval_secs: u32,
    /// How far back each poll looks, in minutes. Wide enough to cover a missed
    /// poll or two without re-reading the day.
    pub download_window_min: u32,
}

impl Default for WsprNetConfig {
    fn default() -> Self {
        WsprNetConfig {
            upload: true,
            download_heard_us: false,
            download_interval_secs: 300,
            download_window_min: 30,
        }
    }
}

/// The WSJT-CB spot server.
///
/// The CB community's own spotting service (`xzgroup.net`), which the WSJT-CB
/// client can post decoded spots to. Separate from PSK Reporter, which that
/// client also supports: this one carries CB-shaped calls and the raw message
/// text, and it is where 11 m operators watch each other.
///
/// Off by default. It is a third-party service, and nothing is sent until the
/// operator asks for it — the same opt-in WSJT-CB itself ships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WsjtCbConfig {
    /// Post each decoded spot to the server.
    pub report: bool,
    /// The ingest endpoint. Only ever changed to point at a mirror or a test
    /// server.
    pub url: String,
}

impl Default for WsjtCbConfig {
    fn default() -> Self {
        WsjtCbConfig { report: false, url: "https://xzgroup.net/spots/api/ingest.php".to_string() }
    }
}

/// The whole network-feature configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
    // ── Spot feeds ──
    pub cluster: ClusterConfig,
    pub pota: FeedConfig,
    pub sota: FeedConfig,
    pub psk: PskConfig,
    pub freedv_reporter: FreeDvReporterConfig,
    /// Drop/expire spots older than this many seconds.
    pub spot_max_age_secs: u32,
    /// Show only spots that fall in the operator's current band.
    pub spot_current_band_only: bool,

    // ── Callsign lookup ──
    pub lookup_provider: LookupProvider,
    pub qrz: Credentials,
    /// HamQTH account. **Shared with the logbook upload** below — HamQTH issues
    /// one account per operator and its real-time QSO endpoint authenticates
    /// with the same username and password the XML callbook lookup does, so a
    /// second copy could only ever be a way for the two to disagree. The
    /// Uploads tab shows this one pair in both places.
    pub hamqth: Credentials,
    /// Auto-look-up on spot click / QSO start / manual call entry.
    pub auto_lookup: bool,

    // ── Uploads / confirmations ──
    pub eqsl: Credentials,
    /// QRZ Logbook API key (from the QRZ logbook settings, not the XML login).
    pub qrz_logbook_key: String,
    /// Club Log account (email in `user`, password in `password`).
    pub clublog: Credentials,
    /// Club Log application API key.
    pub clublog_api_key: String,
    /// LoTW login, used only to *download* confirmations (no auto-upload).
    pub lotw: Credentials,
    /// Automatically upload each newly logged QSO to the enabled targets.
    pub auto_upload: bool,
    pub auto_upload_eqsl: bool,
    pub auto_upload_qrz: bool,
    pub auto_upload_clublog: bool,

    // ── WSPRnet ──
    /// The WSPR network, both directions. Appended last, as the wire requires.
    pub wspr: WsprNetConfig,

    // ── Reverse Beacon Network ──
    /// The skimmer firehose, which feeds the propagation map rather than the
    /// spot list. Appended last, as the wire requires.
    pub rbn: RbnConfig,

    // ── Winlink ──
    /// Radio email. Appended last, as the wire requires.
    pub winlink: crate::WinlinkConfig,

    /// Auto-upload each new QSO to the HamQTH logbook, using the `hamqth`
    /// credentials above. Appended last, as the wire requires — it does not sit
    /// with its three siblings for the same reason [`UploadTarget::HamQth`]
    /// does not: postcard reads this struct positionally.
    ///
    /// [`UploadTarget::HamQth`]: crate::UploadTarget::HamQth
    pub auto_upload_hamqth: bool,

    // ── World Radio League ──
    /// The World Radio League developer API key, `wrl_live_…` (issue #337).
    ///
    /// A key rather than a username and password: WRL issues one per account
    /// under Integrations → Developer API, shows it once, and stores only a
    /// hash of it. Appended last, as the wire requires.
    #[serde(default)]
    pub wrl_api_key: String,
    /// Auto-upload each new QSO to World Radio League. Appended last, as the
    /// wire requires.
    #[serde(default)]
    pub auto_upload_wrl: bool,

    /// The WSJT-CB spot server. Appended last, as the wire requires.
    #[serde(default)]
    pub wsjtcb: WsjtCbConfig,

    // ── LOG11DX logbook ──
    /// The LOG11DX upload endpoint. The operator's profile page names it; the
    /// default is the one their WSJT-X bridge posts to. Appended last, as the
    /// wire requires.
    #[serde(default)]
    pub log11dx_api_url: String,
    /// The LOG11DX API token, from the same profile page. Secret — the site
    /// masks it, so it is entered once here and never shown back.
    #[serde(default)]
    pub log11dx_api_token: String,
    /// Auto-upload each new QSO to LOG11DX. Appended last, as the wire
    /// requires.
    #[serde(default)]
    pub auto_upload_log11dx: bool,

    // ── Reception identity ──
    /// The listener's own identity for reception reports — a registered SWL
    /// number, a club number, a name. When set it replaces the callsign as the
    /// *reporter* on the PSK Reporter and WSPRnet uploads and on a broadcast
    /// reception report; everything that transmits, logs or spots a QSO keeps
    /// the callsign, so an SWL number is never keyed. Appended last, as the
    /// wire requires.
    #[serde(default)]
    pub swl_id: String,

    // ── eQSL upload target ──
    /// The **QTH Nickname** an eQSL upload is filed under — which of the
    /// account's QTH profiles it goes to (issue #647). eQSL only needs it when
    /// the account owns more than one profile; otherwise it refuses the upload
    /// rather than guessing. Empty means "do not send the field", which is what
    /// a single-profile account wants. Appended last, as the wire requires —
    /// `NetworkConfig` rides `SetNetworkConfig` whole.
    #[serde(default)]
    pub eqsl_qth_nickname: String,

    // ── Reverse Beacon Network, second connection ──
    /// The second skimmer connection — by default the FT8/FT4 feed on 7001,
    /// beside [`rbn`](Self::rbn) on 7000 for CW/RTTY. Another node works as
    /// well as RBN's. Appended last, as the wire requires.
    #[serde(default = "RbnConfig::second")]
    pub rbn2: RbnConfig,

    // ── Wavelog ──
    /// The operator's **own** logbook server, with its address, its API key and
    /// — where it has a self-signed certificate — the one certificate to accept.
    /// Appended last, as the wire requires.
    #[serde(default)]
    pub wavelog: crate::WavelogConfig,

    // ── Decoded text over UDP ──
    /// Where this station's decoded text may be sent: `host:port`, one JSON line
    /// per decode, over UDP. Empty — the default — is off.
    ///
    /// The WSJT-X broadcast ([`WsjtxConfig`](crate::WsjtxConfig)) is the other
    /// direction and a logger's protocol: the FT8 family, status and logged
    /// contacts, so GridTracker and friends understand it. This is the **plain
    /// feed a script can read** — the slotted modes, WSPR and every keyboard
    /// mode's decoded lines, each with the dial it was heard on — which no
    /// logger speaks and which is the only way to get RTTY, PSK, Olivia, THOR,
    /// MFSK, DominoEX or Hell out of the program into something else.
    ///
    /// Appended last, as the wire requires — `NetworkConfig` rides
    /// `SetNetworkConfig` whole.
    #[serde(default)]
    pub decode_udp: String,

    // ── Dynamic DNS ──
    /// The provider's **update URL**, exactly as it was issued — DuckDNS,
    /// dynv6, No-IP, freedns and the rest all give one, with the token in it.
    ///
    /// **Empty is off, and empty is the default.** Nothing is started, no
    /// request is made, and a station that does not want this never meets it:
    /// the program works the same offline as it always has.
    ///
    /// It is a secret — the token is part of the address — so it is never
    /// written to a log and the screen never shows it back.
    /// Appended last, as the wire requires.
    #[serde(default)]
    pub dyndns_url: String,
    /// How often that update goes out, in minutes. Held between 1 and 1440: a
    /// provider asked more often than its own limit starts answering with
    /// errors and eventually blocks the hostname, which is worse than being ten
    /// minutes behind an address change.
    /// Appended last, as the wire requires.
    #[serde(default = "NetworkConfig::default_dyndns_minutes")]
    pub dyndns_minutes: u32,

    // ── QRZ card ──
    /// Show a QRZ.com card (photo, name, age, address, locator, QSL) when the
    /// pointer rests on a callsign in the waterfall, the decoders and the spot
    /// lists. Off by default, and only ever active when the provider is QRZ and
    /// its login is filled in ([`NetworkConfig::qrz_card_active`]).
    ///
    /// Held on the station and replayed to every screen, a browser included:
    /// the lookup runs on the station, so one QRZ session serves all of them.
    /// Appended last, as the wire requires.
    #[serde(default)]
    pub qrz_card: bool,
    /// The QRZ card is **on by default** for an operator with a QRZ login: this
    /// is the way to switch it off. It replaces [`NetworkConfig::qrz_card`],
    /// which defaulted to off and was saved as `false` by every station, so it
    /// could not tell "never chosen" from "chose off" (kept only so the wire
    /// layout does not move; nothing reads it).
    /// Appended last, as the wire requires.
    #[serde(default)]
    pub qrz_card_off: bool,
}

impl NetworkConfig {
    /// The card is shown only for an operator who gave QRZ a login: the others
    /// see nothing, make no request and have the switch greyed out.
    pub fn qrz_card_ready(&self) -> bool {
        self.lookup_provider == LookupProvider::Qrz
            && !self.qrz.user.trim().is_empty()
            && !self.qrz.password.trim().is_empty()
    }

    /// The switch is on **and** the login is there.
    pub fn qrz_card_active(&self) -> bool {
        !self.qrz_card_off && self.qrz_card_ready()
    }

    /// Five minutes: often enough that an address change is invisible to a
    /// listener, rare enough that no provider counts it as abuse.
    pub fn default_dyndns_minutes() -> u32 {
        5
    }

    /// The interval in force, clamped to what the providers tolerate.
    pub fn dyndns_interval_minutes(&self) -> u32 {
        self.dyndns_minutes.clamp(1, 1440)
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            cluster: ClusterConfig::default(),
            pota: FeedConfig::default(),
            sota: FeedConfig::default(),
            psk: PskConfig::default(),
            freedv_reporter: FreeDvReporterConfig::default(),
            spot_max_age_secs: 900,
            spot_current_band_only: false,
            lookup_provider: LookupProvider::None,
            qrz: Credentials::default(),
            hamqth: Credentials::default(),
            auto_lookup: false,
            eqsl: Credentials::default(),
            qrz_logbook_key: String::new(),
            clublog: Credentials::default(),
            clublog_api_key: String::new(),
            lotw: Credentials::default(),
            auto_upload: false,
            auto_upload_eqsl: false,
            auto_upload_qrz: false,
            auto_upload_clublog: false,
            wspr: WsprNetConfig::default(),
            winlink: crate::WinlinkConfig::default(),
            rbn: RbnConfig::default(),
            auto_upload_hamqth: false,
            wrl_api_key: String::new(),
            auto_upload_wrl: false,
            wsjtcb: WsjtCbConfig::default(),
            log11dx_api_url: "https://log11dx.com/api/wsjtx/upload-qso.php".to_string(),
            log11dx_api_token: String::new(),
            auto_upload_log11dx: false,
            swl_id: String::new(),
            eqsl_qth_nickname: String::new(),
            rbn2: RbnConfig::second(),
            wavelog: crate::WavelogConfig::default(),
            decode_udp: String::new(),
            dyndns_url: String::new(),
            dyndns_minutes: NetworkConfig::default_dyndns_minutes(),
            qrz_card: false,
            qrz_card_off: false,
        }
    }
}

#[cfg(test)]
mod qrz_card_tests {
    use super::*;

    #[test]
    fn the_card_needs_qrz_and_its_login() {
        let mut c = NetworkConfig::default();
        assert!(!c.qrz_card_active(), "no provider: no card");
        c.lookup_provider = LookupProvider::Qrz;
        assert!(!c.qrz_card_active(), "no login: no card");
        c.qrz.user = "F4XXX".into();
        c.qrz.password = "secret".into();
        assert!(c.qrz_card_active(), "provider and login: on by default");
        c.qrz_card_off = true;
        assert!(!c.qrz_card_active(), "switched off: no card");
        c.qrz_card_off = false;
        c.lookup_provider = LookupProvider::HamQth;
        assert!(!c.qrz_card_active(), "another provider: no card");
    }

    #[test]
    fn the_switch_survives_the_wire_and_an_old_file() {
        let mut cfg = NetworkConfig::default();
        cfg.qrz_card_off = true;
        let bytes = postcard::to_stdvec(&cfg).expect("encode");
        let back: NetworkConfig = postcard::from_bytes(&bytes).expect("decode");
        assert!(back.qrz_card_off);
        // A file written before the switch existed, or by the first version of
        // the card (`qrz_card: false` for everyone), reads as "on".
        let mut v = serde_json::to_value(NetworkConfig::default()).unwrap();
        v.as_object_mut().unwrap().remove("qrz_card_off");
        let old: NetworkConfig = serde_json::from_value(v).unwrap();
        assert!(!old.qrz_card_off);
        assert!(!old.qrz_card, "the old field is ignored");
    }

    #[test]
    fn a_card_with_a_photo_survives_the_wire() {
        let mut i = crate::CallsignInfo::for_call("f4abc");
        i.name = Some("Jean Dupont".into());
        i.age = Some(58);
        i.addr1 = Some("12 rue des Lilas".into());
        i.qsl_lotw = true;
        i.photo = Some(vec![0xFF, 0xD8, 0xFF, 0xD9]);
        let bytes = postcard::to_stdvec(&i).expect("encode");
        let back: crate::CallsignInfo = postcard::from_bytes(&bytes).expect("decode");
        assert_eq!(back, i);
        assert!(!back.is_empty());
    }
}

#[cfg(test)]
mod decode_udp_tests {
    use super::*;

    /// The feed's address is a field on `NetworkConfig`, which rides
    /// `SetNetworkConfig` **whole** — so it is on the wire, and postcard reads
    /// the struct positionally. A value set here has to come back, and an
    /// older `net.json` without the field has to load as off.
    #[test]
    fn the_decode_feed_address_survives_the_wire() {
        let mut cfg = NetworkConfig::default();
        cfg.decode_udp = "192.0.2.5:9999".into();
        let bytes = postcard::to_stdvec(&cfg).expect("encode");
        let back: NetworkConfig = postcard::from_bytes(&bytes).expect("decode");
        assert_eq!(back.decode_udp, "192.0.2.5:9999");
        assert_eq!(back, cfg);

        // The default is off, and a config written before the field existed
        // loads with it off rather than refusing the file.
        assert!(NetworkConfig::default().decode_udp.is_empty());
        let mut v = serde_json::to_value(NetworkConfig::default()).unwrap();
        v.as_object_mut().unwrap().remove("decode_udp");
        let old: NetworkConfig = serde_json::from_value(v).unwrap();
        assert!(old.decode_udp.is_empty());
    }
}

#[cfg(test)]
mod dyndns_tests {
    use super::*;

    /// The update URL and its interval ride `SetNetworkConfig` whole, so they
    /// are on the wire and postcard reads positionally: a value set here comes
    /// back, an older `net.json` without them loads as **off**, and the
    /// interval is clamped to what the providers tolerate rather than trusted.
    #[test]
    fn the_dynamic_dns_settings_survive_the_wire_and_default_to_off() {
        let mut cfg = NetworkConfig::default();
        cfg.dyndns_url = "https://www.duckdns.org/update?domains=x&token=y".into();
        cfg.dyndns_minutes = 15;
        let bytes = postcard::to_stdvec(&cfg).expect("encode");
        let back: NetworkConfig = postcard::from_bytes(&bytes).expect("decode");
        assert_eq!(back.dyndns_url, cfg.dyndns_url);
        assert_eq!(back.dyndns_minutes, 15);
        assert_eq!(back, cfg);

        // Off by default, and a file written before the fields existed loads
        // with them off.
        assert!(NetworkConfig::default().dyndns_url.is_empty());
        let mut v = serde_json::to_value(NetworkConfig::default()).unwrap();
        v.as_object_mut().unwrap().remove("dyndns_url");
        v.as_object_mut().unwrap().remove("dyndns_minutes");
        let old: NetworkConfig = serde_json::from_value(v).unwrap();
        assert!(old.dyndns_url.is_empty());
        assert_eq!(old.dyndns_minutes, NetworkConfig::default_dyndns_minutes());
    }

    /// An interval nobody can keep is not sent: zero would be a request every
    /// frame, a million a hostname that never updates.
    #[test]
    fn the_interval_is_held_to_what_a_provider_tolerates() {
        let mut cfg = NetworkConfig::default();
        cfg.dyndns_minutes = 0;
        assert_eq!(cfg.dyndns_interval_minutes(), 1);
        cfg.dyndns_minutes = 100_000;
        assert_eq!(cfg.dyndns_interval_minutes(), 1440);
        cfg.dyndns_minutes = 30;
        assert_eq!(cfg.dyndns_interval_minutes(), 30);
    }
}

#[cfg(test)]
mod rbn_tests {
    use super::*;

    /// A `net.json` written before the second connection existed has no `rbn2`;
    /// it must load with the second connection off on 7001, not on 7000 like the
    /// first, and the first must keep what the operator had set.
    #[test]
    fn an_older_config_gets_the_second_connection_off_on_7001() {
        let mut v = serde_json::to_value(NetworkConfig::default()).unwrap();
        v.as_object_mut().unwrap().remove("rbn2");
        v["rbn"]["port"] = 7001.into();
        let cfg: NetworkConfig = serde_json::from_value(v).unwrap();
        assert!(!cfg.rbn2.enabled);
        assert_eq!(cfg.rbn2.port, 7001);
        assert_eq!(cfg.rbn2.host, "telnet.reversebeacon.net");
        assert_eq!(cfg.rbn.port, 7001, "the first connection lost what it had");
    }

    #[test]
    fn two_connections_on_one_host_and_port_are_the_same_feed() {
        let a = RbnConfig::default();
        let mut b = RbnConfig::second();
        assert!(!a.same_feed(&b), "7000 and 7001 are different feeds");
        b.port = 7000;
        b.host = " TELNET.reversebeacon.net ".into();
        assert!(a.same_feed(&b));
    }
}
