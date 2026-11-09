//! Translations: the interface's text, looked up by message id in Fluent files.
//!
//! The messages live in `i18n/<language>/sdroxide.ftl`, one file per language,
//! embedded in the binary. English (`en-US`) is the reference: every id is
//! defined there first, the [`tr!`] macro checks each id and its arguments
//! against it at compile time, and any message a translation lacks is drawn
//! from it.
//!
//! **The browser build embeds English only.** The other ten files are about
//! 6 MB of text that an operator who reads one language never uses, so the
//! page asks the station for the one it needs (`i18n/<tag>/sdroxide.ftl`, put
//! beside the page by `trunk`) when the language is picked, stored in the
//! profile, or — with none picked — read from the browser's own list. English
//! stays loaded as the fallback, so a translation that is slow, missing or
//! unreadable costs the operator English, never an id on screen. The picker
//! lists [`LANGUAGES`], which is carried in every build because the files it
//! names are not.
//!
//! What is *not* translated here, on purpose:
//!
//! - anything that is written to disk or sent to the engine — a setting's
//!   value, a mode name in a log, a protocol field. Those keep the English
//!   `label()`s in `sdroxide-types`; the screen asks this module for its own
//!   words for them.
//! - the manual behind F1, which is a document of its own, and the spoken
//!   announcements, which are an English-only voice.
//!
//! Two rules every message follows:
//!
//! - **The message carries its final casing.** The interface writes many
//!   captions in capitals, and upper-casing in code is wrong for some
//!   languages (Turkish `i` becomes `İ`, German `ß` stays `ß` or becomes `SS`
//!   depending on the context). A caption that is shown in capitals is
//!   written in capitals in the file.
//! - **No isolation marks.** Fluent wraps every placeable in the Unicode
//!   bidirectional isolates U+2068/U+2069 by default. egui does not lay out
//!   bidirectional text and draws them as boxes, so they are turned off.

use std::borrow::Cow;
use std::sync::LazyLock;

use fluent::FluentValue;
use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::unic_langid::LanguageIdentifier;
use i18n_embed::{I18nAssets, LanguageLoader};
use sdroxide_types::UiLanguage;

mod labels;
pub(crate) use labels::Tr;

/// What is embedded in the binary. In the browser that is English alone: the
/// other languages are fetched (see [`fetched`]).
#[derive(rust_embed::RustEmbed)]
#[folder = "i18n/"]
#[cfg_attr(target_arch = "wasm32", include = "en-US/*")]
struct Localizations;

/// Every language a translation is carried for: its tag, and its name for
/// itself (`language-name` in its file), English first and the rest by tag.
///
/// Listed here rather than found in the embedded files, because the browser
/// build does not embed them. A test compares this with `i18n/`, so a language
/// added there and not here — or the other way round — fails the build.
pub(crate) const LANGUAGES: &[(&str, &str)] = &[
    ("en-US", "English"),
    ("de", "Deutsch"),
    ("es", "Español"),
    ("fr", "Français"),
    ("it", "Italiano"),
    ("ja", "日本語"),
    ("nl", "Nederlands"),
    ("pt-BR", "Português (Brasil)"),
    ("ru", "Русский"),
    ("tr", "Türkçe"),
    ("zh-CN", "简体中文"),
];

/// Where the messages are read from: the files embedded in the binary, and any
/// translation the browser has fetched from the station.
struct Assets;

impl I18nAssets for Assets {
    fn get_files(&self, file_path: &str) -> Vec<Cow<'_, [u8]>> {
        match fetched::get(file_path) {
            Some(bytes) => vec![Cow::Owned(bytes)],
            None => Localizations.get_files(file_path),
        }
    }

    fn filenames_iter(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(Localizations.filenames_iter().chain(fetched::names()))
    }
}

/// Translations fetched from the station. Only the browser has any; elsewhere
/// every language is embedded and this holds nothing.
#[cfg(not(target_arch = "wasm32"))]
mod fetched {
    pub(super) fn get(_path: &str) -> Option<Vec<u8>> {
        None
    }
    pub(super) fn names() -> std::vec::IntoIter<String> {
        Vec::new().into_iter()
    }
}

