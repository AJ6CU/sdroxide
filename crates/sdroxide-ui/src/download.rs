//! Save a file from the UI. Native pops a "Save As" dialog; wasm triggers a
//! browser download via a Blob + anchor click.

use crate::i18n::tr;

/// The MIME type a browser download is labelled with. Native ignores it — the
/// name and the bytes are all a filesystem needs — but a browser hands the file
/// to whatever the type says, so a picture saved as `text/plain` opens in a text
/// editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mime {
    Text,
    Png,
}

impl Mime {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))] // only the browser labels a download
    fn as_str(self) -> &'static str {
        match self {
            Mime::Text => "text/plain",
            Mime::Png => "image/png",
        }
    }
}

/// Save `data` under a suggested `name`, as a text file.
pub fn save(name: &str, data: &[u8]) {
    save_as(name, data, Mime::Text);
}

/// Save `data` under a suggested `name`, labelled as `mime`.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_as(name: &str, data: &[u8], _mime: Mime) {
    let data = data.to_vec();
    let name = name.to_string();
    // rfd's dialog is blocking; run it off the UI thread.
    std::thread::Builder::new()
        .name("sdroxide-save".into())
        .spawn(move || {
            if let Some(path) = rfd::FileDialog::new().set_file_name(&name).save_file() {
                if let Err(e) = std::fs::write(&path, &data) {
                    eprintln!("sdroxide: saving {}: {e}", path.display());
                }
            }
        })
        .ok();
}

/// A text file as it was read.
pub struct Loaded {
    pub text: String,
    /// The code page the bytes had to be *guessed* to be in, when the file did
    /// not say and was not UTF-8. `None` when nothing was guessed. Worth showing
    /// the operator: it is the one part of the read that could be wrong.
    pub assumed: Option<&'static str>,
}

/// Where a [`load_text`] pick is delivered: the file it read, or why it could
/// not be read. Stays `None` when the operator cancels the dialog.
pub type LoadInbox = std::sync::Arc<std::sync::Mutex<Option<Result<Loaded, String>>>>;

/// Open a text file via a native "Open" dialog (off the UI thread) and store
/// its contents into `inbox` for the UI to pick up next frame. Native opens a
/// filesystem picker; the browser opens its own file input (see the wasm arm).
/// `exts` are the file extensions offered, without the dot — every spelling a
/// format goes by, since a filter that knows only `.adi` hides an `.adif`.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_text(filter_name: &str, exts: &[&str], inbox: LoadInbox) {
    let filter_name = filter_name.to_string();
    let exts: Vec<String> = exts.iter().map(|e| e.to_string()).collect();
    std::thread::Builder::new()
        .name("sdroxide-open".into())
        .spawn(move || {
            let Some(path) = rfd::FileDialog::new().add_filter(&filter_name, &exts).pick_file()
            else {
                return;
            };
            // Read bytes, not a `String`: `read_to_string` refuses a file that
            // is not valid UTF-8 outright, and an operator's log exported by a
            // Windows logger very often is not. Refusing it lost the whole
            // import — every callsign, date and band in it, all of them plain
            // ASCII — over a code page in one name field.
            let outcome = match std::fs::read(&path) {
                Ok(bytes) => Ok(decode_text(&bytes)),
                Err(e) => Err(tr!(
                    "download-reading",
                    path = path.display().to_string(),
                    error = e.to_string()
                )),
            };
            if let Ok(mut g) = inbox.lock() {
                *g = Some(outcome);
            }
        })
        .ok();
}

