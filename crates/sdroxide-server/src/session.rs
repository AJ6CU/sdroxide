//! The single remote WebSocket session: Hello handshake, sign-in, codec
//! negotiation, three-lane sender, and the command/mic receive loop.

use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tracing::{info, warn};

use sdroxide_config as config;
use sdroxide_proto::{
    AudioCaps, AudioCodec, ClientBindingsReply, ClientMsg, ClientSettingsReply, PROTO_VERSION,
    ServerMsg, decode, encode,
};
use sdroxide_types::Command;

use crate::auth;
use crate::{SessionTx, Shared, Station};
/// `/ws` — the station's first radio, which is the whole of what a station
/// with one radio has. Every client that predates the roster arrives here, so
/// this address must never mean anything else.
pub async fn ws_route(
    State(station): State<Arc<Station>>,
    headers: axum::http::HeaderMap,
    upgrade: WebSocketUpgrade,
) -> Response {
    let shared = station.first();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth);
    let browser = browser_of(&headers);
    upgrade.on_upgrade(move |socket| session(socket, shared, station, signed, browser))
}

/// `/ws/<id>` — one named radio out of the station's roster. Unknown ids are
/// refused rather than rounded to a neighbour: a client that asked for the
/// Pluto and silently got the RTL-SDR would be operating the wrong radio.
pub async fn ws_route_for(
    State(station): State<Arc<Station>>,
    Path(id): Path<u32>,
    headers: axum::http::HeaderMap,
    upgrade: WebSocketUpgrade,
) -> Response {
    let signed = crate::session_cookie::from_headers(&headers, &station.auth);
    let browser = browser_of(&headers);
    match station.radio(id) {
        Some(shared) => {
            upgrade.on_upgrade(move |socket| session(socket, shared, station, signed, browser))
        }
        None => {
            let known: Vec<String> = station.list().iter().map(|r| r.id.to_string()).collect();
            (
                axum::http::StatusCode::NOT_FOUND,
                format!("this station has no radio {id}; it serves {}", known.join(", ")),
            )
                .into_response()
        }
    }
}

/// The browser and system this connection comes from, read from its
/// User-Agent — see [`config::browser_label`].
fn browser_of(headers: &axum::http::HeaderMap) -> String {
    let ua = headers.get(axum::http::header::USER_AGENT).and_then(|v| v.to_str().ok());
    config::browser_label(ua)
}

fn msg(m: &ServerMsg) -> Message {
    Message::Binary(encode(m).expect("encode").into())
}

async fn session(
    mut socket: WebSocket,
    shared: Arc<Shared>,
    station: Arc<Station>,
    signed: Option<String>,
    browser: String,
) {
    // Hello and the sign-in first, and only then the single-client slot. The
    // order matters: claiming the slot before knowing who this is would let
    // anyone who can open a socket lock the operator out of their own radio
    // without ever proving they may touch it.
    let Some((audio_caps, login)) = handshake(&mut socket, &shared, signed).await else {
        let _ = socket.close().await;
        return;
    };

    // Single-client rule: the loser gets Busy and is closed immediately.
    if shared.busy.swap(true, Ordering::SeqCst) {
        let _ = socket.send(msg(&ServerMsg::Busy)).await;
        let _ = socket.close().await;
        return;
    }
    run_session(&mut socket, &shared, &station, audio_caps, &login, &browser).await;

    // Cleanup — whatever happened, release the slot and drop the keys.
    *shared.session.lock().unwrap() = None;
    shared.busy.store(false, Ordering::SeqCst);
    release_held_controls(&shared.cmd_tx);
    info!(radio = shared.id, "remote session ended");
}

