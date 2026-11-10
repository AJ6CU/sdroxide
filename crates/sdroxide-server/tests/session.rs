//! End-to-end server test: real engine (signal generator) + real WebSocket
//! client. Covers the handshake, sign-in, state echo, spectrum/audio streaming,
//! and the single-client Busy rule.

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

use sdroxide_proto::{AudioCaps, ClientMsg, PROTO_VERSION, ServerMsg, decode, encode};
use sdroxide_radio::{AudioParams, EngineConfig, MicParams, SigGenSource, start_engine};
use sdroxide_server::{AccessFn, RadioParams, ServerParams, serve};
use sdroxide_types::{Command, DeviceCaps, RemoteAccess, Vfo};

const PORT: u16 = 39471;
/// The sign-in test's own engine, so the two tests cannot disturb each other's
/// turnstile — it is deliberately server-wide state.
const AUTH_PORT: u16 = 39472;

async fn recv_msg(
    ws: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin),
) -> ServerMsg {
    loop {
        let m = tokio::time::timeout(Duration::from_secs(15), ws.next())
            .await
            .expect("timeout waiting for server message")
            .expect("stream ended")
            .expect("ws error");
        if let Message::Binary(bytes) = m {
            return decode::<ServerMsg>(&bytes).expect("decode");
        }
    }
}

/// An engine on a signal generator, served on `port`.
async fn spawn_server(port: u16, access: Option<AccessFn>) {
    let (audio_producer, audio_consumer) = sdroxide_radio::rtrb::RingBuffer::<f32>::new(96_000);
    let (mic_producer, mic_consumer) = sdroxide_radio::rtrb::RingBuffer::<f32>::new(48_000);
    let source = SigGenSource::demo(1_536_000.0, 14_200_000.0);
    let caps = DeviceCaps {
        driver: "siggen".into(),
        label: "Test signal generator".into(),
        rx_channels: 1,
        freq_ranges_rx: vec![(0.0, 6e9)],
        ..DeviceCaps::default()
    };
    let handles = start_engine(
        Box::new(source),
        caps,
        EngineConfig {
            audio: Some(AudioParams { producer: audio_producer, out_rate: 48_000.0 }),
            mic: Some(MicParams { consumer: mic_consumer, rate: 48_000.0 }),
            ..Default::default()
        },
    );

    tokio::spawn(serve(ServerParams {
        radios: vec![RadioParams {
            id: 0,
            name: String::new(),
            cmd_tx: handles.cmd_tx,
            event_rx: handles.event_rx,
            spectrum_out: handles.spectrum_out,
            wide_spectrum_out: handles.wide_spectrum_out,
            audio_rx: audio_consumer,
            mic_tx: mic_producer,
        }],
        bind: "127.0.0.1".into(),
        port,
        web_root: None,
        access,
        // These tests are about the session, not about this machine's buses:
        // a prober that answers from a table keeps them off whatever hardware
        // the test runner happens to have, while still exercising the lane.
        probe: None,
        add_radio: None,
        remove_radio: None,
        rename_radio: None,
        radio_power: None,
        aircraft_online: Some(false),
    }));
    tokio::time::sleep(Duration::from_millis(400)).await;
}

fn hello() -> ClientMsg {
    ClientMsg::Hello {
        proto: PROTO_VERSION,
        audio: AudioCaps { opus_decode: false, opus_encode: false },
    }
}