/// Decode the bytes of a text file, saying what — if anything — had to be
/// assumed to do it.
///
/// A byte-order mark settles the question outright, and so does the file simply
/// being valid UTF-8: nothing else looks like UTF-8 by accident for more than a
/// few bytes. What is left is a legacy single-byte code page, which no file
/// declares and no reader can be certain of, so one is picked by
/// [`looks_cyrillic`] and named in the return so the operator can see the guess.
///
/// Anything is better than nothing here. The fields that matter most in an ADIF
/// log — callsign, date, band, mode, frequency — are ASCII under every one of
/// these encodings, so even a wrongly guessed code page costs at worst the
/// spelling of a name, where refusing the file costs the whole log.
///
/// Both platforms run the same chain: a native pick reads bytes off disk, a
/// browser pick reads the same bytes off a `FileReader`.
pub fn decode_text(bytes: &[u8]) -> Loaded {
    let unicode = |text: String| Loaded { text, assumed: None };
    if let Some(rest) = bytes.strip_prefix(&[0xef, 0xbb, 0xbf]) {
        return unicode(String::from_utf8_lossy(rest).into_owned());
    }
    if let Some(rest) = bytes.strip_prefix(&[0xff, 0xfe]) {
        return unicode(from_utf16(rest, u16::from_le_bytes));
    }
    if let Some(rest) = bytes.strip_prefix(&[0xfe, 0xff]) {
        return unicode(from_utf16(rest, u16::from_be_bytes));
    }
    if let Ok(text) = std::str::from_utf8(bytes) {
        return unicode(text.to_string());
    }
    let (name, table): (_, fn(u8) -> char) = if looks_cyrillic(bytes) {
        ("Windows-1251", sdroxide_types::text::cp1251_char)
    } else {
        ("Windows-1252", sdroxide_types::text::cp1252_char)
    };
    Loaded { text: bytes.iter().map(|&b| table(b)).collect(), assumed: Some(name) }
}

/// UTF-16 code units in the order `pair` reads them, surrogates paired up. A
/// lone surrogate or a trailing odd byte is a truncated file, not a reason to
/// throw the rest away.
fn from_utf16(bytes: &[u8], pair: fn([u8; 2]) -> u16) -> String {
    let units = bytes.chunks_exact(2).map(|c| pair([c[0], c[1]]));
    char::decode_utf16(units).map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER)).collect()
}

/// Whether a file that is not Unicode reads better as Cyrillic than as Western
/// European.
///
/// The two code pages overlap byte for byte — 0xE0 is `à` in one and `а` in the
/// other — so no single value can tell them apart. The shape of the text can:
/// Cyrillic spells a whole word out of high bytes, so its runs are as long as
/// its words, while Western European text is ASCII with an accent dropped into
/// it here and there. Three high bytes in a row is a word in one and a spelling
/// nobody uses in the other, so that is where the line is drawn. Two is not
/// enough — Finnish and Estonian really do write `ää`.
fn looks_cyrillic(bytes: &[u8]) -> bool {
    let (mut run, mut longest) = (0usize, 0usize);
    for &b in bytes {
        run = if b >= 0x80 { run + 1 } else { 0 };
        longest = longest.max(run);
    }
    longest >= 3
}