#[cfg(target_arch = "wasm32")]
mod fetched {
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::Mutex;

    use i18n_embed::unic_langid::LanguageIdentifier;
    use sdroxide_types::UiLanguage;

    /// `<tag>/sdroxide.ftl` → the file's bytes.
    static FILES: Mutex<BTreeMap<String, Vec<u8>>> = Mutex::new(BTreeMap::new());
    /// Tags being fetched now, so asking again while one is on its way does
    /// not send a second request.
    static IN_FLIGHT: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

    fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
        m.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(super) fn get(path: &str) -> Option<Vec<u8>> {
        lock(&FILES).get(path).cloned()
    }

    pub(super) fn names() -> std::vec::IntoIter<String> {
        lock(&FILES).keys().cloned().collect::<Vec<_>>().into_iter()
    }

    /// The carried language `requested` comes to, matched exactly and then on
    /// the language alone (a Mexican browser gets `es`) — or `None` when it is
    /// English, which is already here.
    fn negotiate(requested: &[LanguageIdentifier]) -> Option<&'static str> {
        let tags = || {
            super::LANGUAGES.iter().filter_map(|(tag, _)| Some((*tag, tag.parse::<LanguageIdentifier>().ok()?)))
        };
        for want in requested {
            let hit = tags()
                .find(|(_, id)| id == want)
                .or_else(|| tags().find(|(_, id)| id.language == want.language));
            if let Some((tag, _)) = hit {
                return (tag != "en-US").then_some(tag);
            }
        }
        None
    }

    /// Start the fetch of the translation `requested` comes to, when it is not
    /// here yet. `true` means the language is on its way and the caller should
    /// leave the screen as it is until it arrives.
    pub(super) fn wanted(
        requested: &[LanguageIdentifier],
        language: UiLanguage,
        ctx: &eframe::egui::Context,
    ) -> bool {
        let Some(tag) = negotiate(requested) else { return false };
        if lock(&FILES).contains_key(&format!("{tag}/sdroxide.ftl")) {
            return false;
        }
        if lock(&IN_FLIGHT).insert(tag.to_string()) {
            let ctx = ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch(tag).await {
                    Ok(text) => {
                        lock(&FILES).insert(format!("{tag}/sdroxide.ftl"), text.into_bytes());
                        lock(&IN_FLIGHT).remove(tag);
                        // Only if it is still the language asked for: the
                        // operator may have picked another while this came.
                        super::apply_if_current(language, &ctx);
                    }
                    Err(why) => {
                        lock(&IN_FLIGHT).remove(tag);
                        tracing::warn!("could not fetch the {tag} translation: {why}");
                    }
                }
            });
        }
        true
    }

    /// One translation file, from beside the page. Revalidated by the browser
    /// on every load (the station marks it `no-cache`), so an update of the
    /// station is never read against an older file.
    async fn fetch(tag: &str) -> Result<String, String> {
        use wasm_bindgen::JsCast;
        let window = web_sys::window().ok_or("no window")?;
        let init = web_sys::RequestInit::new();
        init.set_method("GET");
        init.set_mode(web_sys::RequestMode::SameOrigin);
        let request = web_sys::Request::new_with_str_and_init(&format!("i18n/{tag}/sdroxide.ftl"), &init)
            .map_err(|_| "bad address")?;
        let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| "no answer")?
            .dyn_into::<web_sys::Response>()
            .map_err(|_| "not a response")?;
        if response.status() != 200 {
            return Err(format!("status {}", response.status()));
        }
        let text = wasm_bindgen_futures::JsFuture::from(response.text().map_err(|_| "unreadable")?)
            .await
            .ok()
            .and_then(|t| t.as_string())
            .ok_or("unreadable")?;
        // A page that answers every address with itself would be taken for a
        // translation: it has to name its language.
        if !text.contains("language-name") {
            return Err("not a translation file".into());
        }
        Ok(text)
    }
}

/// The one loader every message is drawn from. Starts in English; the
/// language is chosen by [`set_language`] before the first frame.
pub(crate) static LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();
    // The English file is checked at compile time by every `tr!`, so failing
    // to load it means the embedding itself is broken.
    loader.load_fallback_language(&Assets).expect("the English messages are embedded");
    loader.set_use_isolating(false);
    loader
});

