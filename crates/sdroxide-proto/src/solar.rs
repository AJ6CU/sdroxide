//! Wire protocol for the browser's solar-system view (`/solar-ws`).
//!
//! Deliberately separate from [`ClientMsg`](crate::ClientMsg) /
//! [`ServerMsg`](crate::ServerMsg) and separately versioned. Two reasons:
//!
//! * The radio protocol's single-client rule exists so that exactly one client
//!   owns PTT. A map is a *viewer* — several may watch at once, and none of
//!   them may key a transmitter. Giving viewers their own message set is what
//!   makes that structural rather than a rule someone has to remember.
//! * Adding the map's traffic to [`PROTO_VERSION`](crate::PROTO_VERSION) would
//!   force every native remote client to upgrade for a feature it does not use.
//!
//! Framing is shared: [`encode`](crate::encode) / [`decode`](crate::decode)
//! work on any serialisable type, so these ride the same
//! `[VERSION_BYTE][postcard]` envelope.
//!
//! **What is not here:** decoded pixels and SGP4 constants. The SDO image
//! travels as the JPEG that was fetched, and satellites as the element set that
//! was fetched, both of which are an order of magnitude smaller than the
//! decoded product and neither of which is serialisable at all. The receiver
//! calls `sdroxide_solar::imagery::decode` and
//! `sdroxide_solar::satellites::parse_tles` to rebuild them, which is the same
//! code the native feed runs.

use serde::{Deserialize, Serialize};

use sdroxide_solar::{
    ActiveRegion, AuroraOval, CmeEvent, FlareEvent, HemisphericPower, KpPoint, SourceStatus,
    SpaceWeather,
};
use sdroxide_types::Decode;

/// Bump on any incompatible change to the two enums below. Independent of
/// [`crate::PROTO_VERSION`]: the two protocols share a transport and nothing else.
///
/// v3: `SolarServerMsg::SatFreqs` — the operator's satellite frequency
/// overrides. Appended, so no existing postcard discriminant moves, but an
/// older viewer cannot decode it and the handshake has to reject it.
/// v4: sign-in — `SolarClientMsg::Auth` plus `SolarServerMsg::AuthRequired` /
/// `AuthRejected`, appended for the same reason and rejected for the same one.
/// A viewer controls nothing, but it is shown the operator's QTH, everything
/// the station is decoding and every satellite frequency they have corrected —
/// so a server that asks the control client for a password asks this one too,
/// rather than leaving the same station's traffic on an open port.
/// v5: the propagation heat — `SolarServerMsg::Propagation`, appended for the
/// same reason and rejected for the same one. Only the *live* band planes
/// travel (typically two to four), and at most every thirty seconds: the whole
/// fourteen would be two megabytes and thirteen of them are usually empty.
/// Unlike the awards layer this is deliberately *not* left blank in the browser
/// — it is live data about the station's own conditions, which is exactly what
/// this relay exists to carry.
/// v6: band conditions. Two decode-breaking changes at once, and one bump
/// covers both. [`SpaceWeather`] gained a `band_conditions` field, which moves
/// the postcard layout of every [`SolarServerMsg::Weather`]; and
/// `sdroxide_solar::Source::ALL` grew by one, which changes the length and the
/// indexing of [`SolarServerMsg::Status`]. An older viewer would mis-decode the
/// first and mis-attribute every age in the second, so it is rejected at the
/// handshake rather than left to draw a plausible wrong picture.
pub const SOLAR_PROTO_VERSION: u16 = 6;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SolarClientMsg {
    Hello {
        proto: u16,
    },
    /// SDO channel index, as [`sdroxide_solar::SdoChannel::to_u8`].
    SetChannel(u8),
    /// SDO image edge length in pixels.
    SetResolution(u16),
    /// The overlay's ↻ button: make every source due again.
    RefreshAll,
    Ping,
    /// Answer to [`SolarServerMsg::AuthRequired`]. Appended last, like every
    /// addition to a postcard enum.
    Auth {
        username: String,
        password: String,
    },
}