#[cfg(target_arch = "wasm32")]
/// Open a text file through the browser's own picker and store its contents
/// into `inbox` for the UI to pick up next frame — the missing half of issue
/// #445, where the logbook's IMPORT sat native-only while the browser client
/// could still *write* files.
///
/// A hidden `<input type="file">` does the picking — the only filesystem a
/// page is allowed — and a `FileReader` hands back the raw bytes, which go
/// through the same [`decode_text`] a native open uses, so a Web 1.0 log in
/// Windows-1251 imports identically in both. The one picker element lives for
/// the life of the page; opening the dialog again just re-arms its handler,
/// and resetting the input's value each time keeps the *same* file pickable
/// twice (a change event only fires when the selection actually changes).
///
/// The handlers are kept beside the element rather than forgotten. A
/// `Closure::forget` is never freed, so forgetting the change handler leaked
/// one per click, and forgetting both reader handlers leaked whichever of the
/// two never ran. Kept here, each pick's handlers replace the last pick's,
/// which drops those — and a read still in flight when the next pick lands is
/// aborted first, so no callback is left pointing at a handler that has gone.
pub fn load_text(_filter_name: &str, exts: &[&str], inbox: LoadInbox) {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    /// What one pick's read leaves running: the reader and its two handlers.
    type Reading = (web_sys::FileReader, Closure<dyn FnMut()>, Closure<dyn FnMut()>);

    // A file input, kept between calls, with the change handler it is armed
    // with and the read the last pick started. `HtmlInputElement` is not
    // `Sync`, so a `thread_local` instead of a `static`.
    thread_local! {
        static PICKER: std::cell::RefCell<Option<web_sys::HtmlInputElement>> =
            const { std::cell::RefCell::new(None) };
        static ON_CHANGE: std::cell::RefCell<Option<Closure<dyn FnMut()>>> =
            const { std::cell::RefCell::new(None) };
        static READING: std::cell::RefCell<Option<Reading>> =
            const { std::cell::RefCell::new(None) };
    }
    let Some(window) = web_sys::window() else { return };
    let Some(doc) = window.document() else { return };
    let Some(body) = doc.body() else { return };

    // Getting a picker to exist: create it once, or clone the one we have.
    let input = {
        let ready = PICKER.with(|slot| slot.borrow().clone());
        match ready {
            Some(el) => el,
            None => {
                let Ok(el) = doc
                    .create_element("input")
                    .map(|e| e.unchecked_into::<web_sys::HtmlInputElement>())
                else {
                    return;
                };
                let _ = el.set_attribute("type", "file");
                // Invisible, but still in the DOM so it can be told to click.
                let _ = el.set_attribute("style", "display: none;");
                let _ = body.append_child(el.as_ref());
                PICKER.with(|slot| *slot.borrow_mut() = Some(el.clone()));
                el
            }
        }
    };

    let accept: Vec<String> = exts.iter().map(|e| format!(".{e}")).collect();
    input.set_accept(&accept.join(","));
    // The selection can be made again even if it is the same file as last
    // time: clearing the value makes a re-pick a change again.
    input.set_value("");

    let for_change = input.clone();
    let on_change = Closure::<dyn FnMut()>::new(move || {
        let Some(file) = for_change.files().and_then(|list| list.get(0)) else { return };
        let Ok(reader) = web_sys::FileReader::new() else { return };
        let read_here = reader.clone();
        let ok_inbox = inbox.clone();
        let on_load = Closure::<dyn FnMut()>::new(move || {
            let loaded = read_here.result().ok().map(|v| {
                // An ArrayBuffer, read as its bytes and decoded exactly as the
                // native arm decodes bytes off disk.
                let bytes = js_sys::Uint8Array::new(&v).to_vec();
                decode_text(&bytes)
            });
            if let Ok(mut g) = ok_inbox.lock() {
                *g = Some(match loaded {
                    Some(l) => Ok(l),
                    None => Err(tr!("download-browser-could-not-read-file").into()),
                });
            }
        });
        let err_inbox = inbox.clone();
        let on_error = Closure::<dyn FnMut()>::new(move || {
            if let Ok(mut g) = err_inbox.lock() {
                *g = Some(Err(tr!("download-browser-could-not-read-file").into()));
            }
        });
        reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
        reader.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        let _ = reader.read_as_array_buffer(&file);
        // The previous pick's read, if it is somehow still going, is stopped
        // and unhooked before its handlers are dropped with it.
        let previous = READING.with(|slot| slot.borrow_mut().replace((reader, on_load, on_error)));
        if let Some((old, _, _)) = previous {
            old.set_onload(None);
            old.set_onerror(None);
            old.abort();
        }
    });

    // Re-armed rather than accumulated: the handler from the previous pick is
    // unhooked by this and dropped with the slot's old value.
    input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
    ON_CHANGE.with(|slot| *slot.borrow_mut() = Some(on_change));
    let _ = input.click();
}

#[cfg(target_arch = "wasm32")]
pub fn save_as(name: &str, data: &[u8], mime: Mime) {
    use wasm_bindgen::JsCast;

    let array = js_sys::Uint8Array::from(data);
    let parts = js_sys::Array::new();
    parts.push(&array.buffer());
    let opts = web_sys::BlobPropertyBag::new();
    opts.set_type(mime.as_str());
    let blob = match web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts) {
        Ok(b) => b,
        Err(_) => return,
    };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else { return };

    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    if let Ok(a) = doc.create_element("a") {
        let a: web_sys::HtmlAnchorElement = a.unchecked_into();
        a.set_href(&url);
        a.set_download(name);
        a.click();
    }
    let _ = web_sys::Url::revoke_object_url(&url);
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast as _, JsValue};

