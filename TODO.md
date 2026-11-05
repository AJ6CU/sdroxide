# TODO — SDR Oxide Brown

Working queue. Rules in AGENTS.md. Last updated 2026-11-04.

State: `main` at crate 2.2.0, PROTO_VERSION 264 (0x54). Latest release
v2.2.3.6.2_brown (pre-release). Open PRs: 2 (#470 ours — the WEFAX page says what
the trim was fitted over; #467 F4NRM's HPSDR Protocol 2 diversity, conflicting
and his to rebase). Open issues: 5 (re-measured 2026-11-05).

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

- **PR #470 — a WEFAX page says what the line-clock trim was fitted over** (added by madmedicnl, owned by Claude): the trim is invisible in the picture, and the auto-slant's two floors (25 ppm from twenty phasing lines, 10 from fifty) mean "it acted" and "it never had enough signal" differ by a number nothing shows. The bench's five charts left −11.5 to +12.3 ppm over, just above the fine floor, so the fit was probably starved — but the operator's own charts come in straight, so the number has to come from DK8JV and F4NRM. Asked on #444 for one evening's charts plus the log lines.
- **#421 NB with Kiwi** (added by trainrhd-del, owned by Claude) — the promised count is in (**#456** merged): while the blanker is on it writes one line a second with how many samples it zeroed, how close the strongest came to the 5× bar, and the device rate. A simulation of the shipped rule says the dependence is real and in our code — at the same impulse strength 20.3 kHz blanked 14 of 258 bursts where 768 kHz blanked 207 of 297 — but the reporter's own log lines decide the fix, and he gets them in the next pre-release.
- **#388 HL2 datagram loss** (added by DK8JV, owned by Claude) — the reporter confirms it gone in v2.2.3.6.1; today's WEFAX findings are posted there. Closes when his fax test comes back clean.
- **#427 SSTV starts on voice** (added by F4NRM, owned by Claude) — fixed by #441 (merged); F4NRM said he would confirm it on air once a release carries it, and asked that it not be closed until then.
- **WEFAX bench test — stopped 2026-11-03 21:20 UTC**: propagation went before the night half could run; the DWD 3855 sweep of 2026-11-04 ran 14:20–16:10 UTC and gave five complete charts, and WEFAX is finished from our side. Logs: `/tmp/wefax-dwd3855.log`.
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

- **PR #453 — decoded CW over UDP** (AJ6CU, 2026-11-04; owned by Claude) — the port of upstream `1dd977f9` + `d6fb61a4`. Two things stand between it and a merge: the `WsjtxConfig` postcard round-trip test (written, verified red, pasted in the PR — a maintainer push to his fork is refused with 403), and a rebase, because today's seven merges moved `engine.rs` and `proto.rs` under it.
- **PR #464 — CW centring, AJ6CU's version** (AJ6CU, 2026-11-04; owned by Claude) — the same diagnosis and the same fix as our #463, which went in first, so it conflicts. The half still worth having is the clamped-width comparison and `a_width_past_the_maximum_is_sent_once`; the comment asks him to rebase down to that, or to let us add it.
- **PR #453 — decoded CW over UDP** (added by AJ6CU 2026-11-04, owned by Claude) — the wire rule is the only gap left: `WsjtxConfig` rides `Command::SetWsjtxConfig` whole and has no postcard round-trip test. The test is written, verified red by skipping `cw_text` on the wire, and posted in the PR as a paste-ready block; the branch is his fork and a maintainer push is refused (403), so it cannot be merged until it is in. Question to AJ6CU: will you add the test to `brown-cw-text-udp` yourself? The other three (#451, #452, #454) are ready and need nothing from him.

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