/// A translated message, by id: `tr!("settings-ui-theme")`, or with
/// arguments, `tr!("fps", fps = 60)`. An attribute is named after the id:
/// `tr!("settings-ui-theme", "hint")` reads `.hint` off that message.
///
/// The id and every argument are checked against the English file when the
/// crate is compiled.
macro_rules! tr {
    ($id:literal $(, $attr:literal)?) => {
        i18n_embed_fl::fl!($crate::i18n::LOADER, $id $(, $attr)?)
    };
    // Each argument is bound before it is handed over, so `n = count + 1`
    // has its type settled by the arithmetic rather than left for the
    // conversion into a Fluent value to guess.
    //
    // Borrowed, like `format!` borrows: `name = entry.name` does not move the
    // name out of the entry. See [`Arg`].
    ($id:literal $(, $attr:literal)?, $($name:ident = $value:expr),+ $(,)?) => {
        match ($($crate::i18n::arg(&$value),)+) {
            ($($name,)+) => {
                i18n_embed_fl::fl!($crate::i18n::LOADER, $id $(, $attr)?, $($name = $name),+)
            }
        }
    };
}
pub(crate) use tr;

/// A value a message can be given: text, or a number (which Fluent can
/// choose a plural form by). Implemented for references too, so [`tr!`] can
/// take every argument by reference and never move it.
pub(crate) trait Arg {
    fn fluent(&self) -> FluentValue<'_>;
}

impl Arg for str {
    fn fluent(&self) -> FluentValue<'_> {
        FluentValue::from(self)
    }
}

impl Arg for String {
    fn fluent(&self) -> FluentValue<'_> {
        FluentValue::from(self.as_str())
    }
}

impl Arg for Cow<'_, str> {
    fn fluent(&self) -> FluentValue<'_> {
        FluentValue::from(self.as_ref())
    }
}

impl<T: Arg + ?Sized> Arg for &T {
    fn fluent(&self) -> FluentValue<'_> {
        (**self).fluent()
    }
}

macro_rules! number_arg {
    ($($t:ty)*) => {$(
        impl Arg for $t {
            fn fluent(&self) -> FluentValue<'_> {
                FluentValue::from(*self)
            }
        }
    )*};
}
number_arg!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);

impl Arg for char {
    fn fluent(&self) -> FluentValue<'_> {
        FluentValue::from(self.to_string())
    }
}

/// The [`Arg`] behind a reference, for [`tr!`].
pub(crate) fn arg<T: Arg + ?Sized>(value: &T) -> FluentValue<'_> {
    value.fluent()
}

/// The tag picked to draw every message as an accented, lengthened English —
/// a layout test: text that is not going through [`tr!`] stays plain and
/// stands out, and a label that does not leave room for a longer language
/// clips. Never shown to an operator in a release build.
pub(crate) const PSEUDO_TAG: &str = "en-XA";

/// Draw the interface in `language` from the next frame on. Cheap to call
/// with the language already showing — it does nothing.
///
/// In the browser the translation may have to be fetched first: the screen
/// stays as it is until it arrives, and `ctx` is asked to repaint then.
pub(crate) fn set_language(language: UiLanguage, ctx: &eframe::egui::Context) {
    {
        let mut applied = APPLIED.lock().unwrap_or_else(|e| e.into_inner());
        if *applied == Some(language) {
            return;
        }
        *applied = Some(language);
    }
    apply(language, ctx);
}

/// The language last asked for, so a fetch that comes back late can tell
/// whether it is still wanted.
static APPLIED: std::sync::Mutex<Option<UiLanguage>> = std::sync::Mutex::new(None);

/// A translation has just arrived: draw in it, unless the operator has asked
/// for another language since.
#[cfg(target_arch = "wasm32")]
fn apply_if_current(language: UiLanguage, ctx: &eframe::egui::Context) {
    let current = *APPLIED.lock().unwrap_or_else(|e| e.into_inner()) == Some(language);
    if current {
        apply(language, ctx);
        ctx.request_repaint();
    }
}