/// Everything a departing client may have been holding down, released.
///
/// Split out from the socket teardown so the list is one reviewable thing and
/// can be tested: a `Shared` is too much to build to assert four sends, and the
/// send is the whole point of this function.
///
/// The straight key is a key of its own, held apart from PTT: a client that
/// went away with the Space bar down would otherwise leave the carrier on until
/// the keyer's hold cap ran out, half a minute later.
///
/// **The paddle is not the straight key, and forgetting that keys a carrier by
/// itself.** `CwKey` is a down *edge*; the iambic keyer is driven by which
/// contacts are closed. Sending only `CwKey(false)` left `keyer_dot`/`keyer_dah`
/// set on a client that disconnected mid-press, so the keyer went on generating
/// elements with nobody holding anything — and the lost-key-up cap deliberately
/// does not apply to a keyer, because holding a paddle *should* send
/// indefinitely. That is not a truncated over; it is a carrier keyed by a client
/// that no longer exists.
fn release_held_controls(cmd: &crossbeam_channel::Sender<Command>) {
    let _ = cmd.send(Command::SetPtt(false));
    let _ = cmd.send(Command::SetTune(false));
    let _ = cmd.send(Command::CwKey(false));
    let _ = cmd.send(Command::CwContacts { dot: false, dah: false });
}

/// `Hello`, then the sign-in challenge if this server has one. `None` means the
/// socket is finished with — the caller closes it and claims nothing.
///
/// The version check comes first so a client on the wrong protocol is told
/// exactly that, rather than being asked to sign in to a server it could not
/// have talked to anyway.
async fn handshake(
    socket: &mut WebSocket,
    shared: &Arc<Shared>,
    signed: Option<String>,
) -> Option<(AudioCaps, String)> {
    // --- Hello (5 s budget) -------------------------------------------
    let hello = tokio::time::timeout(Duration::from_secs(5), socket.recv()).await;
    let audio_caps = match hello {
        Ok(Some(Ok(Message::Binary(bytes)))) => match decode::<ClientMsg>(&bytes) {
            Ok(ClientMsg::Hello { proto, audio }) if proto == PROTO_VERSION => audio,
            Ok(ClientMsg::Hello { proto, .. }) => {
                let _ = socket
                    .send(msg(&ServerMsg::Error(format!(
                        "protocol mismatch: server {PROTO_VERSION}, client {proto}"
                    ))))
                    .await;
                return None;
            }
            _ => {
                let _ = socket.send(msg(&ServerMsg::Error("expected Hello".into()))).await;
                return None;
            }
        },
        _ => return None,
    };

    // --- Sign-in ------------------------------------------------------
    // The name is captured here rather than returned by `auth::challenge`,
    // whose `bool` answers only "may this client in": it is the profile a
    // client's server-side screen settings are keyed by.
    let login = std::sync::Arc::new(std::sync::Mutex::new(signed.clone().unwrap_or_default()));
    let capture = login.clone();
    let signed_in = auth::challenge(
        socket,
        &shared.auth,
        auth::Frames {
            what: "/ws",
            required: encode(&ServerMsg::AuthRequired).expect("encode"),
            rejected: &|why| encode(&ServerMsg::AuthRejected(why.into())).expect("encode"),
            credentials: &|bytes| match decode::<ClientMsg>(bytes) {
                Ok(ClientMsg::Auth { username, password }) => {
                    *capture.lock().unwrap() = username.clone();
                    Some((username, password))
                }
                _ => None,
            },
        },
        signed.is_some(),
    )
    .await;
    let username = login.lock().unwrap().clone();
    signed_in.then(|| (audio_caps, username))
}

/// Tell the client that asked why a roster edit did not happen.
///
/// A notice rather than a [`ServerMsg::Error`]: the client reads `Error` as the
/// session being over, and a refused edit — the first radio, a name for a radio
/// that has just gone — leaves a perfectly good connection standing. Nothing is
/// sent when it worked: what happened is the new roster, which every session
/// has already been given.
fn report<T>(
    shared: &Shared,
    outcome: Result<Result<T, String>, tokio::task::JoinError>,
    what: &str,
) {
    let why = match outcome {
        Ok(Ok(_)) => return,
        Ok(Err(e)) => e,
        // The blocking task panicked or was cancelled. The client is still
        // owed an answer, or its button would simply have done nothing.
        Err(e) => format!("the station could not answer ({e})"),
    };
    warn!(radio = shared.id, "{what}: {why}");
    if let Some(s) = shared.session.lock().unwrap().as_ref() {
        let _ = s.reliable.try_send(ServerMsg::Notice(Some(format!("{what}: {why}"))));
    }
}