/// A browser `File` built from bytes, or `None` if the browser cannot make one.
#[cfg(target_arch = "wasm32")]
fn share_file_handle(name: &str, data: &[u8], mime: Mime) -> Option<web_sys::File> {
    // `File::new_with_blob_sequence_and_options` takes *blob parts*, so the
    // bytes go in as one `ArrayBuffer`-bearing part rather than as a finished
    // Blob — which is also why no intermediate Blob is needed.
    let parts = js_sys::Array::new();
    parts.push(&js_sys::Uint8Array::from(data).buffer());
    let opts = web_sys::FilePropertyBag::new();
    opts.set_type(mime.as_str());
    opts.set_last_modified(js_sys::Date::now());
    web_sys::File::new_with_blob_sequence_and_options(&parts, name, &opts).ok()
}

/// The share payload for one file: `{ files: [File], title }`.
///
/// Built once and used by both [`can_share_file`] and [`share_file`], so the
/// question "can this browser share a file?" is asked of exactly the object
/// that would be shared. Asking it of a hand-rolled probe is how a share button
/// ends up offering a sheet that then refuses.
#[cfg(target_arch = "wasm32")]
fn share_payload(name: &str, data: &[u8], mime: Mime) -> Option<js_sys::Object> {
    let file = share_file_handle(name, data, mime)?;
    let files = js_sys::Array::new();
    files.push(&file);
    let payload = js_sys::Object::new();
    js_sys::Reflect::set(&payload, &JsValue::from_str("files"), &files).ok()?;
    let _ = js_sys::Reflect::set(&payload, &JsValue::from_str("title"), &JsValue::from_str(name));
    Some(payload)
}

#[cfg(target_arch = "wasm32")]
fn navigator_property(win: &web_sys::Window, what: &str) -> Option<js_sys::Function> {
    let nav = js_sys::Reflect::get(win.as_ref(), &JsValue::from_str("navigator")).ok()?;
    js_sys::Reflect::get(&nav, &JsValue::from_str(what))
        .ok()
        .filter(|f| f.is_function())
        .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
}

/// Whether this browser can hand a **file** to the operating system's share
/// sheet — which is what makes WhatsApp, Telegram and Signal one tap away
/// instead of a download the operator then has to find and attach.
///
/// The test is `navigator.canShare`, not whether `navigator.share` exists: the
/// API is present in browsers that refuse files, and a share button that offers
/// a sheet which then does nothing is exactly the control this fork does not
/// ship. It is asked of the same payload [`share_file`] would hand over.
#[cfg(target_arch = "wasm32")]
pub fn can_share_file() -> bool {
    let Some(win) = web_sys::window() else { return false };
    // An empty picture: `canShare` is asking about file support, not about this
    // particular PNG, and an empty one keeps the question cheap.
    let Some(payload) = share_payload("share.png", &[], Mime::Png) else { return false };
    let Some(can_share) = navigator_property(&win, "canShare") else { return false };
    can_share.call1(&JsValue::NULL, &payload).ok().and_then(|v| v.as_bool()).unwrap_or(false)
}

/// Hand a picture to the operating system's share sheet.
///
/// **This is the whole of feature 1 on a phone.** A `mailto:` on Android opens a
/// blank message with nothing attached, which is not sharing a picture with a
/// friend — it is three more steps and a filing system. `navigator.share` puts
/// the picture *in* the message, so WhatsApp, Telegram, Signal, Mail and
/// everything else the phone already has appear in the one list the operator
/// uses for everything else.
///
/// Returns whether the share sheet was opened. A browser that refuses — no
/// `File`, no `navigator.share`, or the operator dismissing the sheet — returns
/// `false`, and the caller falls back to saving the picture, which always works.
///
/// **Must be called from the click itself.** A share without a user gesture is
/// refused by every browser, and refused *silently*, so a share started from a
/// timer or a later frame would appear to do nothing at all.
#[cfg(target_arch = "wasm32")]
pub fn share_file(name: &str, data: &[u8], mime: Mime) -> bool {
    let Some(win) = web_sys::window() else { return false };
    let Some(payload) = share_payload(name, data, mime) else { return false };
    let Some(share) = navigator_property(&win, "share") else { return false };
    // The returned promise settles when the sheet closes, which is not
    // something the click needs to wait for; what matters is that the call was
    // accepted, which is what `is_ok` says.
    share.call1(&JsValue::NULL, &payload).is_ok()
}