fn apply(language: UiLanguage, ctx: &eframe::egui::Context) {
    let _ = ctx; // only the browser build has to wait for a fetch
    let pseudo = language.tag() == Some(PSEUDO_TAG);
    let requested: Vec<LanguageIdentifier> = match language.tag() {
        Some(PSEUDO_TAG) => Vec::new(),
        Some(tag) => tag.parse().into_iter().collect(),
        None => system_languages(),
    };
    // In the browser the language may not be here yet: ask the station for it
    // and keep drawing what is showing until it comes.
    #[cfg(target_arch = "wasm32")]
    if fetched::wanted(&requested, language, ctx) {
        return;
    }
    // The negotiation always ends at English, so there is always something to
    // load; an error here is a broken embedding, and the loader keeps the
    // languages it had.
    if let Err(e) = i18n_embed::select(&*LOADER, &Assets, &requested) {
        tracing::warn!("could not load the interface language {language:?}: {e}");
    }
    // Both are set on the bundles the selection just built, so they have to
    // be set again on every change.
    LOADER.set_use_isolating(false);
    LOADER.with_bundles_mut(|bundle| bundle.set_transform(pseudo.then_some(pseudo_text as _)));
}

/// The languages the operating system — or the browser — says the operator
/// reads, most preferred first.
fn system_languages() -> Vec<LanguageIdentifier> {
    // The unit tests assert on the English text; they must not depend on the
    // language of the machine that runs them.
    #[cfg(test)]
    if std::env::var_os("SDROXIDE_TEST_SYSTEM_LANG").is_none() {
        return Vec::new();
    }
    sys_locale::get_locales().filter_map(|l| locale_tag(&l)).collect()
}

/// A system locale name as the language tag it stands for. POSIX ones arrive
/// as `es_ES.UTF-8` or `de_DE@euro`; the tag is the part before the dot or the
/// at, with a hyphen for the underscore.
fn locale_tag(locale: &str) -> Option<LanguageIdentifier> {
    locale.split(['.', '@']).next()?.replace('_', "-").parse().ok()
}

/// The name of the language "follow the system" comes to: the first of the
/// system's languages a translation is carried for, matched on the language
/// alone when the region differs (a Mexican system gets `es`), and English
/// when none is.
pub(crate) fn system_language_name() -> &'static str {
    static NAME: LazyLock<String> = LazyLock::new(|| {
        let list = languages();
        let found = system_languages().into_iter().find_map(|want| {
            let tags =
                || list.iter().filter_map(|l| Some((l, l.tag.parse::<LanguageIdentifier>().ok()?)));
            tags()
                .find(|(_, id)| *id == want)
                .or_else(|| tags().find(|(_, id)| id.language == want.language))
                .map(|(l, _)| l.name.clone())
        });
        found.unwrap_or_else(|| list[0].name.clone())
    });
    &NAME
}

/// One language the interface can be drawn in, as the language picker lists it.
pub(crate) struct Language {
    /// What is stored in the settings when it is picked.
    pub tag: String,
    /// The language's name for itself — `Español`, `Deutsch` — so an
    /// operator who cannot read the language the interface is in can still
    /// find their own.
    pub name: String,
}

/// Every language a translation is carried for, English first and the rest
/// by tag. In a debug build the pseudo-language is at the end.
pub(crate) fn languages() -> &'static [Language] {
    static LIST: LazyLock<Vec<Language>> = LazyLock::new(|| {
        let mut list: Vec<Language> = LANGUAGES
            .iter()
            .map(|(tag, name)| Language { tag: (*tag).into(), name: (*name).into() })
            .collect();
        if cfg!(debug_assertions) {
            list.push(Language {
                tag: PSEUDO_TAG.into(), name: "Ƥşḗŭḓǿ (layout test)".into()
            });
        }
        list
    });
    &LIST
}

/// The pseudo-language: every letter accented and every run of text a third
/// longer, inside brackets, so a clipped end shows. Applied by Fluent to the
/// message's own text only, never to what is filled into it, so a callsign or
/// a frequency in a message stays readable.
fn pseudo_text(text: &str) -> Cow<'_, str> {
    if text.trim().is_empty() {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len() * 2);
    out.push('⟦');
    let mut letters = 0usize;
    for c in text.chars() {
        out.push(accented(c));
        letters += c.is_alphabetic() as usize;
    }
    for _ in 0..letters.div_ceil(3) {
        out.push('~');
    }
    out.push('⟧');
    Cow::Owned(out)
}