async fn run_session(
    socket: &mut WebSocket,
    shared: &Arc<Shared>,
    // The station this radio belongs to, for the roster it announces. Named
    // apart from the `station` below, which is the *config* of the station.
    roster: &Arc<Station>,
    audio_caps: AudioCaps,
    login: &str,
    // Which browser this is, from its User-Agent — the "do not ask me again"
    // answers are kept per login *and* per browser. See `browser_label`.
    browser: &str,
) {
    let rx_codec =
        if audio_caps.opus_decode { AudioCodec::Opus48kMono } else { AudioCodec::Pcm16_48k };
    let tx_codec =
        if audio_caps.opus_encode { AudioCodec::Opus48kMono } else { AudioCodec::Pcm16_48k };

    let (
        caps,
        state,
        memories,
        mem_folders,
        scanner,
        digi,
        voice,
        images,
        notice,
        station,
        tle_subs,
        sat_track,
        radio,
        rds,
        ism_reports,
        ism_status,
        adsb_status,
        vdl2_status,
        ais_status,
        vor_status,
        qo100_status,
        hfdl_status,
        sonde_status,
        meshtastic_status,
        drm,
        hd,
        relay,
        profiles,
    ) = {
        let latest = shared.latest.lock().unwrap();
        (
            latest.caps.clone(),
            latest.state.clone(),
            latest.memories.clone(),
            latest.mem_folders.clone(),
            latest.scanner.clone(),
            latest.digi.clone(),
            latest.voice.clone(),
            latest.images.clone(),
            latest.notice.clone(),
            latest.station.clone(),
            latest.tle_subs.clone(),
            latest.sat_track.clone(),
            latest.radio.clone(),
            latest.rds.clone(),
            latest.ism_reports.clone(),
            latest.ism_status.clone(),
            latest.adsb_status.clone(),
            latest.vdl2_status.clone(),
            latest.ais_status.clone(),
            latest.vor_status.clone(),
            latest.qo100_status.clone(),
            latest.hfdl_status.clone(),
            latest.sonde_status.clone(),
            latest.meshtastic_status.clone(),
            latest.drm.clone(),
            latest.hd.clone(),
            latest.relay.clone(),
            latest.profiles.clone(),
        )
    };
    let ack = ServerMsg::HelloAck { proto: PROTO_VERSION, caps, state, rx_codec, tx_codec };
    if socket.send(msg(&ack)).await.is_err() {
        return;
    }
    // Which radio this is, and what else the station has. Straight after the
    // acknowledgement because a client that can hold several radios opens the
    // rest from it, and it should do that before the operator has finished
    // looking at the first one.
    let _ = socket
        .send(msg(&ServerMsg::Radios {
            me: shared.id,
            radios: roster.roster(),
            editable: roster.editable(),
        }))
        .await;
    let _ = socket.send(msg(&ServerMsg::Memories(memories))).await;
    let _ = socket.send(msg(&ServerMsg::MemoryFolders(mem_folders))).await;
    let _ = socket.send(msg(&ServerMsg::Scanner(scanner))).await;
    let _ = socket.send(msg(&ServerMsg::Profiles(profiles))).await;
    // The screen settings this client asked to keep on the server, keyed by the
    // profile it signed in as (its own set, else the station default). Offered
    // unconditionally — the client decides from its own `client_save_scope`
    // whether to apply them, and reading a small JSON here costs nothing.
    //
    // Beside them, the opt-in control bindings, on the same terms: offered, and
    // applied only by a client whose operator enabled them.
    {
        let store = config::load_client_settings();
        let profile = (!login.is_empty()).then_some(login);
        // Name the profile the client *signed in as*, not the one the store
        // happened to answer from. `for_profile` falls back to the station
        // default when the named profile is still empty, and reports `None` for
        // it — so a client on a password server was told it was on `default`,
        // and its Save then wrote `default` forever, leaving `profiles` empty
        // (discussion #4). Naming the login here is what lets its next Save
        // create that profile; the settings offered are still the fallback's
        // until it does.
        //
        // When the store holds *nothing* for the login (a fresh server, no
        // default saved yet) there is no set to send, but the client still has
        // to be told which profile it is on — otherwise "Save to profile" sends
        // the `None` it would send on a passwordless server. So a signed-in
        // client is always told its name; the settings are its own until the
        // first save, and it discards them if it has a local look of its own.
        match store.for_profile(profile) {
            Some((_, settings)) => {
                let _ = socket
                    .send(msg(&ServerMsg::ClientSettings(ClientSettingsReply {
                        profile: profile.map(str::to_string),
                        settings,
                        has_stored: true,
                    })))
                    .await;
            }
            None if profile.is_some() => {
                let _ = socket
                    .send(msg(&ServerMsg::ClientSettings(ClientSettingsReply {
                        profile: profile.map(str::to_string),
                        settings: sdroxide_types::ClientScreen::default(),
                        has_stored: false,
                    })))
                    .await;
            }
            None => {}
        }
        // The "do not ask me again" answers, ahead of the bindings so the
        // client already knows its answer when they arrive. A profile that said
        // "keep mine" is not sent the bindings at all: the question cannot come
        // back if there is nothing to ask about.
        let acks = store.acks_for(login, browser);
        let declined = acks.bindings == Some(false);
        let _ = socket.send(msg(&ServerMsg::ClientAcks(acks))).await;
        if !declined && let Some((_, bindings)) = store.bindings_for_profile(profile) {
            let _ = socket
                .send(msg(&ServerMsg::ClientBindings(ClientBindingsReply {
                    profile: profile.map(str::to_string),
                    bindings,
                })))
                .await;
        }
    }
    // The operator config, which the engine announced once at startup. Without
    // this replay the client's callsign and grid come up empty and greyed out.
    if let Some(d) = digi {
        let _ = socket.send(msg(&ServerMsg::Ft8Status(d))).await;
    }
    // Likewise the voice keyer's slots, announced once at engine start.
    if let Some(v) = voice {
        let _ = socket.send(msg(&ServerMsg::VoiceStatus(v))).await;
    }
    // The station the radio is sitting on, if it is a WFM broadcast carrying
    // RDS. A condition rather than an event: the name and programme type may
    // have arrived minutes ago and will not be sent again until they change.
    if let Some(d) = rds {
        let _ = socket.send(msg(&ServerMsg::Rds(d))).await;
    }
    // The DRM broadcast being decoded, for the same reason as the RDS station
    // above: sync and a service label are conditions, not events.
    if let Some(d) = drm {
        let _ = socket.send(msg(&ServerMsg::Drm(d))).await;
    }
    // The HD Radio broadcast being decoded, for the same reason again.
    if let Some(d) = hd {
        let _ = socket.send(msg(&ServerMsg::Hd(d))).await;
    }
    // The ISM device table and where the decoder is listening. Both are slow
    // conditions — see `Latest::ism_reports`.
    if let Some(st) = ism_status {
        let _ = socket.send(msg(&ServerMsg::IsmStatus(st))).await;
    }
    if !ism_reports.is_empty() {
        let _ = socket.send(msg(&ServerMsg::IsmReports(ism_reports))).await;
    }
    // The aircraft table, for the same reason: what is overhead is a condition.
    if let Some(st) = adsb_status {
        let _ = socket.send(msg(&ServerMsg::AdsbStatus(st))).await;
    }
    // ...and the VDL2 log, for the same reason: what has been said is a record,
    // not an event that has already happened to somebody else.
    if let Some(st) = vdl2_status {
        let _ = socket.send(msg(&ServerMsg::Vdl2Status(st))).await;
    }
    // ...and the vessel table, most of all: at three minutes between reports
    // from a ship at anchor, a client that had to wait for one would sit in
    // front of a blank chart long enough to conclude the decoder was broken.
    if let Some(st) = ais_status {
        let _ = socket.send(msg(&ServerMsg::AisStatus(st))).await;
    }
    // ...and the VOR read-out, so the view is not blank until the next one.
    if let Some(st) = vor_status {
        let _ = socket.send(msg(&ServerMsg::VorStatus(st))).await;
    }
    // ...and the QO-100, HFDL and radiosonde read-outs, likewise.
    if let Some(st) = qo100_status {
        let _ = socket.send(msg(&ServerMsg::Qo100Status(st))).await;
    }
    if let Some(st) = hfdl_status {
        let _ = socket.send(msg(&ServerMsg::HfdlStatus(st))).await;
    }
    if let Some(st) = sonde_status {
        let _ = socket.send(msg(&ServerMsg::SondeStatus(st))).await;
    }
    if let Some(st) = meshtastic_status {
        let _ = socket.send(msg(&ServerMsg::MeshtasticStatus(st))).await;
    }
    // And the transmit-image presets, for the same reason. The received
    // galleries are not replayed: a panel lists its store when it opens, which
    // is both authoritative and the only view that can be paged.
    if let Some(p) = images {
        let _ = socket.send(msg(&ServerMsg::ImagePresets(p))).await;
    }
    // What the station is set up to do, announced at engine start like the
    // operator config. Without this replay the settings dialog here shows
    // defaults for every server-side tab — and applying them would write those
    // defaults over the operator's real configuration.
    if let Some(s) = station {
        let _ = socket.send(msg(&ServerMsg::StationConfig(s))).await;
        let _ = socket.send(msg(&ServerMsg::TleSubStatus(tle_subs))).await;
    }
    // And which interface this machine has open, with every backend's settings.
    // Same reason again, and the same failure without it: the Radio tab would
    // come up on defaults, and the first thing touched there would write a
    // default sample rate and an empty device selection over the operator's.
    if let Some(r) = radio {
        let _ = socket.send(msg(&ServerMsg::RadioConfig(r))).await;
    }
    // The satellite lock is a condition too: a client attaching mid-pass has
    // to see it immediately, and must not offer to start one that is running.
    if sat_track.is_some() {
        let _ = socket.send(msg(&ServerMsg::SatTrack(sat_track))).await;
    }
    // And the T/R switch, for the strongest version of the same reason: a relay
    // that is not answering is the one standing condition here that a client
    // needs to know about *before* it touches the PTT button.
    if let Some(r) = relay {
        let _ = socket.send(msg(&ServerMsg::RelayStatus(r))).await;
    }
    // A standing condition rather than an event: whoever attaches next has to
    // know the radio is refusing tunes or reconnecting, not just whoever
    // happened to be connected when it started.
    if notice.is_some() {
        let _ = socket.send(msg(&ServerMsg::Notice(notice))).await;
    }
    info!(radio = shared.id, ?rx_codec, ?tx_codec, "remote client connected");

    // --- register lanes -----------------------------------------------
    let (rel_tx, mut rel_rx) = mpsc::channel::<ServerMsg>(256);
    let (aud_tx, mut aud_rx) = mpsc::channel::<ServerMsg>(crate::AUDIO_LANE_DEPTH);
    *shared.session.lock().unwrap() = Some(SessionTx { reliable: rel_tx, audio: aud_tx, rx_codec });

    let (mut ws_tx, mut ws_rx) = futures_util::StreamExt::split(socket);

    // Sender: audio first, then reliable, then latest spectrum.
    //
    // Audio leads because it is the only lane that cannot wait: it is 50
    // small packets a second at most, so it cannot starve the others, while a
    // burst on the reliable lane (up to 256 queued messages on connect or after
    // a decode run) used to be written out ahead of it and overflowed the
    // audio lane meanwhile -- holes in the sound with nothing wrong on the wire.
    let mut spectrum_rx = shared.spectrum_rx.clone();
    let mut wide_rx = shared.wide_spectrum_rx.clone();
    let sender = async {
        let mut last_spectrum_seq = 0u32;
        let mut last_wide_seq = 0u32;
        loop {
            tokio::select! {
                biased;
                m = aud_rx.recv() => {
                    let Some(m) = m else { break };
                    if ws_tx.send(msg(&m)).await.is_err() { break; }
                }
                m = rel_rx.recv() => {
                    let Some(m) = m else { break };
                    if ws_tx.send(msg(&m)).await.is_err() { break; }
                }
                changed = wide_rx.changed() => {
                    if changed.is_err() { break; }
                    let frame = wide_rx.borrow_and_update().clone();
                    if let Some(f) = frame {
                        if f.seq != last_wide_seq {
                            last_wide_seq = f.seq;
                            if ws_tx.send(msg(&ServerMsg::WideSpectrum(f))).await.is_err() { break; }
                        }
                    }
                }
                changed = spectrum_rx.changed() => {
                    if changed.is_err() { break; }
                    let frame = spectrum_rx.borrow_and_update().clone();
                    if let Some(f) = frame {
                        if f.seq != last_spectrum_seq {
                            last_spectrum_seq = f.seq;
                            if ws_tx.send(msg(&ServerMsg::Spectrum(f))).await.is_err() { break; }
                        }
                    }
                }
            }
        }
    };

    // Receiver: commands, mic frames, pings.
    let receiver = async {
        let mut opus_dec: Option<opus::Decoder> = None;
        let mut pcm = vec![0.0f32; 5760];
        while let Some(Ok(m)) = ws_rx.next().await {
            let Message::Binary(bytes) = m else {
                if matches!(m, Message::Close(_)) {
                    break;
                }
                continue;
            };
            match decode::<ClientMsg>(&bytes) {
                Ok(ClientMsg::Command(cmd)) => {
                    let _ = shared.cmd_tx.send(cmd);
                }
                Ok(ClientMsg::MicFrame { payload, .. }) => {
                    let n = match tx_codec {
                        AudioCodec::Opus48kMono => {
                            let dec = opus_dec.get_or_insert_with(|| {
                                opus::Decoder::new(48_000, opus::Channels::Mono)
                                    .expect("opus decoder")
                            });
                            match dec.decode_float(&payload, &mut pcm, false) {
                                Ok(n) => n,
                                Err(e) => {
                                    warn!("opus decode: {e}");
                                    continue;
                                }
                            }
                        }
                        AudioCodec::Pcm16_48k => {
                            let mut n = 0;
                            for (i, c) in payload.chunks_exact(2).enumerate() {
                                if i >= pcm.len() {
                                    break;
                                }
                                pcm[i] = i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0;
                                n += 1;
                            }
                            n
                        }
                    };
                    let mut mic = shared.mic_tx.lock().unwrap();
                    for &s in &pcm[..n] {
                        if mic.push(s).is_err() {
                            break; // ring full — engine will catch up
                        }
                    }
                }
                // Not into the engine: a bus scan or a connection test blocks
                // for as long as the hardware takes, and the engine thread is
                // the one carrying the radio. Its own worker answers, in the
                // order these arrive — see `crate::probe`.
                Ok(ClientMsg::Probe(req)) => crate::probe::ask(shared, req),
                // The station's roster, not this radio's engine. Answered here
                // rather than forwarded: a command goes to *a* radio, and how
                // many radios there are belongs to the station.
                //
                // Awaited in the receive loop, so roster edits are taken one at
                // a time in the order they arrive — the same rule as probes,
                // and for a stronger reason: adding a radio starts an engine,
                // and two of them racing would both read the same `next_id`.
                Ok(ClientMsg::AddRadio { name }) => {
                    let station = roster.clone();
                    // On a blocking thread: it creates the radio's
                    // configuration scope and starts an engine, neither of
                    // which belongs on the socket task.
                    let done = tokio::task::spawn_blocking(move || station.add_radio(&name)).await;
                    report(shared, done, "adding a radio");
                }
                Ok(ClientMsg::RemoveRadio { id }) => {
                    let station = roster.clone();
                    let done = tokio::task::spawn_blocking(move || station.remove_radio(id)).await;
                    report(shared, done, "closing a radio");
                }
                Ok(ClientMsg::RenameRadio { id, name }) => {
                    let station = roster.clone();
                    let done =
                        tokio::task::spawn_blocking(move || station.rename_radio(id, &name)).await;
                    report(shared, done, "renaming a radio");
                }
                // Also the station's business, not this radio's engine: it is
                // the roster that says whether a radio has an interface at all.
                // Blocking for the same reason as the rest — it writes the
                // host's roster file.
                Ok(ClientMsg::SetRadioEnabled { id, on }) => {
                    let station = roster.clone();
                    let done =
                        tokio::task::spawn_blocking(move || station.set_radio_power(id, on)).await;
                    report(shared, done, "switching a radio");
                }
                // This client's screen settings, kept on the server against the
                // profile it signed in as. Blocking — it reads and writes a
                // JSON — and only the presentation half is stored, the rest
                // dropped by the store.
                //
                // Keyed on the **authenticated identity**, and the profile the
                // message names is deliberately ignored. A client only ever
                // knows the profile it was *offered*, which is `default`
                // whenever its own profile holds nothing yet (`for_profile`
                // falls back and reports the fallback), so saving under that
                // wrote every operator's screen into the station's shared
                // default and never created a named profile at all — which is
                // what a listener on a server with a password saw. Keying here
                // also means a client cannot write another profile's settings
                // by naming it.
                Ok(ClientMsg::SetClientSettings { profile: _, settings }) => {
                    let key = (!login.is_empty()).then(|| login.to_string());
                    let done = tokio::task::spawn_blocking(move || {
                        let mut store = config::load_client_settings();
                        store.set(key.as_deref(), settings);
                        config::save_client_settings(&store).map_err(|e| e.to_string())
                    })
                    .await;
                    let ok = matches!(done, Ok(Ok(())));
                    report(shared, done, "saving the screen settings");
                    // Echo back what is now stored, so the client knows which
                    // set it is using and that it landed.
                    if ok {
                        let store = config::load_client_settings();
                        let profile = (!login.is_empty()).then_some(login);
                        // Echo the *signed-in* name, not the fallback's `None`:
                        // the write above just landed under this login, so
                        // reporting `None` here would tell the client its save
                        // went to the shared default — the same loop the offer
                        // had (discussion #4).
                        if let Some((_, stored)) = store.for_profile(profile)
                            && let Some(s) = shared.session.lock().unwrap().as_ref()
                        {
                            let _ = s.reliable.try_send(ServerMsg::ClientSettings(
                                ClientSettingsReply {
                                    profile: profile.map(str::to_string),
                                    settings: stored,
                                    has_stored: true,
                                },
                            ));
                        }
                    }
                }
                // This client's opt-in control bindings, kept beside its screen.
                // The server stores whatever it is given, per profile: whether
                // the operator actually acknowledged the risk is a question only
                // the sending client can answer, and it is the *applying* client
                // that re-checks its own opt-in. So a binding cannot reach an
                // operator who did not ask for one.
                // Keyed on the authenticated identity for the same reason as
                // the screen above, and for the same reason the named profile
                // is ignored: on a server with a password the profile a client
                // was offered is the shared default until its own exists, so
                // bindings saved from it would land there instead.
                Ok(ClientMsg::SetClientBindings { profile: _, bindings }) => {
                    let key = (!login.is_empty()).then(|| login.to_string());
                    let done = tokio::task::spawn_blocking(move || {
                        let mut store = config::load_client_settings();
                        store.set_bindings(key.as_deref(), bindings);
                        config::save_client_settings(&store).map_err(|e| e.to_string())
                    })
                    .await;
                    let ok = matches!(done, Ok(Ok(())));
                    report(shared, done, "saving the control bindings");
                    if ok {
                        let store = config::load_client_settings();
                        let profile = (!login.is_empty()).then_some(login);
                        if let Some((_, stored)) = store.bindings_for_profile(profile)
                            && let Some(s) = shared.session.lock().unwrap().as_ref()
                        {
                            let _ = s.reliable.try_send(ServerMsg::ClientBindings(
                                ClientBindingsReply {
                                    profile: profile.map(str::to_string),
                                    bindings: stored,
                                },
                            ));
                        }
                    }
                }
                // The "do not ask me again" answers, keyed on the authenticated
                // identity *and the browser* (the operator's rule: a new browser
                // answers again). Merged rather than replaced:
                // each radio tab of one login sends its own copy, and a tab that
                // never saw an answer must not erase it.
                Ok(ClientMsg::SetClientAcks(incoming)) => {
                    let (who, what) = (login.to_string(), browser.to_string());
                    let done = tokio::task::spawn_blocking(move || {
                        let mut store = config::load_client_settings();
                        let merged = store.merge_acks(&who, &what, &incoming);
                        config::save_client_settings(&store)
                            .map(|()| merged)
                            .map_err(|e| e.to_string())
                    })
                    .await;
                    if let Ok(Ok(merged)) = &done
                        && let Some(s) = shared.session.lock().unwrap().as_ref()
                    {
                        let _ = s.reliable.try_send(ServerMsg::ClientAcks(merged.clone()));
                    }
                    report(shared, done.map(|r| r.map(|_| ())), "saving the client's answers");
                }
                Ok(ClientMsg::Ping(t)) => {
                    if let Some(s) = shared.session.lock().unwrap().as_ref() {
                        let _ = s.reliable.try_send(ServerMsg::Pong(t));
                    }
                }
                Ok(ClientMsg::Hello { .. }) => {} // ignore late Hello
                // Likewise a late `Auth`: this socket is already signed in, so
                // there is nothing to re-check, and running it through the gate
                // would let an established client lock everybody else's sign-in
                // out for three seconds at a time.
                Ok(ClientMsg::Auth { .. }) => {}
                Err(e) => warn!("bad client message: {e}"),
            }
        }
    };

    tokio::select! {
        _ = sender => {}
        _ = receiver => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A client that disconnects mid-press must not leave the transmitter keyed.
    ///
    /// The straight key and the paddle are two different things and only one of
    /// them was released: `CwKey` carries a down *edge*, while the iambic keyer
    /// is driven by which contacts are closed. A client that vanished holding a
    /// paddle therefore left the keyer generating elements with nobody holding
    /// anything — and the lost-key-up cap deliberately does not apply to a
    /// keyer, so nothing downstream would ever stop it.
    #[test]
    fn a_departing_client_releases_the_paddle_and_not_only_the_key() {
        let (tx, rx) = crossbeam_channel::unbounded();
        release_held_controls(&tx);
        let sent: Vec<Command> = rx.try_iter().collect();
        assert!(
            sent.contains(&Command::CwContacts { dot: false, dah: false }),
            "the paddle must be released, not only the key edge: got {sent:?}"
        );
        assert!(sent.contains(&Command::CwKey(false)), "and the straight key");
        assert!(sent.contains(&Command::SetPtt(false)), "and the PTT");
    }
}
