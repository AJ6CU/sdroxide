# Instructions for agents working in this repository

## Every piece of interface text is translated

The interface speaks English, French, Spanish, German, Dutch, Italian, Brazilian Portuguese, Russian, Turkish, Japanese and Simplified Chinese. **Any change that
adds or edits text a person reads on screen must add or update the message in
all eleven languages in the same change.** That covers labels, buttons, hover
text, window titles, notes, status lines, error messages shown to the operator
and the names of values (`label()` of an enum drawn on screen).

How:

1. Never write display text as a string literal in `crates/sdroxide-ui`.
   Use `tr!("some-id")` (`use crate::i18n::tr;`), or `tr!("some-id", n = 3)`
   for a message with `{ $n }` placeables, or `tr!("some-id", "hint")` for an
   attribute. `tr!` returns a `String`; pass `&tr!(..)` where `&str` is wanted.
2. Add the message to every language with the helper (it locks the files, so it
   is safe to run in parallel):

   ```bash
   python3 tools/i18n_add.py <<'JSON'
   {"id": "area-thing", "en": "Thing", "fr": "Chose", "es": "Cosa", "de": "Ding", "nl": "Ding", "it": "Cosa", "pt-BR": "Coisa", "ru": "Вещь", "tr": "Şey", "ja": "もの", "zh-CN": "东西"}
   JSON
   ```

   Ids are `area-words-of-the-text` in lower case with hyphens. Keep capitals
   where the English has them (captions in capitals stay in capitals).
   Keep `{ $name }` placeables as they are. Do not translate ids, file names,
   protocol names, mode names (FT8, SSTV…) or units.
3. For a value an enum shows on screen, add the arm in
   `crates/sdroxide-ui/src/i18n/labels.rs` (a `match` that must stay
   exhaustive) rather than calling `label()` from the interface.
4. `const`/`static` items cannot call `tr!`. Keep the English identity there and
   translate when drawing (see `tab_label` in `app/settings/mod.rs`).
5. Run `cargo test -p sdroxide-ui --lib i18n` before pushing. It checks that
   every message exists in English, that every placeable matches, and that no
   message is unused.

Read `crates/sdroxide-ui/i18n/README.md` for the Fluent syntax and the rules for
translators. A change that adds a literal to the interface and no messages is
not finished.

### Adding a language

Languages are the folders under `crates/sdroxide-ui/i18n/` (one per tag, the
steps are in its `README.md`); no Rust list has to change. **The list of
languages is also written in this file (above) and in `AGENTS.md`, and the
agent of the other contributor reads only `AGENTS.md`.** So a change that adds
(or removes) a language must, in the same PR:

1. update the list and the word "eleven" in the first paragraph of this section;
2. update the language list and the word "eleven" in `AGENTS.md`, section
   "Every piece of interface text is translated", and the `tools/i18n_add.py`
   example above so it carries a key for the new tag;
3. give the new language every message (or accept the English fall-back only
   if the PR says so in its description), and run
   `cargo test -p sdroxide-ui --lib i18n`.

A language added in the folders but missing from `AGENTS.md` means the other
agent keeps writing ten languages.

## CTR on = the frequency is always in the middle

With the CTR chip lit the red line is at the centre of the waterfall on every
frame — wheel, pinch, keyboard zoom, pan, fit, mode or decoder change, every
radio, the whole bandwidth, zoom in or out, fast or slow. No drift, no jitter,
no off/on to re-centre. Modes keep their preset bandwidth; CTR only moves the
line. Exception by design: modes that decode anywhere in the passband (FT8, FT4,
JS8, WSPR, Pi4, PSK) centre the middle of their band (`Mode::audio_band_centre_hz`,
dial + 1500 Hz), like SSTV's picture. Touching waterfall or zoom code: keep the `ctr_zoom_sim` tests in
`widgets/spectrum_view.rs` green and add the gesture.

## Working rules

- Answer and plan in French, short and direct. No automatic approval window
  (do not call ExitPlanMode): ask questions as plain text in the chat.
- **Pull requests are written in English** (title, description, comments), so
  the other contributors can read and review them. Talk to the user in French.
  Push the branch and open the PR only when asked; **never merge it and never
  follow it** (no `subscribe_pr_activity`) unless the user says so.
- You must complete TODO.md
- You must write on multiple lines if you are doing multiple PRs and complete the TODO.md file.
- **11 m / CB and the FT modes are the core audience: ask madmedicnl before
  branching.** Per-band transmit settings, the USB sound card as a radio
  interface, and the FT modes (FT8/FT4/FT2/JS8) and their audio chain. A fault
  may be reported at once; whether we fix it the way proposed is his call. Ease
  of use counts as part of the fix. See the same rule in `AGENTS.md`.

## Building in a sandbox without GitHub archive access (Opus)

`sdroxide-rade` (RADE) builds a patched Opus from a GitHub **archive** zip. In a
cloud session the proxy answers **403** to `github.com/.../archive/*.zip` and to
`codeload.github.com`, so any build or test of `sdroxide-radio` or
`sdroxide-ui` stops there. `git fetch` of a public repo does work, so build the
archive locally and hand it to the build script through `OPUS_URL` (read by
`crates/sdroxide-rade/build.rs`, which passes it to rade_c's `BuildOpus.cmake`).

1. Read the commit the build wants: `grep -n OPUS_URL vendor/rade_c/cmake/BuildOpus.cmake`
   (today `940d4e5af64351ca8ba8390df3f555484c567fbb`; take it from that file,
   not from here, if it changed).
2. Make the zip. The folder inside it must be a single top-level directory:

   ```bash
   C=940d4e5af64351ca8ba8390df3f555484c567fbb
   mkdir -p /tmp/opus-dl && cd /tmp/opus-dl && git init -q opus && cd opus
   git remote add origin https://github.com/xiph/opus.git
   git fetch --depth 1 origin $C
   git archive --format=zip --prefix=opus-$C/ -o /tmp/opus-dl/opus-$C.zip FETCH_HEAD
   ```

3. Build and test with the variable set (a local path, not a URL):

   ```bash
   export OPUS_URL=/tmp/opus-dl/opus-$C.zip
   cargo check -p sdroxide-radio -p sdroxide-ui
   cargo test -p sdroxide-ui --lib i18n
   ```

Notes: the first build of the UI crates takes several minutes (run it in the
background); Opus's model download from `media.xiph.org` is reachable. Also
needed once: the vendor submodules (`git submodule update --init --depth 1`) and
the system libraries (alsa, soapysdr, udev, cmake, clang) via apt.

**Disk:** a full `cargo test` links hundreds of large binaries and fills the
session disk (linking then fails with `linking with cc failed` or a rustc
panic). Run tests one `--test <name>` at a time and delete the binaries after
(`find target/debug/deps -maxdepth 1 -type f -name '<name>-*' -perm -u+x -delete`),
or delete `target/debug/incremental`.
