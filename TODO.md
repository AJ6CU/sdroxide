# TODO — SDR Oxide Brown

Working queue. Rules in AGENTS.md. Last updated 2026-11-03.

State: `main` at crate 2.2.0, PROTO_VERSION 260 (0x50). Latest release
v2.2.3.6.2_brown (pre-release). Open PRs: 3 (#425 dynamic DNS, #434 public-SDR
favourites, #435 the carrier rule's corrected trigger). Open issues: 7
(re-measured 2026-11-03).

## Task tracking rules

- Every task started: add to **Now** (one line: what, why, where from) and board
  **In Progress** before starting.
- Every task finished: move to **Recently done** (today's date, one line) and
  board **Achieved**; remove from In Progress.
- Every task blocked: move to **Waiting on other people** with the single question.
- Issues take priority over development.
- Each task must name who added it and who owns it:
  `Item (added by X, owned by Y) — description.`
- Re-measurable numbers (crate version, PROTO_VERSION, open issues/PRs) must be
  kept current in the state line.

## Now

- PR #426 — WEFAX pages closed a minute early, cutting the chart's footer: "blank" meant a line whose mean brightness cleared 0.9, which a chart's nearly-white footer does with ink on it. Blank now also demands no ink, and the fallback limit went 60 s → 120 s. Test verified red on the old rule (reported from the bench by madmedicnl, owned by Claude; decoder, so a PR). Wacht op F4NRM's review.

- PR #425 — Dynamic DNS: one provider update URL and an interval, off unless an address is set (no thread, no request), the URL never logged and shown masked, and only the *applied* address used. `NetworkConfig` gained two fields at the end, so PROTO_VERSION 260 → 261 (owner: Claude, asked by madmedicnl). Wacht op F4NRM's review.

- PR #424 — the crash that ended the program on the first frame with a signal in it (`min > max … 284.5 against 283.0`, and `584.0625` against `582.1875` the day before): `label_box` clamped a label into a box shorter than the label. Placement ordered instead, test verified red. Found from a KiwiSDR run at the bench where it killed two receivers within five seconds of connecting (owned by Claude). Wacht op F4NRM's review.

- PR — BW chip names the preset in force (was the measured width: a "3.0k" preset on 2.7 kHz edges read "BW 2.7k"), a fifth memory chip `M+` per mode (click stores the current passband, right-click edits, Reset empties it; `filter_presets.json`, slot after the built-ins), and a large 10 s hint under the BW chip, put away by a click, back on the next hover; 3 new messages in 11 languages (reported by the operator, added by madmedicnl, owned by Claude). Wacht op F4NRM's review.
- PR #417 — #413: a service decoder (ACARS, DSC, NAVTEX, WEFAX, SSTV, ADS-B, VDL2, AIS, HFDL, DAB, HD Radio, DRM…) falls back to the band's own mode on a band change; a mode you carry (SSB/FM/CW, FT8/FT4/JS8, PSK, RTTY, Olivia…) keeps following. `Mode::belongs_to_a_service`, test verified red (reported by drdatapi, owned by Claude). Wacht op F4NRM's review.
- #388 HL2/HPSDR P1 loses 2-10 datagrams per 2 s (reported by DK8JV, owned by Claude) — **the reporter confirms it is gone in v2.2.3.6.1**, so it was the host's socket buffer and #396 fixed it; open only until his fax test comes back clean — the two seconds is our own reporting interval, so the loss is continuous; his build is v2.2.3.5, from before #396's 4 MB receive buffer. Asked him to try v2.2.3.6.1 and send the `UDP receive buffer … bytes` line. **Be careful here: at the bench an identical symptom was the operator's OS update, not our code.**

- PR #414 — decodes: the settings gear was painted over by LIVE LOG and there were two chips for the same flag (reported by Roy, owned by Claude). One right-to-left run now; the worded duplicate gone with its message in all eleven languages. Wacht op F4NRM's review.
- PR #415 — digi: TX STOPPEN left the QSO machine in the calling step, so CALL CQ stayed greyed out with no QSO on screen (reported by Roy, owned by Claude). `abort_tx` ends the QSO; test verified red. Wacht op F4NRM's review.

- SWL users group requests (added by the SWL users group, owned by Roy) — three of four in review: **1** band/mode dock remembered per radio (PR #409), **3** a band never visited keeps the mode in use (PR #406), **4** CTR ratchets the span down on keyboard tuning while the user cannot widen it again — ours, and exposed by #397's per-rate usable span, needs a `ctr_zoom_sim` case with `usable < rate`. **2** (start in Listening/Operating per radio) parked.

- Translation rules in AGENTS.md and the "new language" sync rule in CLAUDE.md (added by F4NRM, owned by Claude) — branch `f4nrm/compassionate-mendel-e3kjll`, PR pending: the maintainer's agent is driven by AGENTS.md, which lacked the translation rule, and shipped English-only messages (#395); AGENTS.md now carries the eleven-language rule, and CLAUDE.md says a new language must update the list in both files. Docs only. Wacht op F4NRM's review.
- Airport card: one airport = one card (added by F4NRM, owned by Claude) — branch `f4nrm/youthful-mayer-38fi1q`, PR pending: radio aids within 3 km of a field (EPL beside LFSG) are listed on the airport's card and no longer have a symbol that steals the click; the card shows height, distances, runway lengths and temperature in both systems; "no frequency published" is said instead of silence.
- Airport card: Wikipedia picture stays on "…" (added by F4NRM, owned by Claude) — same branch and PR: repaint when the picture arrives (the poll runs after the map is drawn), a 404 from the station settles at once instead of three retries.
- Airport card: 91 airports never got a picture (added by F4NRM, owned by Claude) — same branch and PR: `Article::new` refused titles with an en dash or a quote (KATL, CYUL, EDAC…); test over every title of `wiki.bin`.
- Pinned ADS-B card: lookup follows the question, thumbnail comes back, retry countdown (reported by F4NRM, owned by Claude) — branch `f4nrm/bold-ramanujan-ckwlff`, PR #394: a click on empty map now closes the pinned card (no ✕ needed for beginners); after a closed card the picture was never asked again (`thumb_wish` depended on the pressed lookup), and a failed or lost question was never retried on the map card (only the Cards view asks every frame), so the card stayed on radio data. Now the pressed lookup is followed every frame, a failure shows "trying again in N s" with the button back, and the picture is wanted whenever the answer has one.
- PROGRAMS list overflow in long languages and wide windows (added by F4NRM, owned by Claude) — branch `f4nrm/kind-maxwell-8m2ipv`, same family as MEMORIES #373: columns built for the width left after the scroll bar, window floor follows the language, `schedule_preview` test renders the real window. Update: gap after the name capped at 40 characters, right-hand group against the edge. Update 2: PROGRAMS state (TOUT, stars, filters, colours) moves to the station — `/programs-state` route + client sync, nothing kept in the browser. Built and tested: server (7 tests), ui (22), types, wasm32 check; the sandbox needs `OPUS_URL` set to a local zip of opus 940d4e5 (git fetch + `git archive`) because the archive download is refused. Wacht op F4NRM's review.
- PROGRAMS window layout and persistence (added by F4NRM, owned by Claude) — branch `f4nrm/stoic-fermi-a0z0zp`, PR pending: columns, Sort menus, chip-framed colour, two-row bar, saved filters. Wacht op F4NRM's review.
- SAT Doppler for CAT rigs (added by Roy, owned by Roy/SP9TKN) — step 1+2a+2b+2c
  implemented on branch `fork/sat-doppler-cat`. MSI build available. Wacht op
  SP9TKN's testresultaten. Design doc: `docs/SAT-DOPPLER-CAT-PLAN.md`.
- NAVTEX restore (added by Roy, owned by Roy) — PR #317 open, mergeable. Wacht
  op F4NRM's test.
- ADS-B overlay (added by Roy, owned by Roy) — PR #312 open. Wacht op F4NRM's
  review (relay rule gefixt).
- Airspy HF+ per-rate usable span (added by Roy, owned by Roy) — PR #309 open.
  Wacht op Graeme's test.
- Antenna calculator (added by Roy, owned by Roy) — PR #304 open.
- FT8 auto queue UI (added by Roy, owned by Roy) — PR #323 open (not
  live-tested).
- NAVTEX sync-drop (added by Roy, owned by Roy) — PR #308 open.
- UDP feed VDL2/ACARS/DSC/HFDL (added by Roy, owned by Roy) — PR #318 open.
  Rolling-window fix toegepast.
- APRS panel: all messages heard, 500 kept, composer note, RX/TX engine tests (added by madmedicnl, owned by Roy; touches F4NRM's APRS area, PR for his review) — decoder fault #259 not reproduced in the engine; needs a real capture.
- Per-band TX permission (added by madmedicnl, owned by Roy; touches the radio engine and every radio tab, PR for F4NRM's review) — PR #358 open, not merged. Every band has a YES/NO switch in Settings; new install = all off, upgrade = bands carried over but TX blocked until the operator confirms once per radio; not applied on SWL/listen screens. PROTO_VERSION 259 (0x4F). Needs a hands-on test with a real radio.
- TX band banner is a warning, not a lock (added by madmedicnl, owned by Claude; touches every radio tab, PR for F4NRM's review) — PR A: the dial is never held or put back, no forced listen mode, no "Listen mode" button, "Close" only; one message in the interface language (5 languages); at most 3 showings per session and per radio tab, count not saved. Fixes the false banner on 80 m at 3.643 MHz (stale radio config) and the Radio classic waterfall click that snapped back.
- BAND TX pill + "11M-CB" label (added by madmedicnl, owned by Claude; PR B, re-pushed on main after #377 merged into the PR A branch) — pill under "BW cascade": green = allowed, red = refused, blinking red = transmitting; hidden on SWL, listen mode, receive-only radios and when no band is ticked and confirmed. "11M" shown as "11M-CB" in the TX band plan only.
- APRS panel fit and heard text (added by madmedicnl, owned by Claude; touches F4NRM's APRS area) — branch `f4nrm/amazing-einstein-sqkc6q`, PR pending: the message pane is sized from what is really left (map chip row and gaps counted) so the composer no longer hangs off the window bottom; ALL also lists station comments/status heard. Not live-tested on screen.
- MEMORIES panel fit (added by F4NRM, owned by Claude) — branch `f4nrm/memories-panel-fit`, PR pending: name rows sized to their button, chip-framed colour, PST at chip height. Wacht op F4NRM's review.
- FT8 setup window drew its captions with no controls beside them (#357) (added by F4NRM, owned by Roy; FT8 is his area, PR for his review) — `setup-tx-audio` carried a hint paragraph as well as the caption, the dialog draws it as a row label of a two-column grid, so the label column measured 1356 pt and every control beside it was pushed off the window. Removed from all 11 languages, render probe + caption test added. PR #390.
- Settings reports the crate version, not the installed release (#354) (added by F4NRM, owned by Roy) — a release now reports the tag's own version (`SDROXIDE_RELEASE` from `GITHUB_REF_NAME`), and the window title follows it. PR #392.

## Backlog — our open issues (board: Todo)

- **STANAG 4481 FSK** (added by madmedicnl, owned by Claude) — doable, per a worked decode shown on video: <https://www.youtube.com/watch?v=ofiSzScw6RM> ("Decoding STANAG 4481 FSK"). Kept as the reference for the mode: FSK, not 4285's PSK, and the video is the evidence that off-air decoding is within reach. Not started; the scope and the legal question get settled with the operator first, as 4285's coded payload did.

- #388 HL2 (HPSDR P1) datagram loss every ~2 s (added by DK8JV, owned by Roy).
- #340 S-meter (added by F4NRM, owned by Roy) — measurement disagrees with the report; the measurement is the first suspect.
- #331 Long labels in the tab strip (added by F4NRM, owned by Roy).
- #264 Public SDR stuck fetching the list in a web session (added by F4NRM, owned by Roy).
- #263 Zoom (added by F4NRM, owned by Roy).
- #251 FT8 continuous automatic operation (added by madmedicnl, owned by Roy; enhancement).
- #132 Wrong frequency display (added by emi83170, owned by Roy).

## Waiting on other people

- ~~STANAG 4285~~ **finished** (2026-10-29) — Mark confirmed decode with video.
  Coded payload decoding intentionally not implemented (restricted by law in
  several countries). Rust equaliser still does not converge.
- ALE — never decoded a real burst. Needs recording.
- S-meter jumping with mouse — question to reporter: number or needle?
- #105 AppImage — planned for stable release.
- #265 "outside receive range" — partially fixed (notice logged, not shown).
  Second message (2.941 MHz) still open.
- #239 WEEFAX — not fixed.
- #262 SSTV editor freeze — not fixed.
- #263 Zoom — not fixed.
- #264 Public SDR — not fixed.
- #259 APRS — not fixed.
- #257 NAVTEX — PR #317 open, wacht op test.
- #132 Wrong frequency display — not fixed.
- #256 SSTV overlay preview — enhancement.
- #252 Central logbook — enhancement.
- #251 FT8 continuous operation — core logic built, queue UI added (PR #323).
  9 demonstrations not verifiable without test.
- #249 SSTV layout templates — enhancement.
- #302 Memories paste-knop — F4NRM oppakt het.
- #283 Settings question — gesloten (fix da78715b).

## Parked
- WEFAX broadcast schedules (added by Roy, owned by Roy) — station picker chip with times/links, offline data. Batch with other UI tweaks.

- 4 brede PSK-banken (PSK125C12, PSK500C4, PSK500RC4, PSK1000C2) — 0/3
  words. Geparkeerd tot iemand ze op de band hoort.
- Antenna modelling tool — branch fork/antenna-tool, MoM solver.
- Satellite telemetry decoders — ISS gedekt, Fox/QO-100 BPSK, per-mission
  layouts nodig.
- FT4 console, SSDV across passes, Codec2 playback — ideeën, niet gescopeerd.
- FST4W — do not ship as wired.
- SDRuno look — parked, operator's pet project.
- DMR — out (patent).
- ATS Mini / DiamondRadio — closed/unlicensed (research only).
- AppImage — beslissing nodig.

## Recently done
- 2026-11-03 — WEFAX bench day on DWD 3855 (added by madmedicnl, owned by Claude). **Merged**: #429 (unique chart names), #430 (KiwiSDR tuning range), #431 (auto-slant fitted over the whole phasing signal — the 14:07 chart leaned 21 ppm, 51 px, under the 25 ppm a twenty-line measurement could act on), #432 (a page ends when the carrier goes away), #433 (schedule links to the keepers' own timetables), #417 (mode fallback — merged by F4NRM). **Open**: #435, because the carrier rule #432 merged reads the wrong thing and the bench proved it the same evening (1687/1687 windows "in band" over a real gap). What the day showed about the decoder itself: four pages ended on their own boundary (1353/1357/1434/1440 lines), the schedule charts are legible to the footer, and 8 ppm of slant is what is left after the fit. The test stopped at 21:20 when propagation went.
- 2026-11-03 — #343 (Scottie 2 colour shift) closed into #427: F4NRM's own comment there says the decoder is activated by voice without an SSTV signal, and a picture that starts on a false lock reads its colour channels from the wrong line.
- 2026-11-02 — BW chip shows the active preset's name instead of the measured width, `M+` memory chip as the fifth preset, large timed hover hint on the BW chip (see Now; PR for F4NRM's review).
- 2026-11-02 — FT8 on the CRT SS9900v fell away mid-over (reported by Roy, owned by Claude): **the USB topology, not the audio**. The radio's sound card sat behind two hubs, sharing them with a USB mass-storage device and a Billboard device, all at full speed; isochronous audio cannot hold its 1 ms frames there, so the card's output stuttered and the VOX let go. Straight on a root port: no drops, no warnings, a clean 100 s run. Ruled out on the way (each by measurement, not reasoning): the card itself (5 s read directly, 240022 frames, real signal), the app's audio (48 000 samples/s for the whole 12.64 s over, peak 1.000, `the over finished` on time), the radio (10 W, SWR 1.1, TOT 6:05), the band rails (no refusal logged), the mute (it was on, and that accounted for the silence the operator heard), and the tone offset (2727 Hz sat above the rig's 450–2500 Hz SSB passband; now 810 Hz).
- 2026-11-02 — #408 and #409 merged by F4NRM after the review rounds.
- 2026-11-01 — Custom receive filter presets: right-click a chip in the BW popup to edit label and edges (`filter_presets.json`), Reset restores the built-in chip, CW keeps the operator's pitch; 12 new messages in 11 languages. Wacht op F4NRM's review.
- 2026-11-02 — PR #409 (band dock) review round: the popup's "window" entry and the dock header's two chips wrote `band_docked = false` without remembering it; `band_dock_open` is now a map (key = docked, value = shown), the write re-reads the stored map so a second tab cannot erase the first radio, docs put back on their functions, SAFETY on the test's env var (added by F4NRM, owned by Claude; PR back for his re-review)
- 2026-11-02 — PR #408 (transmit bands) review round: the settings grid bypassed `set_tx_band` (the bench case was still open under a green "confirmed"), so the grid now goes through `apply_tx_band_choice`; three tests, all verified red on the old body; German "eines Bandes"; upgrade note in the description (added by F4NRM, owned by Claude; PR back for his re-review)
- 2026-11-02 — PR #395 (WEFAX station picker) merged by F4NRM at 23:26Z on 2026-11-01 (added by madmedicnl, owned by Claude)
- 2026-11-01 — WEFAX measured on the air until four charts were asked for: the phasing between two transmissions was drawn into the picture instead of cutting the page (strict pulse test wanted a line mean ≤ 0.20; off air it measured 0.28–0.50 with the pulse at 1.00), so charts were glued together and pages ran to `MAX_LINES` 2600. Median + contrast, one missed line no longer resets the run, two pieces of the real recording in the tree as reference (PR #401, merged). Auto-slant guarded against a 47 500 ppm mis-read (PR #393). The two tests our last merges left red (PR #391) (added by F4NRM, owned by Roy)
- 2026-11-01 — Paired radios (transceiver + attached panadapter receiver): the S-meter of the transceiver tab is now the attached receiver's (engine measures its I/Q; `PanadapterSource::rx_signal_dbm` returns `None`; with no main chain the channel power is read off the spectrum; audio level only as last resort). Before, a CAT rig without a usable meter (Wolf) made the meter follow its audio level. Squelch/scan unchanged. Tests in `external_rx_audio.rs` and `panadapter_source.rs` updated. Trade-off: a rig with an accurate CAT S-meter no longer drives the meter in a pairing.
- 2026-11-01 — TX lock on listening/broadcast frequencies: PTT and TUNE (desktop strip and classic layout, which had no guard) are greyed with a big red message on hover and on every press (mouse, key, MIDI; uncapped); one rule — transmit only on bands ticked AND validated in Settings → Radio, refused everywhere else (SWL/broadcast/Other included, no more separate amateur-only rail while the list is in force); "Allow transmit on 11 m (CB)" and the 11M-CB tick are one setting in both directions; unknown capabilities grey the PTT. Not yet built or tested here: the sandbox cannot download Opus for `sdroxide-rade` (403), so the UI crate was only syntax-checked; `sdroxide-types` tests pass. The engine's band refusal is typed (`RadioEvent::TxRefused` / `ServerMsg::TxRefused`, PROTO_VERSION 259 → 260) and worded in the operator's language by the UI; other engine refusals stay English. Branch `f4nrm/relaxed-wozniak-b4m3da` (added by madmedicnl, owned by Claude; PR only with F4NRM's OK)
- 2026-11-01 — CTR on FT8/FT4/JS8/WSPR/Pi4/PSK centres the middle of the decode band (dial + 1500 Hz) instead of the dial, so the whole 0–3 kHz spectrum is in the window and the station boxes sit on their signals; `ctr_zoom_sim` gesture test added (added by F4NRM, owned by Claude; PR for his review)
- 2026-10-31 — Airport card (PR 2 of 4): the pinned card lists up to 4 runways and 6 frequencies from embedded OurAirports tables (offline, dated), lengths in the chosen units, and "aircraft within" reaches 100 km instead of 25 (added by madmedicnl, owned by Roy)
- 2026-10-31 — Aircraft online relay is ON by default and one visible "Online" switch sits at the top of the AIRCRAFT window (was off, hidden under a selected card, 4 clicks away). The station keeps an "off" mark (`aircraft-online.off`); `SDROXIDE_AIRCRAFT_ONLINE=0` holds it off, `=1` on. First of 4 PRs for the airport card (runways/frequencies, Wikipedia thumbnail, local time/temperature follow) (added by madmedicnl, owned by Roy; changes a privacy default, PR for F4NRM's review)
- 2026-10-30 — Aircraft photo: the station keeps 10 thumbnails for 30 s (was 1), loads several aircraft at once (up to 3, same aircraft once) instead of answering 429, frees its place even if a download panics, a closing card releases only its own picture, and a failed thumbnail is asked again (3 times) while the card stays open (added by madmedicnl, owned by Roy)
- 2026-10-29 — Every message log keeps at most 200 (ACARS, DSC, APRS, VDL2 were 300/300/300/2000), VDL2 list newest first (added by madmedicnl, owned by Roy; touches F4NRM's DSC/NAVTEX/APRS caps, PR for his review)
- 2026-10-29 — ACARS channel chips tune the VFO in use, light the real channel, grey out unreachable ones (added by madmedicnl, owned by Roy)
- 2026-10-29 — Aircraft online read: a failing or slow card no longer blocks the others (3 at once, 15 s limit, retry 30 s, failure says so) (added by madmedicnl, owned by Roy)
- 2026-10-29 — Aircraft photo a quarter larger: map card 84→106 px, Cards view 140→175 px (added by madmedicnl, owned by Roy)
- 2026-10-29 — Language selector in Settings → UI with Apply button (PR #339, #331)
- 2026-10-29 — S-meter calibration removed from UI and reading (PR #341, #340)
- 2026-10-29 — SSTV received-image viewer nav + composer text transfer (PR #333)
- 2026-10-29 — UDP feed rolling-window unit tests added (PR #318)

- 2026-10-29 — ADS-B overlay (adsb.lol, spearheads, relay fix, manual)
- 2026-10-29 — NAVTEX restore to F4NRM's PR #157/#161
- 2026-10-29 — NAVTEX sync-drop (DK8JV suggestion)
- 2026-10-29 — UDP feed VDL2/ACARS/DSC/HFDL + rolling-window fix
- 2026-10-29 — Airspy HF+ per-rate usable span
- 2026-10-29 — Antenna calculator (MoM solver, presets, 3D, Yagi optimizer)
- 2026-10-29 — FT8 auto queue UI
- 2026-10-29 — Grey-line gradient confirmed
- 2026-10-29 — S-meter troubleshooting (wireless mouse RFI) added to manual
- 2026-10-29 — STANAG 4285 finished (coded payload not implemented — legal)
- 2026-10-29 — Olivia proven (RX+TX)
- 2026-10-29 — S-meter still an issue
- 2026-10-31 — Airport card (PR 4 of 4): the pinned card shows the airport's local time and temperature, online only — the station asks Open-Meteo (`/airport-weather?lat=&lon=`, only the airport's position rounded to ~1 km, kept 30 s in memory, same sign-in/switch/ceiling as the other online routes, credit shown); local time is the client's UTC clock moved by the zone's offset (summer time included), 12 h/°F in Imperial and 24 h/°C otherwise; offline it reads "UTC hh:mm (local time: offline)", never an invented local time (added by madmedicnl, owned by Roy)
- 2026-10-28 — NAVTEX false-sync fix
- 2026-10-28 — WEFAX negative auto-correct
- 2026-10-28 — #265 "outside receive range" notice fixed (logged, not shown)
- 2026-10-27 — Aircraft window clean-up (10 PRs)
- 2026-10-27 — #283 settings question resolved (da78715b)
- 2026-10-31 — Airport card (PR 3 of 4): the pinned card shows the airport's Wikipedia thumbnail, asked of the station (`/airport-photo?lang=&title=`, hosts fixed to `*.wikipedia.org` and `upload.wikimedia.org`), held in memory for the open card only and dropped when it closes, credit links to the article; `PhotoStore` is now generic on its key (added by madmedicnl, owned by Roy)