async fn send(
    ws: &mut (impl SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin),
    m: &ClientMsg,
) {
    ws.send(Message::Binary(encode(m).unwrap().into())).await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn full_session_flow() {
    // No credentials configured: the server is open, exactly as it was before
    // sign-in existed.
    spawn_server(PORT, None).await;

    let (mut ws, _) = tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{PORT}/ws"))
        .await
        .expect("connect");

    // Hello → HelloAck with the device caps, PCM16 negotiated.
    send(&mut ws, &hello()).await;
    match recv_msg(&mut ws).await {
        ServerMsg::HelloAck { proto, caps, state, rx_codec, .. } => {
            assert_eq!(proto, PROTO_VERSION);
            assert_eq!(caps.label, "Test signal generator");
            assert!(state.sample_rate > 0.0);
            assert_eq!(rx_codec, sdroxide_proto::AudioCodec::Pcm16_48k);
        }
        other => panic!("expected HelloAck, got {other:?}"),
    }

    // Streams flow: within a few seconds we must see spectrum AND audio. The
    // station config has to arrive too — the engine announces it once, long
    // before anybody connects, so it only reaches a client if the server kept
    // it and replayed it. Without that the settings dialog here shows every
    // server-side tab as unconfigured.
    let (mut got_spectrum, mut got_audio, mut got_station) = (false, false, false);
    while !(got_spectrum && got_audio && got_station) {
        match recv_msg(&mut ws).await {
            ServerMsg::Spectrum(f) => {
                assert!(!f.bins.is_empty());
                assert!(f.span_hz > 0.0);
                got_spectrum = true;
            }
            ServerMsg::RxAudio { payload, .. } => {
                assert_eq!(payload.len(), 1920, "20 ms of PCM16");
                got_audio = true;
            }
            ServerMsg::StationConfig(_) => got_station = true,
            _ => {}
        }
    }

    // Command → state echo.
    let cmd = ClientMsg::Command(Command::SetVfo { vfo: Vfo::A, hz: 14_100_000.0 });
    send(&mut ws, &cmd).await;
    loop {
        if let ServerMsg::State(s) = recv_msg(&mut ws).await {
            if (s.vfo_a_hz - 14_100_000.0).abs() < 1.0 {
                break;
            }
        }
    }

    // Second client gets Busy.
    let (mut ws2, _) = tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{PORT}/ws"))
        .await
        .expect("connect 2");
    send(&mut ws2, &hello()).await;
    match recv_msg(&mut ws2).await {
        ServerMsg::Busy => {}
        other => panic!("expected Busy, got {other:?}"),
    }

    // First client drops; after cleanup a new client can connect again.
    drop(ws);
    drop(ws2);
    tokio::time::sleep(Duration::from_millis(500)).await;
    let (mut ws3, _) = tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{PORT}/ws"))
        .await
        .expect("reconnect");
    send(&mut ws3, &hello()).await;
    match recv_msg(&mut ws3).await {
        ServerMsg::HelloAck { .. } => {}
        other => panic!("expected HelloAck on reconnect, got {other:?}"),
    }
}

/// The sign-in, over a real socket: what a wrong password costs, and the two
/// things a client that has not given one must not be able to do — take the
/// single-client slot, or reach the radio.
#[tokio::test(flavor = "multi_thread")]
async fn a_server_with_credentials_signs_clients_in() {
    spawn_server(
        AUTH_PORT,
        Some(Box::new(|| RemoteAccess { username: "oe1test".into(), password: "hunter2".into() })),
    )
    .await;
    let url = format!("ws://127.0.0.1:{AUTH_PORT}/ws");

    // Hello is answered with the challenge, not with the radio.
    let (mut ws, _) = tokio_tungstenite::connect_async(&url).await.expect("connect");
    send(&mut ws, &hello()).await;
    assert_eq!(recv_msg(&mut ws).await, ServerMsg::AuthRequired);

    // A command sent instead of credentials must not reach the engine. Nothing
    // comes back for it — the socket simply stays in the challenge — so the
    // proof is that the *next* thing the server says is still a rejection.
    send(&mut ws, &ClientMsg::Command(Command::SetPtt(true))).await;
    send(&mut ws, &ClientMsg::Auth { username: "oe1test".into(), password: "wrong".into() }).await;
    match recv_msg(&mut ws).await {
        ServerMsg::AuthRejected(why) => {
            assert!(!why.is_empty());
            // Which half was wrong is exactly what an attacker with a list of
            // callsigns wants to be told, so it must not be in there.
            let why = why.to_lowercase();
            assert!(!why.contains("unknown user"), "the message must not name the wrong half");
        }
        other => panic!("expected AuthRejected, got {other:?}"),
    }

    // A client that has not signed in holds nothing: a second one is still
    // offered the challenge rather than being turned away as Busy. This is the
    // part that stops a stranger locking the operator out of their own radio
    // simply by opening a socket.
    let (mut ws2, _) = tokio_tungstenite::connect_async(&url).await.expect("connect 2");
    send(&mut ws2, &hello()).await;
    assert_eq!(recv_msg(&mut ws2).await, ServerMsg::AuthRequired);

    // The right credentials get in — but not before the wrong answer above has
    // finished shutting the door, which is what makes guessing impractical.
    let started = std::time::Instant::now();
    send(&mut ws2, &ClientMsg::Auth { username: "oe1test".into(), password: "hunter2".into() })
        .await;
    match recv_msg(&mut ws2).await {
        ServerMsg::HelloAck { proto, caps, .. } => {
            assert_eq!(proto, PROTO_VERSION);
            assert_eq!(caps.label, "Test signal generator");
        }
        other => panic!("expected HelloAck after signing in, got {other:?}"),
    }
    assert!(
        started.elapsed() >= Duration::from_millis(2_500),
        "a correct sign-in was judged {:?} after a wrong one, without serving out the lockout",
        started.elapsed()
    );

    // ...and now that somebody is in, the slot really is held: the one still
    // sitting on the challenge signs in and is told the radio is taken.
    send(&mut ws, &ClientMsg::Auth { username: "oe1test".into(), password: "hunter2".into() })
        .await;
    match recv_msg(&mut ws).await {
        ServerMsg::Busy => {}
        other => panic!("expected Busy for the second signed-in client, got {other:?}"),
    }
}

/// A signed-in client's screen settings land in **its own profile**, not the
/// station's shared default.
///
/// The bug this pins was reported on a server **with** a password (discussion
/// #4): `clientsettings.json` had a populated `default` block and an empty
/// `profiles` map, so the write worked but always landed in the shared bucket.
/// The cause is that a client only knows the profile it was *offered*, and
/// `for_profile` falls back to `default` — reporting `profile: None` — whenever
/// that profile is still empty. Saving under what you were offered therefore
/// wrote the default forever and never created a profile.
///
/// It is invisible without a password, because there `default` is the right
/// bucket. Hence credentials here, and a client that deliberately sends the
/// `None` it really would have sent.
#[tokio::test(flavor = "multi_thread")]
async fn a_signed_in_clients_screen_lands_in_its_own_profile() {
    // Its own config directory: this writes `clientsettings.json`, and a test
    // has no business touching the operator's real profile.
    let dir = std::env::temp_dir().join("sdroxide-test-clientsettings");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("config dir");
    // SAFETY: set before any other test in this binary reads it; no other test
    // here touches the config store.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    const PORT: u16 = 39474;
    let user = "f6kim";
    spawn_server(
        PORT,
        Some(Box::new(|| RemoteAccess { username: user.into(), password: "hunter2".into() })),
    )
    .await;
    let url = format!("ws://127.0.0.1:{PORT}/ws");

    let mut screen = sdroxide_types::ClientScreen::from_settings(&Default::default());
    screen.theme = sdroxide_types::UiTheme::Rainbow;

    /// Read until the wanted variant. Bounded, because the server streams state
    /// continuously and an unbounded loop never reaches `recv_msg`'s timeout.
    async fn wait_for<T>(
        ws: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin),
        mut f: impl FnMut(&ServerMsg) -> Option<T>,
    ) -> T {
        for _ in 0..500 {
            if let Some(v) = f(&recv_msg(ws).await) {
                return v;
            }
        }
        panic!("the server never sent the expected message");
    }

    // First session: a fresh store, so nothing is saved yet. The client must
    // still be told its own profile name (`has_stored: false`, no look to
    // adopt) so that its save targets the profile and not the shared default.
    {
        let (mut ws, _) = tokio_tungstenite::connect_async(&url).await.expect("connect");
        send(&mut ws, &hello()).await;
        assert_eq!(recv_msg(&mut ws).await, ServerMsg::AuthRequired);
        send(&mut ws, &ClientMsg::Auth { username: user.into(), password: "hunter2".into() }).await;
        assert!(matches!(recv_msg(&mut ws).await, ServerMsg::HelloAck { .. }), "signed in");
        let offer = wait_for(&mut ws, |m| match m {
            ServerMsg::ClientSettings(r) => Some(r.clone()),
            _ => None,
        })
        .await;
        assert_eq!(
            offer.profile.as_deref(),
            Some(user),
            "a signed-in client is told its own profile even before it has saved"
        );
        assert!(!offer.has_stored, "and knows the look is not its own yet");
    }

    // Save, sending the `None` a real client sends.
    {
        let (mut ws, _) = tokio_tungstenite::connect_async(&url).await.expect("connect");
        send(&mut ws, &hello()).await;
        assert_eq!(recv_msg(&mut ws).await, ServerMsg::AuthRequired);
        send(&mut ws, &ClientMsg::Auth { username: user.into(), password: "hunter2".into() }).await;
        assert!(matches!(recv_msg(&mut ws).await, ServerMsg::HelloAck { .. }), "signed in");

        send(&mut ws, &ClientMsg::SetClientSettings { profile: None, settings: screen }).await;
        // Wait for the *stored* echo, not the name-only offer the connect sent
        // just before — the offer has `has_stored: false`.
        let reply = wait_for(&mut ws, |m| match m {
            ServerMsg::ClientSettings(r) if r.has_stored => Some(r.clone()),
            _ => None,
        })
        .await;
        // **The assertion that fails before the fix**: it came back as `None`,
        // because the screen had been written into the shared default.
        assert_eq!(
            reply.profile.as_deref(),
            Some(user),
            "the screen is stored against the profile it signed in as"
        );
        assert_eq!(reply.settings.theme, screen.theme);
    }

    // On disk, under the profile and not only the default.
    let saved = std::fs::read_to_string(dir.join("clientsettings.json")).expect("clientsettings.json written");
    assert!(saved.contains(user), "the profile is in the store: {saved}");

    // A new session is offered that profile's own set, unasked.
    {
        let (mut ws, _) = tokio_tungstenite::connect_async(&url).await.expect("reconnect");
        send(&mut ws, &hello()).await;
        assert_eq!(recv_msg(&mut ws).await, ServerMsg::AuthRequired);
        send(&mut ws, &ClientMsg::Auth { username: user.into(), password: "hunter2".into() }).await;
        assert!(matches!(recv_msg(&mut ws).await, ServerMsg::HelloAck { .. }), "signed in");
        let got = wait_for(&mut ws, |m| match m {
            ServerMsg::ClientSettings(r) => Some(r.clone()),
            _ => None,
        })
        .await;
        assert_eq!(got.profile.as_deref(), Some(user));
        assert_eq!(got.settings.theme, screen.theme, "and it is the set that was saved");
    }

    let _ = std::fs::remove_dir_all(&dir);
}