/// Native: there is no share sheet to hand a file to, so this is always false
/// and the caller saves the picture and opens the mail client instead.
#[cfg(not(target_arch = "wasm32"))]
pub fn can_share_file() -> bool {
    false
}

/// Native counterpart of [`share_file`] — see above.
#[cfg(not(target_arch = "wasm32"))]
pub fn share_file(_name: &str, _data: &[u8], _mime: Mime) -> bool {
    false
}

/// Whether this is a URL worth handing to the desktop's opener, and what to
/// hand it.
///
/// **Split out from [`open_external`] so it can be tested without launching
/// anything.** The first version of that test called `open_external` directly,
/// which shelled out to the real `xdg-open` — so `cargo test` opened a mail
/// window on the operator's desktop, in the middle of a test run. A test that
/// touches the machine it runs on is not a test; the launching half is now
/// exercised by pressing the button and nothing else.
fn external_url_ok(url: &str) -> Option<&str> {
    let url = url.trim();
    // Empty is nothing to open, and a leading `-` would be read as a flag by
    // every one of these openers rather than as a URL.
    if url.is_empty() || url.starts_with('-') {
        return None;
    }
    Some(url)
}

/// Hand a URL to whatever the desktop has registered for it — a mail client, a
/// browser — and say whether it was handed over.
///
/// **This exists because `egui`'s own `open_url` does nothing on the desktop.**
/// `Context::open_url` queues an `OutputCommand`, and eframe implements that
/// command **only in its web target** (`eframe/src/web/app_runner.rs`); the
/// native backend never reads it, so the command is dropped without a sound.
/// That is why a *Share* or *MAIL REPORT* button did nothing at all on a
/// desktop — and why the signal-identification window's sigidwiki links and the
/// manual's own cross-reference links have never opened anything either. It is
/// not a Wayland or compositor problem; there is simply no native handler.
///
/// Returns whether the platform's opener was **launched**. That is the most
/// that can be known here: the handler is another process and whether it showed
/// a window is its own business. So a caller that must not lose the content
/// keeps a copy of it rather than trusting this.
pub fn open_external(url: &str) -> bool {
    let Some(url) = external_url_ok(url) else { return false };
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().is_some_and(|w| w.open_with_url(url).is_ok())
    }
    #[cfg(target_os = "linux")]
    {
        // `xdg-open` rather than `gio open`: it is the one every desktop
        // implements, and on a tiling compositor there is no portal to ask.
        // `gio` is used when present because it resolves the handler the same
        // way but honours a running `dbus`-backed handler faster.
        let launched = std::process::Command::new("xdg-open")
            .arg(url)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .is_ok();
        launched
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn().is_ok()
    }
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "linux"), not(target_os = "macos")))]
    {
        // Windows: `start` is a `cmd` builtin, and the empty pair of quotes is
        // the *window title* — without it a quoted URL becomes the title and
        // nothing opens.
        std::process::Command::new("cmd").args(["/c", "start", ""]).arg(url).spawn().is_ok()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    /// Plain ASCII, and UTF-8 that says so, are read as themselves with nothing
    /// assumed — the case that must not regress into a guess.
    #[test]
    fn unicode_is_read_as_itself() {
        let plain = decode_text(b"<CALL:4>W1AW<EOR>");
        assert_eq!(plain.text, "<CALL:4>W1AW<EOR>");
        assert_eq!(plain.assumed, None);

        let utf8 = decode_text("<NAME:9>Владимир".as_bytes());
        assert_eq!(utf8.text, "<NAME:9>Владимир");
        assert_eq!(utf8.assumed, None);

        // A BOM belongs to the encoding, not to the text: left in, it would be
        // the first character of the first tag.
        let mut bom = vec![0xef, 0xbb, 0xbf];
        bom.extend_from_slice("<CALL:4>W1AW".as_bytes());
        assert_eq!(decode_text(&bom).text, "<CALL:4>W1AW");

        let utf16: Vec<u8> = [0xff, 0xfe]
            .into_iter()
            .chain("<CALL:4>W1AW".encode_utf16().flat_map(|u| u.to_le_bytes()))
            .collect();
        let read = decode_text(&utf16);
        assert_eq!(read.text, "<CALL:4>W1AW");
        assert_eq!(read.assumed, None);
    }

    /// The issue this was written for: a log exported in a national code page
    /// used to import as nothing at all.
    #[test]
    fn a_cyrillic_code_page_still_imports() {
        // "<NAME:8>Владимир <QTH:6>Москва <EOR>" in Windows-1251.
        let mut raw = b"<NAME:8>".to_vec();
        raw.extend_from_slice(&[0xc2, 0xeb, 0xe0, 0xe4, 0xe8, 0xec, 0xe8, 0xf0]);
        raw.extend_from_slice(b" <QTH:6>");
        raw.extend_from_slice(&[0xcc, 0xee, 0xf1, 0xea, 0xe2, 0xe0]);
        raw.extend_from_slice(b" <EOR>");
        let read = decode_text(&raw);
        assert_eq!(read.assumed, Some("Windows-1251"));
        assert_eq!(read.text, "<NAME:8>Владимир <QTH:6>Москва <EOR>");
        // And the declared lengths, counted in the code page's own bytes, still
        // land: the parser reads them as characters once they no longer fit as
        // UTF-8 bytes.
        let recs = sdroxide_types::adif_to_qso_log(&format!("<CALL:5>UA1AB {}", read.text));
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].name, "Владимир");
        assert_eq!(recs[0].qth, "Москва");
    }

    /// The other direction: an accent dropped into ASCII is Western European,
    /// and reading it as Cyrillic would turn every one of them into a letter
    /// from the wrong alphabet.
    #[test]
    fn a_western_code_page_is_not_mistaken_for_cyrillic() {
        // "<NAME:5>Jörg <QTH:9>Jyväskylä <EOR>" in Windows-1252.
        let mut raw = b"<NAME:5>J".to_vec();
        raw.push(0xf6);
        raw.extend_from_slice(b"rg <QTH:9>Jyv");
        raw.push(0xe4);
        raw.extend_from_slice(b"skyl");
        raw.push(0xe4);
        raw.extend_from_slice(b" <EOR>");
        let read = decode_text(&raw);
        assert_eq!(read.assumed, Some("Windows-1252"));
        assert_eq!(read.text, "<NAME:5>Jörg <QTH:9>Jyväskylä <EOR>");
    }
}

#[cfg(test)]
mod open_tests {
    use super::external_url_ok;

    /// The refusal half of handing a URL to the desktop, **with nothing
    /// launched**.
    ///
    /// There is deliberately no test here that calls `open_external`: it shells
    /// out to the machine's real `xdg-open`, and a test run that opened a mail
    /// window on the operator's desktop — which is exactly what the first
    /// version of this did — is not a test. What can be decided without a side
    /// effect is decided and pinned here; the launching half is what pressing
    /// the button is for.
    #[test]
    fn an_empty_or_flag_shaped_url_is_never_handed_over() {
        for bad in ["", "   ", "-e", "--version", " -oProxyCommand=x"] {
            assert!(
                external_url_ok(bad).is_none(),
                "{bad:?} must be refused: it is empty, or a shell opener would \
                 read it as an option"
            );
        }
        // A real URL passes, and comes back trimmed.
        for good in ["mailto:reports@example.org", "https://example.org/a?b=c"] {
            assert_eq!(external_url_ok(good), Some(good));
        }
        assert_eq!(external_url_ok("  https://example.org  "), Some("https://example.org"));
    }
}
