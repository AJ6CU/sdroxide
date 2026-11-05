# TODO — SDR Oxide Brown

Working queue. Rules in AGENTS.md. Last updated 2026-11-03.

State: `main` at crate 2.2.0, PROTO_VERSION 260 (0x50). Latest release
v2.2.3.6.2_brown (pre-release). Open PRs: 4 (#425 dynamic DNS, #434 public-SDR
favourites, #435 the carrier rule's corrected trigger, this TODO correction). Open issues: 7
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

- **Session QSO window: columns line up with their chips** (added by the operator, owned by Claude): the FT8 live logbook drew every cell at its text width, so an empty locator or CQ zone shifted the rest left and values sat under the wrong title. One shared width per column for chips and rows, numbers right-aligned, a `#` column, zebra rows and thin column lines. Layout only, no text added. Waiting for review; not checked on screen here.
- **PR #445 — SSTV: a frame locked onto noise is not saved, the panel says so, and the lock's evidence is adjustable** (added by the operator at the bench, owned by Claude): the save gate (rows 0.25 primary, sync 0.85 when nearly every line found its pulse — a noise frame scored 0.70 on the first, low bar and kept exactly what the gate exists to drop); `SstvStatus.not_saved` so the panel wipes the dropped frame and says why, in eleven languages; `DigiConfig.sstv_sync_pulses` (4/5/6, **6** by default) as a SYNC PULSES row. Both fields appended, one bump: PROTO_VERSION 262 → 263, `DigiConfig`'s pinned layout 479 → 480. Wacht op F4NRM.
- **PR #442 — SSTV's right edge is not the next segment** (added by the operator at the bench, owned by Claude): every scan ends against a tone change and the discriminator is still settling, so the last millisecond read the next segment — on Robot 36's half-width chroma that painted a green band down the right of every picture. Pixels within 0.9 ms of a scan's end repeat the last settled one. Test verified red ((0,186,0) without it). Wacht op F4NRM.
- **PR #443 — two receivers finishing the same picture keep both** (added by the operator at the bench, owned by Claude): a public SDR and the RSP1 completed the same frame in the same millisecond and the second overwrote the first; the first keeps the plain name, the later ones take `-2`, `-3`, as WEFAX's charts do. Test verified red. Wacht op F4NRM.
- **Bench, waiting on the operator's numbers:** `feat/sstv-save-gate` (the save gate; thresholds corrected to rows 0.25 / sync 0.85 after a noise frame scored 0.70 on sync) and `fix/sstv-freerun-six-pulses` (six pulses instead of four, so 27 MHz noise stops starting images). Both are in the local build of 2026-11-04 12:10.
- **PR #441 — SSTV keeps reading the VIS header while a picture is drawn** (added by F4NRM, owned by Claude, built on the operator's yes): the decoder stopped hunting for a header once an image started, so voice on the frequency or a free-run lock cost the real picture — drawn on the false lock's clock, which is #427 and #343's Scottie 2 shift. `step_vis` is now read by both phases; the guards are unchanged (leader, parity, stop bit, all bits below 1600 Hz), and a second test decodes Scottie 1, Martin 1 and Robot 36 end to end to prove a picture's own sync pulses do not restart it. Both tests verified red without the change; dsp lib 521 passed. Wacht op F4NRM.
- **PR #440 — test: pin what a CW click does to the dial** (added by drdatapi via #439, owned by Claude): the click arithmetic is identical to 1.6.11 and to upstream, so the two halves of the chain are pinned instead of argued — the tone lands at the operator's pitch, and the CW passband is centred on that pitch. #439's remaining suspect is the readout (QRG chip, off by default, and brown has its own config dir). Wacht op review.
- ~~PR #435~~ **merged 2026-11-03** — the carrier rule reads power, not the band's share (added by madmedicnl, owned by Claude): #432 merged a trigger the bench disproved within hours — over a real gap 1687 of 1687 windows read "in band", because the share test cannot see one. The rule keeps a twenty-second reference of the subcarrier band's power and ends a page after 90 s at 10 dB below it. Not fired on air yet. Wacht op review.
- ~~PR #425 — dynamic DNS~~ **merged 2026-11-03**: was back after F4NRM's review. The worker lives in the engine now (a headless Pi updates; a laptop UI cannot send its own address), the answer travels as `RadioEvent::DyndnsAnswer` / `ServerMsg::DyndnsAnswer` (PROTO_VERSION 260 → 262 on the branch), the address field is hidden, `answer_is_ok` reads the first word, and the token-in-clear note is in all eleven languages. Wacht op F4NRM.
- **PR #434 — public-SDR favourites** (added by madmedicnl, owned by Claude): a star on every row and a MINE chip, kept by address in the settings so a renamed receiver keeps its star. Three tests, four messages in eleven languages. Wacht op review.
- **#427 SSTV starts on voice** (added by F4NRM, owned by Claude) — ours after F4NRM made no move: the decoder stops looking for a VIS once an image starts, so a false start costs the real picture. #343 was closed into it. The fix (keep hunting while drawing) is proposed in the issue and waiting on a yes.
- **#421 NB with Kiwi** (added by trainrhd-del, owned by Claude) — the reporter came back: the blanker works on strong noise and not on weak, and its threshold is in samples, so it means something different at 20 kHz than at 768 kHz. A build that counts what it removes was promised.
- **#388 HL2 datagram loss** (added by DK8JV, owned by Claude) — the reporter confirms it gone in v2.2.3.6.1; today's WEFAX findings are posted there. Closes when his fax test comes back clean.
- **#402 "#283 still same problem"** (added by kvenl, owned by nobody) — a pointer to #283, which is closed; a candidate to close as a duplicate.
- **WEFAX bench test — stopped 2026-11-03 21:20 UTC**: propagation went before the night half could run. What the day gave is in Recently done; the logs are `/tmp/wefax-dwd3855-attempt1.log`, `-dwd3855.log` and `-dwd7880.log`.
- SWL users group requests (added by the SWL users group, owned by Roy) — point **4** is still ours: CTR ratchets the span down on keyboard tuning while the user cannot widen it again, exposed by #397's per-rate usable span; needs a `ctr_zoom_sim` case with `usable < rate`. Points 1 and 3 are in (#409, #406); 2 (start in Listening/Operating per radio) is parked.

Everything else that was in this list has been merged or closed since — see **Recently done** and the PR list on the board.

## Backlog — our open issues (board: Todo)


- #388 HL2 (HPSDR P1) datagram loss every ~2 s (added by DK8JV, owned by Roy).
- #340 S-meter (added by F4NRM, owned by Roy) — measurement disagrees with the report; the measurement is the first suspect.
- #331 Long labels in the tab strip (added by F4NRM, owned by Roy).
- #264 Public SDR stuck fetching the list in a web session (added by F4NRM, owned by Roy).
- #263 Zoom (added by F4NRM, owned by Roy).
- #251 FT8 continuous automatic operation (added by madmedicnl, owned by Roy; enhancement).
- #132 Wrong frequency display (added by emi83170, owned by Roy).

## Waiting on other people

- **Dream PR #51** (ours, offered 2026-11-03, no reply): `AacCodec::Decode` copies a fixed 960 samples per channel whatever faad2 reports, which is a segfault with faad2 ≥ 2.10. markjfine's `dream-mjf` branch fixes the *return value* (`eDecError` commented out, always `OK`) but the fixed-length copy is still there at line 242, and neither the codec-list nor the FFT-mutex patches are in it; his branch's other work is Qt, Hamlib, Windows and packaging, none of which we build.

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

- **SSTV's remaining modes** (Martin 3/4, Robot 12/24 and the B/W variants, Pasokon P3/P5/P7) — parked 2026-11-04 at the operator's word. We decode 16 modes and report the rest by name when a header arrives. The blocker is the spec, not the work: slowrx's table gives Martin M4 a 226.8 ms line with **two** rows and a 0.2288 ms pixel, which needs 1920 pixels in one line — 439 ms of scan in 226.8 ms, and the 0.1144 ms pixel that would fix it is 8.7 kHz of brightness, past what a 3 kHz SSTV channel carries. Robot 12 B/W has the same shape of contradiction (two rows per line, one luma scan). The published *picture durations* are consistent (M3 57 s, M4 29 s, Robot 24 36 s, Robot 12 18 s), so the line times are right and the structure around them is what does not reconcile. What would unblock it: KB4YZ's "SSTV modes – line timing" or N7CXI's proposal, or one 20–30 s recording per mode to check the timing against real audio.
- **STANAG 4481 FSK** (added by madmedicnl, owned by Claude) — parked 2026-11-03 until there is more context. What the look found: the *modem* is simpler than 4285's (75 Bd two-tone FSK, 850 Hz shift, no equaliser or Viterbi) but our 4285 is **4866 lines**, and the real work is the same class — bit timing with no preamble, the character framing (5N1, and 5N1.5 / 5N3.8 in the wild), the speed and shift variants (50–600 Bd, 850/170 Hz, the 16-channel V3) and the end of signal. The **output** is less: 4285 gives uncoded text, 4481 is usually KG-84, so what is delivered is a bit editor with characters rendered — the video the operator sent shows exactly that, not a decrypted message. Licensing: the complete Rust receiver (`hfmodem`, W9SSJ) is **AGPL-3.0-only** and not on crates.io, which would pull this GPL-3 tree under the network clause; GPL-compatible references are fldigi's FSK path, Rivet (Java) and a small MIT bitstream script. Realistic scope if it is taken up: 1000–1500 lines plus the view, and a real recording with a known message before anything is claimed.
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

- 2026-11-03 — Unit tests follow the machine's language (added by F4NRM, owned by Claude): on a French system 56 `sdroxide-ui` lib tests failed (aircraft, panels, frame… expect English); `main` in an English locale passed 1495/0. Not a leak between tests: "follow the system" read the OS locale. In test builds the system language is now empty (English) unless `SDROXIDE_TEST_SYSTEM_LANG` is set. **Merged**: #436.
- 2026-11-03 — WEFAX bench day on DWD 3855 (added by madmedicnl, owned by Claude). **Merged**: #429 (unique chart names), #430 (KiwiSDR tuning range), #431 (auto-slant fitted over the whole phasing signal — the 14:07 chart leaned 21 ppm, 51 px, under the 25 ppm a twenty-line measurement could act on), #432 (a page ends when the carrier goes away), #433 (schedule links to the keepers' own timetables), #417 (mode fallback — merged by F4NRM). **Open**: #435, because the carrier rule #432 merged reads the wrong thing and the bench proved it the same evening (1687/1687 windows "in band" over a real gap). What the day showed about the decoder itself: four pages ended on their own boundary (1353/1357/1434/1440 lines), the schedule charts are legible to the footer, and 8 ppm of slant is what is left after the fit. The test stopped at 21:20 when propagation went.
- 2026-11-03 — Merged or closed earlier and no longer in the queue: #426 (WEFAX page ends early), #424 (panadapter label crash), #417 (mode fallback), #414 (decodes gear), #415 (TX STOPPEN), the BW-chip preset PR, and the airport-card and PROGRAMS branches (no open PR on 2026-11-03).
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