/// One update from the server's feed.
///
/// Split by product rather than sent as one snapshot struct so that a refresh
/// of the X-ray class does not re-send a megabyte of solar disk. On connect the
/// server sends one of each it has, cheapest first, so the globe and the panels
/// are up before the image lands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SolarServerMsg {
    HelloAck {
        proto: u16,
    },
    Error(String),
    Pong,
    /// Per-source freshness, in [`sdroxide_solar::Source::ALL`] order. Sent
    /// whenever any source's status changes, so "offline" and "3 h ago" stay
    /// honest in the browser exactly as they do natively.
    Status(Vec<SourceStatus>),
    /// The propagation numbers and the aurora scalars — a few hundred bytes.
    Weather {
        weather: SpaceWeather,
        aurora_power: Option<HemisphericPower>,
        kp_forecast: Vec<KpPoint>,
    },
    /// DONKI and SWPC event text.
    Events {
        cmes: Vec<CmeEvent>,
        flares: Vec<FlareEvent>,
        regions: Vec<ActiveRegion>,
    },
    /// The SDO browse image, as fetched. ~150 kB at 1024², every ten minutes.
    Sun {
        /// [`sdroxide_solar::SdoChannel::to_u8`].
        channel: u8,
        fetched_unix: i64,
        jpeg: Vec<u8>,
    },
    /// The OVATION grid. Already a `Vec<u8>` of percentages — 65 kB, half-hourly.
    Aurora(AuroraOval),
    /// A TLE set in its original three-line form; `geo` marks QO-100's.
    Tles {
        geo: bool,
        text: String,
    },
    /// The operator's identity and QSO state, for the globe's QSO layer.
    ///
    /// `preview` has no equivalent here: a decode the operator has clicked but
    /// not answered is state of the *main* window, which this viewer is not.
    Digi {
        my_grid: String,
        dx_grid: Option<String>,
        transmitting: bool,
        /// The station being worked, for the card over the QSO arc. Separate
        /// from `dx_grid` because either can arrive without the other: a
        /// callsign whose grid we have not been told yet still names the
        /// contact, and a redirected arc has a grid and no call at all.
        #[serde(default)]
        dx_call: Option<String>,
        /// The mode's label, as [`sdroxide_types::Mode::label`] gives it.
        #[serde(default)]
        mode: String,
        /// Dial frequency, which is what the card turns into a band.
        #[serde(default)]
        freq_hz: f64,
        /// When the contact began and what has been exchanged.
        #[serde(default)]
        qso: Option<sdroxide_types::QsoLive>,
    },
    /// Fresh decodes. The viewer does its own fade bookkeeping, so this is the
    /// same list the flat map sees and the two agree on which stations are up.
    Decodes(Vec<Decode>),
    /// One channel of the cloud mosaic, as the PNG that was fetched — 280 kB
    /// every ten minutes, against roughly a megabyte for the decoded planes.
    /// The receiver runs `clouds::parse_plane` on it, which is the same code
    /// the native feed runs.
    Clouds {
        /// `true` for the longwave infrared channel, `false` for the visible.
        infrared: bool,
        /// The hour the picture is *of*, not when it was fetched.
        frame_unix: i64,
        fetched_unix: i64,
        png: Vec<u8>,
    },
    /// The operator's satellite frequency entries — the ones that override the
    /// built-in table, or add one for a satellite it never knew about.
    ///
    /// Knowing when a bird comes over is only half of working it; the other
    /// half is what to tune to, and a viewer that does not have these shows the
    /// published table where the operator has deliberately corrected it. Sent
    /// on connect and whenever the station's satellite config changes.
    ///
    /// Only the frequency table crosses: it is the one part of `SatConfig` this
    /// view reads. The element sets themselves arrive as [`SolarServerMsg::Tles`]
    /// — already fetched and parsed — and the subscriptions that produced them
    /// are the engine host's business, not a viewer's.
    SatFreqs(Vec<sdroxide_types::SatFreqs>),
    /// This server wants a username and password before it will start the feed.
    /// Sent in place of [`SolarServerMsg::HelloAck`], and answered with
    /// [`SolarClientMsg::Auth`].
    ///
    /// Nothing is sent and nothing is started until they are accepted — in
    /// particular the outbound feed, which talks to thirteen endpoints and
    /// exists precisely because it should run only when somebody is watching.
    AuthRequired,
    /// Those were not the credentials. Same contract as the radio protocol's:
    /// the socket stays open for another try, at the server's pace.
    AuthRejected(String),
    /// The propagation field: which bands are getting through where.
    ///
    /// Only the bands that have anything in them, so a station working one band
    /// sends one plane rather than fourteen. The viewer reassembles a
    /// [`sdroxide_types::PropField`] from these and draws it with the same code
    /// the native window does.
    Propagation {
        halflife_s: f64,
        /// `(band index into `Band::ALL`, plane)`.
        planes: Vec<(u8, sdroxide_types::BandPlane)>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decode, encode};

    /// Every variant has to survive the round trip, or the map shows something
    /// the server did not say.
    #[test]
    fn every_message_round_trips() {
        let region = ActiveRegion {
            number: 4321,
            observed_unix: 1_784_937_600,
            lat_deg: -12.0,
            lon_west_deg: 33.0,
            carrington_deg: 210.0,
            area: Some(210),
            num_spots: Some(9),
            spot_class: Some("Eki".into()),
            mag_class: Some("BG".into()),
            c_prob: 40.0,
            m_prob: 15.0,
            x_prob: 1.0,
        };
        // One band's field, as the relay carries it. Built through the public
        // API rather than by hand so the test breaks if the layout changes
        // without the version being bumped.
        fn prop_plane() -> sdroxide_types::BandPlane {
            let mut f = sdroxide_types::PropField::default();
            let o = sdroxide_types::PropObservation::new(
                (51.5, -0.1),
                (40.7, -74.0),
                14_097_100.0,
                1_784_937_600,
                sdroxide_types::PropSource::Wspr,
                Some(9.0),
            )
            .expect("20 m is an amateur band");
            f.deposit(&o, 1_784_937_600, sdroxide_types::DEFAULT_HM_KM);
            f.plane(sdroxide_types::Band::M20).expect("just deposited").clone()
        }

        let oval = AuroraOval {
            observed_unix: 1_784_937_600,
            forecast_unix: 1_784_940_000,
            grid: vec![0, 7, 40, 99],
        };
        let server = [
            SolarServerMsg::HelloAck { proto: SOLAR_PROTO_VERSION },
            SolarServerMsg::Error("no feed".into()),
            SolarServerMsg::Pong,
            SolarServerMsg::Status(vec![
                SourceStatus::default();
                sdroxide_solar::Source::ALL.len()
            ]),
            SolarServerMsg::Weather {
                weather: SpaceWeather::default(),
                aurora_power: None,
                kp_forecast: Vec::new(),
            },
            SolarServerMsg::Events { cmes: Vec::new(), flares: Vec::new(), regions: vec![region] },
            SolarServerMsg::Sun { channel: 3, fetched_unix: 1_784_937_600, jpeg: vec![0xff, 0xd8] },
            SolarServerMsg::Aurora(oval),
            SolarServerMsg::Tles { geo: true, text: "QO-100\n1 43700U\n2 43700U\n".into() },
            SolarServerMsg::Digi {
                my_grid: "JN88".into(),
                dx_grid: Some("FN42".into()),
                transmitting: true,
                dx_call: Some("AB1CD".into()),
                mode: "FT8".into(),
                freq_hz: 14_074_000.0,
                qso: Some(sdroxide_types::QsoLive {
                    started_utc: 1_784_937_600,
                    rpt_sent: Some(-12),
                    rpt_rcvd: Some(-9),
                }),
            },
            SolarServerMsg::Decodes(Vec::new()),
            SolarServerMsg::Clouds {
                infrared: true,
                frame_unix: 1_785_348_000,
                fetched_unix: 1_785_352_493,
                png: vec![0x89, b'P', b'N', b'G'],
            },
            // A transponder and a beacon on one satellite: passbands and single
            // frequencies take different paths through `Passband`.
            SolarServerMsg::SatFreqs(vec![sdroxide_types::SatFreqs::new(
                43_700,
                "QO-100",
                vec![
                    sdroxide_types::SatLink::duplex(
                        "NB transponder",
                        "SSB/CW",
                        sdroxide_types::Passband::range(2400.050, 2400.300),
                        sdroxide_types::Passband::range(10489.550, 10489.800),
                    ),
                    sdroxide_types::SatLink::down(
                        "Beacon",
                        "CW",
                        sdroxide_types::Passband::at(10489.550),
                    ),
                ],
            )]),
            SolarServerMsg::Propagation { halflife_s: 2700.0, planes: vec![(5, prop_plane())] },
            SolarServerMsg::AuthRequired,
            SolarServerMsg::AuthRejected("username or password not accepted".into()),
        ];
        for m in server {
            let bytes = encode(&m).expect("encode");
            assert_eq!(decode::<SolarServerMsg>(&bytes).expect("decode"), m);
        }

        let client = [
            SolarClientMsg::Hello { proto: SOLAR_PROTO_VERSION },
            SolarClientMsg::SetChannel(2),
            SolarClientMsg::SetResolution(2048),
            SolarClientMsg::RefreshAll,
            SolarClientMsg::Ping,
            SolarClientMsg::Auth { username: "oe1test".into(), password: "pässwörd ✓".into() },
        ];
        for m in client {
            let bytes = encode(&m).expect("encode");
            assert_eq!(decode::<SolarClientMsg>(&bytes).expect("decode"), m);
        }
    }

    /// The status array is indexed by `Source::index()`, so a viewer that sizes
    /// its own array from a stale constant would silently mis-attribute ages.
    #[test]
    fn the_status_vector_matches_the_source_list() {
        let m = SolarServerMsg::Status(vec![
            SourceStatus::default();
            sdroxide_solar::Source::ALL.len()
        ]);
        let SolarServerMsg::Status(v) = &m else { unreachable!() };
        assert_eq!(v.len(), sdroxide_solar::Source::ALL.len());
    }
}
