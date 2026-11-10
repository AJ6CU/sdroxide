//! The "do not ask me again" answers are kept **on the server**, per login and
//! per browser: once answered, the login is not asked again on that browser —
//! the bindings are not even sent — and on a new browser it is asked once more.
//!
//! Its own test binary (so its own process) because it points the config store
//! at a directory of its own through the environment.

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use sdroxide_proto::{AudioCaps, ClientMsg, PROTO_VERSION, ServerMsg, decode, encode};
use sdroxide_radio::{AudioParams, EngineConfig, MicParams, SigGenSource, start_engine};
use sdroxide_server::{AccessFn, RadioParams, ServerParams, serve};
use sdroxide_types::{ClientAcks, DeviceCaps, RemoteAccess};

const PORT: u16 = 39490;
const CHROME: &str = "Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) \
    Chrome/141.0.0.0 Mobile Safari/537.36";
const CHROME_UPDATED: &str = "Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) \
    Chrome/142.0.1.2 Mobile Safari/537.36";
const FIREFOX: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:143.0) Gecko/20100101 Firefox/143.0";

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

type Ws =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

/// Connect as `user` from a browser that says `ua`, and sign in.
async fn sign_in(ua: &str, user: &str) -> Ws {
    let mut req = format!("ws://127.0.0.1:{PORT}/ws").into_client_request().unwrap();
    req.headers_mut().insert("user-agent", ua.parse().unwrap());
    let (mut ws, _) = tokio_tungstenite::connect_async(req).await.expect("connect");
    send(&mut ws, &hello()).await;
    assert_eq!(recv_msg(&mut ws).await, ServerMsg::AuthRequired);
    send(&mut ws, &ClientMsg::Auth { username: user.into(), password: "hunter2".into() }).await;
    assert!(matches!(recv_msg(&mut ws).await, ServerMsg::HelloAck { .. }), "signed in");
    ws
}

/// Everything the server sends in the next `ms`, for asserting on what was
/// *not* sent as well as what was.
async fn collect(ws: &mut Ws, ms: u64) -> Vec<ServerMsg> {
    let mut out = Vec::new();
    let until = tokio::time::Instant::now() + Duration::from_millis(ms);
    while let Ok(Some(Ok(m))) = tokio::time::timeout_at(until, ws.next()).await {
        if let Message::Binary(b) = m {
            out.push(decode::<ServerMsg>(&b).expect("decode"));
        }
    }
    out
}

fn acks_in(msgs: &[ServerMsg]) -> Option<ClientAcks> {
    msgs.iter().find_map(|m| match m {
        ServerMsg::ClientAcks(a) => Some(a.clone()),
        _ => None,
    })
}

fn bindings_in(msgs: &[ServerMsg]) -> bool {
    msgs.iter().any(|m| matches!(m, ServerMsg::ClientBindings(_)))
}

async fn close(mut ws: Ws) {
    let _ = ws.close(None).await;
    // The station serves one client at a time; let the slot free up.
    tokio::time::sleep(Duration::from_millis(300)).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn an_answer_is_kept_per_login_and_browser_on_the_server() {
    let dir = std::env::temp_dir().join("sdroxide-test-client-acks");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("config dir");
    // SAFETY: the only test in this binary, set before the server starts.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    let user = "f6kim";
    spawn_server(
        PORT,
        Some(Box::new(|| RemoteAccess { username: user.into(), password: "hunter2".into() })),
    )
    .await;

    // The profile carries bindings (stored from some other device), and this
    // Chrome has never answered: it is told so, and offered them.
    {
        let mut ws = sign_in(CHROME, user).await;
        send(
            &mut ws,
            &ClientMsg::SetClientBindings { profile: None, bindings: Default::default() },
        )
        .await;
        let first = collect(&mut ws, 1500).await;
        assert_eq!(acks_in(&first), Some(ClientAcks::default()), "nothing answered yet");
        // "Keep mine", and dismiss the receive-only banner on radio 0.
        send(
            &mut ws,
            &ClientMsg::SetClientAcks(ClientAcks {
                bindings: Some(false),
                rx_only_dismissed: vec![0],
                ..Default::default()
            }),
        )
        .await;
        let echo = collect(&mut ws, 1500).await;
        assert_eq!(acks_in(&echo).and_then(|a| a.bindings), Some(false), "stored and echoed");
        close(ws).await;
    }

    // Next session, same login, same browser (after an update): the answers
    // are there, and the bindings are **not sent**, so the question cannot
    // come back however the browser kept its storage.
    {
        let mut ws = sign_in(CHROME_UPDATED, user).await;
        let msgs = collect(&mut ws, 2000).await;
        let acks = acks_in(&msgs).expect("the answers are sent on connect");
        assert_eq!(acks.bindings, Some(false));
        assert_eq!(acks.rx_only_dismissed, vec![0]);
        assert!(!bindings_in(&msgs), "a profile that said keep mine is not offered them again");
        close(ws).await;
    }

    // Same login, another browser: asked again, as the operator wants.
    {
        let mut ws = sign_in(FIREFOX, user).await;
        let msgs = collect(&mut ws, 2000).await;
        assert_eq!(acks_in(&msgs), Some(ClientAcks::default()), "a new browser answers again");
        assert!(bindings_in(&msgs), "and is offered the bindings");
        close(ws).await;
    }
}