- **USB sound card lost mid-session** (added by the operator at the bench, owned by Claude) — parked 2026-11-04 at his word: his station is an exotic setup, and this is not the moment. What the bench did show, so it is not lost: pulling the card while the app runs kills the **radio audio input** and the **audio output** (`22:52:53`, both logged as "device is not available"), and neither is reopened — only the microphone is, and on his bench that stream never broke (#461, merged, is right for a microphone and does nothing for this). The app then hung at `shutdown: joining the engine thread`, with the engine still alive but starved (0.65 rows/s). Unpark with a thread stack from a reproducible run: `gdb -p PID -batch -ex "thread apply all bt"`. Order matters: a reopen loop against a device ALSA has wedged may be what hangs, so understand the hang first.

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

- 2026-11-05 — #453 and #464 merged by F4NRM (AJ6CU's): decoded CW over UDP with its `WsjtxConfig` round-trip test, the 9998 note, the manual and three messages in all eleven languages, `PROTO_VERSION` 263 → **264**; and the clamp half of the rigctl CW follow-ups on top of our #463. The two upstream wire commits are unblocked by it and take 265 and 266.
- 2026-11-04 — Seven of ours merged by F4NRM in one evening (added by madmedicnl, owned by Claude): **#456** the noise blanker's count, **#458** the CW click grid capped at the fine one in CW (plus the removal of the `cw click:` diagnostic that #450's `git add -A` had swept into `main`), **#459** a PTT release ends a keyed over (upstream `5f9f1aa3`), **#460** public SDRs fall back only to tight fuzzy matches (upstream `ee5b7fbf`), **#461** a microphone whose stream the host broke is reopened (upstream `eeb6bfc5`; **still needs the bench** — card out and in mid-session), **#462** the TODO mirror, and **#463** a CW width over rigctl centred on the sidetone (F4NRM's review point on #451). Main after all seven: gate silent, `sdroxide-rigctld` 50, `nb` 5, `fuzzy` 13, `publicsdr` 10, `cw_ptt` 2, `local_controller` 1.
- 2026-11-04 — #465 merged (AJ6CU's own): the `\dump_state` level-mask test, which was F4NRM's second point on #452.
- 2026-11-04 — #439 closed as user error (added by drdatapi, owned by Claude, the operator's call): the click, the readout and the release's code were all right; his **Click-tune rounding** was set to 1000 Hz for hunting SSB, and in CW that is a one-kilohertz grid, so a click landed up to half of it away. His 1.6.11 comparison is explained by that install still having 10 Hz. Open question left with the operator: should the grid fall to the fine one whenever the CW decoder is in front, so the coarse setting cannot bite there.
- 2026-11-04 — #132 (wrong frequency display) closed (added by emi83170, owned by Claude, the operator's call): the answer was already in the thread — the Brick2's I/Q polarity (`Settings → Radio → Swap I/Q`) for the mirrored stations, and IARU Region 2 for the 7.200 MHz band edge — and it has been quiet since 28 October. The close asks for a reopen if it is still there, which keeps the videos and the history together.
- 2026-11-04 — #402 closed as a duplicate of #283 (added by kvenl, owned by Claude, the operator's call): a pointer to a closed issue cannot be worked from; a fresh report with the version and what is actually seen is what gets it looked at. #419 (DRM) answered the same day: not in the queue now, a PR would be reviewed, otherwise F4NRM's to pick up — with a line on what "not working" means asked for so it is actionable.
- 2026-11-04 — AJ6CU's rigctl set merged (added by AJ6CU, owned by Claude): **#451** a new width in the mode already running sets the filter and leaves the mode alone, with the `PKT` guard so a digital session still ignores WSJT-X's periodic write; **#452** squelch, AGC and NR strength as Hamlib levels, and `U NR 1` now restores the engine and strength last on instead of always the spectral NR at medium; **#454** the front-end gain as `RF`, advertised in `\dump_state` only where a gain stage exists. Checked on a local build before merging: gate silent, `sdroxide-rigctld` 48 tests, and `l RF` moves the same element the top bar's slider moves. **#453** (decoded CW over UDP) is blocked on its `WsjtxConfig` round-trip test — see Waiting on other people.
- 2026-11-04 — SSTV's save gate reads the rows only (added by the operator at the bench, owned by Claude; **merged** #450): four of nine frames kept in one evening were speckle with a sliver of picture at the top, their rows 0.03–0.10, and the sync arm at 0.85 had let them through — so the share is not a discriminator at any threshold, and the rows are. A genuinely flat frame is the casualty and SAVE ALL is the way back. Red verified with the gate off; the reason code `SstvDrop::NoPicture` stays in the type for a peer that sends it.
- 2026-11-04 — WEFAX finished from our side (added by madmedicnl, owned by Claude): five of five charts complete on DWD 3855, each ending on the station's own boundary, posted in #444. Residual slant after the auto-slant, measured on each chart's printed frame line: −11.5, −2.9, +6.5, −1.1, +12.3 ppm — a 24 ppm spread from one transmitter to one receiver in 70 minutes, so the varying part is our twenty-line measurement and not the clocks. Reported in #444; the better fit (over the whole chart's frame line) is F4NRM's call.
- 2026-11-04 — house rules recovered (**merged** #449): "read your own diff before pushing" and "follow a name or format change to its readers", lost to a branch mix-up; and the tone rules (not submissive, short, friendly).
- 2026-11-04 — SSTV merged by F4NRM: #441 the VIS header is still read while a picture is drawn, #442 the right edge is not the next segment, #443 two receivers finishing the same picture keep both, #445 the save gate with `not_saved` and the SYNC PULSES row, #446 WEFAX page start and finish logging.
- 2026-11-04 — #447 airport card accepts `thumb.wikimedia.org` and #448 the map's aircraft, airport and VOR symbols (both F4NRM's own, merged).

- 2026-11-04 — Airport card shows its Wikipedia thumbnail again (added by madmedicnl, owned by Claude): the summary API now hands out thumbnails from `thumb.wikimedia.org`, which the station refused (only `upload.wikimedia.org` was allowed), so the card stayed empty; both Wikimedia picture hosts are accepted, https/no port/no credentials as before. Next: airport symbol on the map instead of a circle.

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
