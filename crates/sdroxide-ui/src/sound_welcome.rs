//! The web client's welcome message for the sound.
//!
//! A browser plays nothing until the page has been touched, so a fresh
//! session, or one reloaded with F5, is silent, and a beginner reads that as a
//! broken program. `audio_bridge.js` puts a message in the middle of the
//! screen when the page opens; while it is up, the first click, tap or key
//! starts the sound and does nothing else.
//!
//! The page draws the message before the program has loaded, so it cannot
//! wait for this setting. [`sync`] hands the setting to the page whenever it
//! changes: switched off, the message goes at once, and the page remembers
//! the answer on this device so the next opening does not show it either.
//! The setting itself is [`UiSettings::sound_welcome`], kept with the
//! profile's screen on the server; the page's copy only spares a flash
//! before the server's answer arrives.
//!
//! [`UiSettings::sound_welcome`]: sdroxide_types::UiSettings::sound_welcome

/// Whether the setting does anything here, for the settings row to say.
pub fn available() -> bool {
    cfg!(target_arch = "wasm32")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn sync(_want: bool) {}

#[cfg(target_arch = "wasm32")]
pub fn sync(want: bool) {
    use std::cell::Cell;
    use wasm_bindgen::{JsCast, JsValue};

    thread_local! {
        static SENT: Cell<Option<bool>> = const { Cell::new(None) };
    }
    if SENT.with(|s| s.get()) == Some(want) {
        return;
    }
    let Some(window) = web_sys::window() else { return };
    let get = |t: &JsValue, k: &str| {
        js_sys::Reflect::get(t, &JsValue::from_str(k)).unwrap_or(JsValue::UNDEFINED)
    };
    let bridge = get(&window, "sdroxideAudio");
    // A page holding a cached bridge from before this existed: nothing to
    // tell, and nothing shown that could be wrong either way.
    let Ok(f) = get(&bridge, "setSoundWelcome").dyn_into::<js_sys::Function>() else {
        SENT.with(|s| s.set(Some(want)));
        return;
    };
    if f.call1(&bridge, &JsValue::from_bool(want)).is_ok() {
        SENT.with(|s| s.set(Some(want)));
    }
}
