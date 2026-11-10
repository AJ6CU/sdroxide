//! On-disk cache for the fetched feeds.
//!
//! Two jobs, both about not being useless offline:
//!
//! * The worker publishes the cache *before* its first network request, so the
//!   window opens instantly with the last-known Sun and CME list even with no
//!   connection at all.
//! * `ETag` / `Last-Modified` are remembered per URL, so a refresh that finds
//!   nothing new costs a 304 and no body — which matters when the SDO image is
//!   polled every ten minutes but only changes every few.
//!
//! Everything here degrades to a no-op rather than failing: a missing or
//! unwritable cache directory should cost freshness, never functionality.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// What a previous fetch of a URL told us, for conditional requests.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Validators {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub last_modified: Option<String>,
    #[serde(default)]
    pub fetched_unix: i64,
}

#[derive(Default, Serialize, Deserialize)]
struct Index {
    #[serde(default)]
    entries: HashMap<String, Validators>,
}

pub struct Cache {
    dir: Option<PathBuf>,
    index: Index,
}

impl Cache {
    /// Open the cache directory, or a disabled cache if it is unavailable.
    pub fn open() -> Cache {
        let dir = match sdroxide_config::solar_cache_dir() {
            Ok(d) => Some(d),
            Err(e) => {
                tracing::warn!("solar cache unavailable, running without it: {e}");
                None
            }
        };
        let index = dir
            .as_ref()
            .and_then(|d| std::fs::read_to_string(d.join("index.json")).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Cache { dir, index }
    }

    pub fn is_enabled(&self) -> bool {
        self.dir.is_some()
    }

    pub fn validators(&self, url: &str) -> Validators {
        self.index.entries.get(url).cloned().unwrap_or_default()
    }

    /// When `url` was last fetched successfully, or 0.
    pub fn fetched_at(&self, url: &str) -> i64 {
        self.index.entries.get(url).map_or(0, |v| v.fetched_unix)
    }

    pub fn read(&self, name: &str) -> Option<Vec<u8>> {
        std::fs::read(self.dir.as_ref()?.join(sanitize(name))).ok()
    }

    pub fn read_string(&self, name: &str) -> Option<String> {
        String::from_utf8(self.read(name)?).ok()
    }

    /// Store a body and remember its validators. Failures are logged, not
    /// propagated: a full or read-only disk must not take the view down.
    pub fn write(&mut self, name: &str, url: &str, bytes: &[u8], v: Validators) {
        if let Some(dir) = &self.dir {
            let path = dir.join(sanitize(name));
            if let Err(e) = std::fs::write(&path, bytes) {
                tracing::warn!("solar cache write {}: {e}", path.display());
                return;
            }
        }
        self.index.entries.insert(url.to_string(), v);
        self.save_index();
    }

    /// Refresh a URL's timestamp without rewriting its body — the 304 case.
    pub fn touch(&mut self, url: &str, now_unix: i64) {
        self.index.entries.entry(url.to_string()).or_default().fetched_unix = now_unix;
        self.save_index();
    }

    fn save_index(&self) {
        let Some(dir) = &self.dir else { return };
        match serde_json::to_string_pretty(&self.index) {
            Ok(s) => {
                if let Err(e) = std::fs::write(dir.join("index.json"), s) {
                    tracing::warn!("solar cache index: {e}");
                }
            }
            Err(e) => tracing::warn!("solar cache index encode: {e}"),
        }
    }
}

/// Keep cache names to a flat, well-behaved file name.
///
/// The names are all built from our own enums today, but they end up in a path,
/// and a name that could ever contain a separator or `..` must not be able to
/// escape the cache directory.
fn sanitize(name: &str) -> String {
    let s: String = name
        .chars()
        .map(
            |c| if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' { c } else { '_' },
        )
        .collect();
    if s.is_empty() || s.starts_with('.') { format!("_{s}") } else { s }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_keeps_names_inside_the_cache_directory() {
        assert_eq!(sanitize("sun_HMIIC_1024.jpg"), "sun_HMIIC_1024.jpg");
        assert_eq!(sanitize("cme.json"), "cme.json");
        assert_eq!(sanitize("../../etc/passwd"), "_.._.._etc_passwd");
        assert_eq!(sanitize("a/b\\c"), "a_b_c");
        assert_eq!(sanitize(""), "_");
        assert_eq!(sanitize(".hidden"), "_.hidden");
        // Never produces a path with a separator, whatever goes in.
        for s in ["/", "..", "a/../../b", "\0weird"] {
            let out = sanitize(s);
            assert!(!out.contains('/') && !out.contains('\\'), "{s:?} → {out:?}");
            assert_ne!(out, "..");
        }
    }

    #[test]
    fn a_disabled_cache_is_harmless() {
        let mut c = Cache { dir: None, index: Index::default() };
        assert!(!c.is_enabled());
        assert_eq!(c.read("anything"), None);
        assert_eq!(c.fetched_at("http://x"), 0);
        // Writing still records validators, so conditional requests keep
        // working even with nowhere to put the bodies.
        c.write("x.json", "http://x", b"{}", Validators { fetched_unix: 42, ..Default::default() });
        assert_eq!(c.fetched_at("http://x"), 42);
    }

    #[test]
    fn validators_round_trip_through_the_index() {
        let mut c = Cache { dir: None, index: Index::default() };
        c.write(
            "cme.json",
            "http://donki/CME",
            b"[]",
            Validators {
                etag: Some("\"abc\"".into()),
                last_modified: Some("Fri, 24 Jul 2026 06:00:00 GMT".into()),
                fetched_unix: 1_784_937_600,
            },
        );
        let v = c.validators("http://donki/CME");
        assert_eq!(v.etag.as_deref(), Some("\"abc\""));
        assert_eq!(v.fetched_unix, 1_784_937_600);
        // Unknown URLs come back blank rather than panicking.
        assert!(c.validators("http://nowhere").etag.is_none());

        c.touch("http://donki/CME", 1_784_940_000);
        assert_eq!(c.fetched_at("http://donki/CME"), 1_784_940_000);
        // A touch must not lose the entity tag — that is the whole point.
        assert_eq!(c.validators("http://donki/CME").etag.as_deref(), Some("\"abc\""));
    }

    #[test]
    fn an_index_from_a_future_version_still_loads() {
        // Unknown fields and missing ones both have to be survivable, or a
        // format change bricks the cache instead of just missing it.
        let idx: Index =
            serde_json::from_str(r#"{"entries":{"u":{"etag":"e","surprise":1}},"extra":true}"#)
                .expect("index must tolerate schema drift");
        assert_eq!(idx.entries["u"].etag.as_deref(), Some("e"));
        assert_eq!(idx.entries["u"].fetched_unix, 0);
    }
}