fn accented(c: char) -> char {
    const FROM: &str = "aeiouyAEIOUYcnszCNSZgdlrtGDLRT";
    const TO: &str = "ȧḗīǿŭẏȦḖĪǾŬẎçñşẑÇÑŞẐğḓŀřŧĞḒĿŘŦ";
    FROM.chars().position(|f| f == c).and_then(|i| TO.chars().nth(i)).unwrap_or(c)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use i18n_embed::fluent::FluentLanguageLoader;
    use i18n_embed::{I18nAssets, LanguageLoader};

    use super::*;

    /// The message and attribute ids a file defines, each with the variables
    /// its text uses — parsed strictly, so a syntax error fails the test.
    fn messages(lang: &str) -> BTreeMap<String, BTreeSet<String>> {
        use fluent_syntax::ast;
        let path = format!("{lang}/sdroxide.ftl");
        let file = Localizations.get_files(&path).pop().expect("the file is embedded");
        let text = String::from_utf8(file.to_vec()).expect("the file is UTF-8");
        let res = match fluent_syntax::parser::parse(text.as_str()) {
            Ok(res) => res,
            Err((_, errors)) => panic!("{path} does not parse: {errors:#?}"),
        };

        fn vars(p: &ast::Pattern<&str>, out: &mut BTreeSet<String>) {
            for el in &p.elements {
                if let ast::PatternElement::Placeable { expression } = el {
                    expr(expression, out);
                }
            }
        }
        fn expr(e: &ast::Expression<&str>, out: &mut BTreeSet<String>) {
            match e {
                ast::Expression::Select { selector, variants } => {
                    inline(selector, out);
                    for v in variants {
                        vars(&v.value, out);
                    }
                }
                ast::Expression::Inline(i) => inline(i, out),
            }
        }
        fn inline(i: &ast::InlineExpression<&str>, out: &mut BTreeSet<String>) {
            match i {
                ast::InlineExpression::VariableReference { id } => {
                    out.insert(id.name.to_string());
                }
                ast::InlineExpression::FunctionReference { arguments, .. } => {
                    for a in &arguments.positional {
                        inline(a, out);
                    }
                }
                ast::InlineExpression::Placeable { expression } => expr(expression, out),
                _ => {}
            }
        }

        let mut out = BTreeMap::new();
        for entry in res.body {
            if let ast::Entry::Message(m) = entry {
                let id = m.id.name.to_string();
                // Fluent keeps the first of two definitions and drops the
                // second without a word, so a clash is a silent wrong text.
                assert!(!out.contains_key(&id), "{path} defines {id} twice");
                let mut v = BTreeSet::new();
                if let Some(p) = &m.value {
                    vars(p, &mut v);
                }
                out.insert(id.clone(), v);
                for a in &m.attributes {
                    let mut v = BTreeSet::new();
                    vars(&a.value, &mut v);
                    out.insert(format!("{id}.{}", a.id.name), v);
                }
            }
        }
        out
    }

    fn translations() -> Vec<String> {
        let english = LOADER.fallback_language().to_string();
        LOADER
            .available_languages(&Localizations)
            .unwrap()
            .into_iter()
            .map(|l| l.to_string())
            .filter(|l| *l != english)
            .collect()
    }

    /// A translation may leave messages out — English stands in for them — but
    /// it may not define one English does not have, which is a renamed or
    /// deleted message nobody will ever see, and it may not use a variable the
    /// English does not pass, which would be drawn as its own name.
    #[test]
    fn every_translation_matches_the_english_messages() {
        let english = messages("en-US");
        assert!(english.contains_key("language-name"));
        for lang in translations() {
            let theirs = messages(&lang);
            assert!(theirs.contains_key("language-name"), "{lang} names itself");
            for (id, vars) in &theirs {
                let Some(want) = english.get(id) else {
                    panic!("{lang} defines {id}, which English does not");
                };
                assert!(
                    vars.is_subset(want),
                    "{lang} {id} uses {vars:?}; English only passes {want:?}"
                );
            }
        }
    }

    /// Every English message is asked for somewhere, so a translator is never
    /// given text nobody will see. A message is used when its id appears as a
    /// string in the source — through `tr!` or otherwise.
    #[test]
    fn every_english_message_is_used() {
        fn sources(dir: &std::path::Path, out: &mut String) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    sources(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    out.push_str(&std::fs::read_to_string(&path).unwrap());
                }
            }
        }
        let mut src = String::new();
        sources(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut src);
        let unused: Vec<_> = messages("en-US")
            .into_keys()
            .filter(|id| !id.contains('.') && id != "language-name")
            .filter(|id| !src.contains(&format!("\"{id}\"")))
            .collect();
        assert!(unused.is_empty(), "messages nothing asks for: {unused:?}");
    }

    /// [`LANGUAGES`] is what the picker lists and what the browser may ask the
    /// station for, and the browser build embeds none of the files it names —
    /// so it has to be the languages in `i18n/`, each under the name its own
    /// file gives itself. A language added to one and not the other would be
    /// missing from the picker, or offered and never found.
    #[test]
    fn the_language_index_matches_the_files() {
        let mut on_disk: Vec<String> =
            std::fs::read_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("i18n"))
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect();
        on_disk.sort();
        let mut listed: Vec<&str> = LANGUAGES.iter().map(|(tag, _)| *tag).collect();
        listed.sort();
        assert_eq!(listed, on_disk, "LANGUAGES and i18n/ name different languages");
        assert_eq!(LANGUAGES[0].0, "en-US", "English comes first");

        for (tag, name) in LANGUAGES {
            let one = FluentLanguageLoader::new(LOADER.domain(), tag.parse().unwrap());
            one.load_fallback_language(&Localizations).unwrap();
            assert_eq!(one.get("language-name"), *name, "{tag} names itself differently");
        }
    }

    /// Every message formats without the bidirectional isolates egui would
    /// draw as boxes, in every language, after a language change too.
    #[test]
    fn no_message_carries_isolation_marks() {
        for lang in std::iter::once("en-US".to_string()).chain(translations()) {
            let one = FluentLanguageLoader::new(LOADER.domain(), lang.parse().unwrap());
            one.load_fallback_language(&Localizations).unwrap();
            one.set_use_isolating(false);
            for id in messages(&lang).keys().filter(|id| !id.contains('.')) {
                let mut args = std::collections::HashMap::new();
                args.insert("n", "1");
                let text = one.get_args(id, args);
                assert!(
                    !text.contains(['\u{2068}', '\u{2069}']),
                    "{lang} {id} carries isolation marks: {text:?}"
                );
            }
        }
    }

    /// The language picker finds English first, and every name is a name
    /// rather than the id it is looked up by.
    #[test]
    fn the_picker_lists_english_first() {
        let list = languages();
        assert_eq!(list[0].tag, "en-US");
        assert_eq!(list[0].name, "English");
        assert!(list.iter().all(|l| !l.name.is_empty() && l.name != "language-name"));
        assert!(
            list.iter().any(|l| l.tag == "es" && l.name == "Español"),
            "{:?}",
            list.iter().map(|l| &l.tag).collect::<Vec<_>>()
        );
    }

    /// The pseudo-language stretches and accents the text it is given and
    /// leaves blank runs alone, so spacing between placeables is unchanged.
    #[test]
    fn the_pseudo_language_is_longer_and_accented() {
        let out = pseudo_text("Theme");
        assert_eq!(out, "⟦Ŧhḗmḗ~~⟧");
        assert!(out.chars().count() > "Theme".chars().count());
        assert_eq!(pseudo_text("  "), "  ");
    }

    /// POSIX locale names are read as the language tags they stand for.
    #[test]
    fn posix_locales_read_as_tags() {
        let parse = locale_tag;
        assert_eq!(parse("es_ES.UTF-8").unwrap().to_string(), "es-ES");
        assert_eq!(parse("de_DE@euro").unwrap().to_string(), "de-DE");
        assert_eq!(parse("pt-BR").unwrap().to_string(), "pt-BR");
    }
}
