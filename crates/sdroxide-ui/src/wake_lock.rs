//! Keep the screen on: the browser's Screen Wake Lock, for a phone or tablet
//! left on the waterfall.
//!
//! A phone dims and locks after a minute of no touching, and a listener
//! watching the waterfall — or a station left decoding on a tablet, the anti-
//! touch padlock closed — touches nothing. The Screen Wake Lock API asks the
//! browser to keep the screen lit while the page is in front.
//!
//! The browser takes the lock back by itself whenever the page is hidden — a
//! tab switch, the phone locked by its own button — so it is not asked for
//! once: [`keep_awake`] runs every frame and asks again when the page is back
//! in front and the lock is gone. It needs a secure page (https, or localhost);
//! where the browser has no such API, or refuses, [`status`] says so, so the
//! setting never silently does nothing.
//!
//! The installed desktop program has no equivalent here: its screen follows
//! the system's own power settings, and [`status`] says that too.

use crate::i18n::tr;

/// Where the lock stands, for the settings row to say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Each target builds a different subset: the browser never says `Native`,
// the installed program never says anything else.
#[allow(dead_code)]
pub enum WakeStatus {
    /// Not asked for.
    Off,
    /// Held: the screen stays on while this page is in front.
    Held,
    /// Asked for and not answered yet, or waiting for the page to be in front.
    Asking,
    /// This browser has no Screen Wake Lock, or the page is not secure.
    Unsupported,
    /// The browser refused (power saving, or a permission policy).
    Refused,
    /// The installed program: the system's power settings decide.
    Native,
}

impl WakeStatus {
    /// The few words shown beside the setting.
    pub fn label(self) -> String {
        match self {
            Self::Off => String::new(),
            Self::Held => tr!("wake-held"),
            Self::Asking => tr!("wake-asking"),
            Self::Unsupported => tr!("wake-unsupported"),
            Self::Refused => tr!("wake-refused"),
            Self::Native => tr!("wake-native"),
        }
    }

    /// Whether the label is a problem worth showing in the warning colour.
    pub fn is_problem(self) -> bool {
        matches!(self, Self::Unsupported | Self::Refused | Self::Native)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn keep_awake(_want: bool, _now: f64) {}

#[cfg(not(target_arch = "wasm32"))]
pub fn status(want: bool) -> WakeStatus {
    if want { WakeStatus::Native } else { WakeStatus::Off }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::WakeStatus;
    use std::cell::RefCell;
    use wasm_bindgen::{JsCast, JsValue};

    /// How long to wait before asking again after a refusal, so a browser that
    /// says no is not asked sixty times a second.
    const RETRY_AFTER_REFUSAL_S: f64 = 5.0;

    struct Lock {
        sentinel: Option<JsValue>,
        pending: bool,
        status: WakeStatus,
        last_ask: f64,
    }

    thread_local! {
        static LOCK: RefCell<Lock> = const {
            RefCell::new(Lock { sentinel: None, pending: false, status: WakeStatus::Off, last_ask: f64::NEG_INFINITY })
        };
    }

    fn get(target: &JsValue, key: &str) -> JsValue {
        js_sys::Reflect::get(target, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED)
    }

    fn call0(target: &JsValue, method: &str) -> Option<JsValue> {
        get(target, method).dyn_into::<js_sys::Function>().ok()?.call0(target).ok()
    }

    fn page_visible() -> bool {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return false };
        get(&doc, "visibilityState").as_string().is_none_or(|s| s == "visible")
    }

    pub fn keep_awake(want: bool, now: f64) {
        LOCK.with(|l| {
            let mut l = l.borrow_mut();
            if !want {
                if let Some(s) = l.sentinel.take() {
                    let _ = call0(&s, "release");
                }
                l.status = WakeStatus::Off;
                return;
            }
            // A lock the browser took back (the page was hidden) reads
            // `released`; drop it and ask again below.
            if let Some(s) = &l.sentinel {
                if get(s, "released").as_bool() == Some(false) {
                    l.status = WakeStatus::Held;
                    return;
                }
                l.sentinel = None;
            }
            if l.pending {
                return;
            }
            let Some(window) = web_sys::window() else { return };
            let wake = get(&get(&window, "navigator"), "wakeLock");
            if wake.is_undefined() || wake.is_null() {
                l.status = WakeStatus::Unsupported;
                return;
            }
            let wait = if l.status == WakeStatus::Refused { RETRY_AFTER_REFUSAL_S } else { 1.0 };
            if now - l.last_ask < wait || !page_visible() {
                if l.status != WakeStatus::Refused {
                    l.status = WakeStatus::Asking;
                }
                return;
            }
            let Some(request) = get(&wake, "request").dyn_into::<js_sys::Function>().ok() else {
                l.status = WakeStatus::Unsupported;
                return;
            };
            let Ok(promise) = request.call1(&wake, &JsValue::from_str("screen")) else {
                l.status = WakeStatus::Refused;
                l.last_ask = now;
                return;
            };
            let Ok(promise) = promise.dyn_into::<js_sys::Promise>() else {
                l.status = WakeStatus::Unsupported;
                return;
            };
            l.pending = true;
            l.last_ask = now;
            if l.status != WakeStatus::Refused {
                l.status = WakeStatus::Asking;
            }
            wasm_bindgen_futures::spawn_local(async move {
                let answer = wasm_bindgen_futures::JsFuture::from(promise).await;
                LOCK.with(|l| {
                    let mut l = l.borrow_mut();
                    l.pending = false;
                    match answer {
                        Ok(sentinel) => {
                            l.sentinel = Some(sentinel);
                            l.status = WakeStatus::Held;
                        }
                        Err(_) => l.status = WakeStatus::Refused,
                    }
                });
            });
        });
    }

    pub fn status(want: bool) -> WakeStatus {
        if !want {
            return WakeStatus::Off;
        }
        LOCK.with(|l| l.borrow().status)
    }
}

#[cfg(target_arch = "wasm32")]
pub use web::{keep_awake, status};

#[cfg(test)]
mod tests {
    use super::*;

    /// The setting must never sit there doing nothing in silence: off says
    /// nothing, and the installed program says plainly that it is browser-only.
    #[test]
    fn the_desktop_program_says_the_lock_is_browser_only() {
        assert_eq!(status(false), WakeStatus::Off);
        assert_eq!(status(false).label(), "");
        let on = status(true);
        assert_eq!(on, WakeStatus::Native);
        assert!(on.is_problem() && !on.label().is_empty());
    }
}
