//! The "do not ask me again" answers are kept per login **and** per browser:
//! once answered, a login is not asked again on that browser; on a new browser
//! it answers once more. The browser is named from its User-Agent, without its
//! version, so an update is not a new browser.

use sdroxide_config::{ClientSettingsStore, browser_label};
use sdroxide_types::ClientAcks;

const CHROME_ANDROID: &str = "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/141.0.0.0 Mobile Safari/537.36";
const CHROME_ANDROID_NEXT: &str = "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/142.0.7444.12 Mobile Safari/537.36";
const FIREFOX_WINDOWS: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:143.0) Gecko/20100101 Firefox/143.0";
const EDGE_WINDOWS: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36 Edg/141.0.0.0";
const SAFARI_IPHONE: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) \
    AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1";

#[test]
fn a_browser_is_its_family_and_system_not_its_version() {
    assert_eq!(browser_label(Some(CHROME_ANDROID)), "Chrome/Android");
    assert_eq!(browser_label(Some(CHROME_ANDROID_NEXT)), "Chrome/Android", "an update is not new");
    assert_eq!(browser_label(Some(FIREFOX_WINDOWS)), "Firefox/Windows");
    assert_eq!(browser_label(Some(EDGE_WINDOWS)), "Edge/Windows", "Edge is not Chrome");
    assert_eq!(browser_label(Some(SAFARI_IPHONE)), "Safari/iOS", "nor is iOS macOS");
    assert_eq!(browser_label(None), "App");
}

#[test]
fn an_answer_holds_on_its_browser_and_not_on_another() {
    let mut store = ClientSettingsStore::default();
    let chrome = browser_label(Some(CHROME_ANDROID));
    let firefox = browser_label(Some(FIREFOX_WINDOWS));
    store.merge_acks(
        "f6kim",
        &chrome,
        &ClientAcks { bindings: Some(false), rx_only_dismissed: vec![2], ..Default::default() },
    );
    // Same login, same browser, next session (and after a browser update).
    let again = store.acks_for("f6kim", &browser_label(Some(CHROME_ANDROID_NEXT)));
    assert_eq!(again.bindings, Some(false));
    assert_eq!(again.rx_only_dismissed, vec![2]);
    // Same login, another browser: asked again.
    assert_eq!(store.acks_for("f6kim", &firefox), ClientAcks::default());
    // Another login on the same browser: its own answers.
    assert_eq!(store.acks_for("kevin", &chrome), ClientAcks::default());

    // And the store survives its own JSON file.
    let back: ClientSettingsStore =
        serde_json::from_str(&serde_json::to_string(&store).unwrap()).unwrap();
    assert_eq!(back.acks_for("f6kim", &chrome).bindings, Some(false));
}
