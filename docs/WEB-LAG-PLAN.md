# Web client: slowdowns on a healthy network — plan

Report: in a Chrome web session (app window) the audio and waterfall stutter
as if packets were being lost, while the network itself is fine. Native app and
Linux **not yet checked** (see the checklist below).

## What the code showed

| # | Suspect | Where | Effect on a good network |
|---|---------|-------|--------------------------|
| 1 | No `TCP_NODELAY` on accepted sockets (Nagle + browser delayed ACK) | `sdroxide-server/src/lib.rs` (`serve`) | Small WS messages (20 ms audio, meters, spectrum rows) wait up to 40 ms (200 ms on Windows) then leave in a burst: looks like loss |
| 2 | Audio lane only 8 packets deep (160 ms), drops were silent | `lib.rs` pump, `session.rs` | One big spectrum frame being written to a slow link overflows it; holes in audio, nothing in the logs |
| 3 | Sender served the reliable lane before audio | `session.rs` sender `select!` | A burst of up to 256 reliable messages is written ahead of audio, which overflows meanwhile |
| 4 | Browser only drains the socket when egui draws | `sdroxide-ui/src/remote.rs` `poll_event` | Throttled/occluded Chrome window → backlog → server TCP buffer fills → #2. Not changed here; see step 3 |
| 5 | `data-wasm-opt="z"` (size-optimised wasm) | `sdroxide-web/index.html` | Possibly slower frames. Unmeasured; not changed |

## Done in this PR (server side, applies to every client)

1. `TCP_NODELAY` on every accepted connection.
2. Audio lane 8 → 16 packets (`AUDIO_LANE_DEPTH`, 320 ms). The client plays at
   most the newest 250 ms of a backlog (`fresh_tail`), so a longer stall is
   shed there rather than heard as stale audio.
3. Sender polls audio first, then reliable, then spectra (audio is ≤ 50
   packets/s so it cannot starve the others).
4. Dropped audio packets are now counted and reported:
   `audio lane full: the client is not draining its socket fast enough`
   (at most once per 5 s per radio).

No interface text changed, so no i18n messages are involved.

## Verification still owed

Not built or run here: the sandbox could not fetch Opus for `sdroxide-rade`,
so `cargo test -p sdroxide-server` was not run. Only the `tap_io` call was
compile-checked, in an isolated axum 0.8.9 crate. CI must run the workspace.

### A. Chrome web (the reported case)
1. Run the new server, open the page in a Chrome app window, listen 10 min on a
   voice mode and on FT8.
2. Watch the server log for `audio lane full`. None → fixes 1–3 were enough.
   Some → step 3 below.
3. In Chrome DevTools → Performance, check the frame time while the window is
   in the foreground, then covered/minimised.

### B. Native app (not yet checked)
1. Same station, same network, native remote client, 10 min.
2. Compare `receive audio: packets did not arrive from the server`
   (`remote.rs`, client log) with the web client's. Expect 0.
3. Note `vendor/ewebsock` native: one thread alternates send and a 10 ms
   `SO_RCVTIMEO` read — a possible extra latency source, only if A is clean
   and B is not.

### C. Linux (not yet checked)
1. Server on Linux and client on Linux (native and Chrome), same test.
2. `ss -tin dst <client-ip>` on the server during a stutter: look at `rtt`,
   `retrans`, `send-q`. A growing `send-q` means the client is not reading
   (item 4); `retrans` > 0 means a real network issue.

## If it persists

1. Browser draining (item 4): drain the socket in the `onmessage` callback
   into a JS queue independent of egui's repaint, so a throttled window does
   not back-pressure the server.
2. Measure `-O3` vs `-Oz` wasm-opt (item 5) on a mid-range machine; keep `z`
   only if frame time is equal.
3. Add a client-visible counter of lost audio packets (the client already
   counts `rx_lost`) to the status line so the next report carries a number.
