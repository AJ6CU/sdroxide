//! The Wikipedia thumbnail of an airport, for the airport card.
//!
//! The station is told a language and an article title (both from the table
//! embedded in the program, but a client could send anything) and builds every
//! address itself: [`Article::new`] accepts only a two-letter language and a
//! plain title, so the host is always `xx.wikipedia.org` and the path always
//! the summary API's. The summary names a thumbnail; only an `https` address on
//! `upload.wikimedia.org` or `thumb.wikimedia.org` is followed ([`image_url_ok`]), and what comes back
//! must be a JPEG or a PNG of a bounded size ([`crate::image_ok`]).
//!
//! Nothing here is cached: the picture is held in memory for the open card and
//! let go with it, as an aircraft's is.

use serde::Deserialize;

/// The longest title taken, in bytes. Wikipedia's own limit is 255; the airports'
/// are far shorter, and the shorter the less a request can carry.
const MAX_TITLE: usize = 120;

/// A Wikipedia article, as far as the station will ask for it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Article {
    lang: String,
    title: String,
}

impl Article {
    /// `lang` is two lowercase letters; `title` is letters, digits and
    /// `_ - ( ) , . ' ! : & " – — ’` (so `Amsterdam_Airport_Schiphol`; the dashes
    /// and the quote are in real airport titles, e.g. `Hartsfield–Jackson_Atlanta_International_Airport`), not starting
    /// with a dot. Anything else is not an article this station looks up.
    pub fn new(lang: &str, title: &str) -> Option<Article> {
        let lang_ok = lang.len() == 2 && lang.bytes().all(|b| b.is_ascii_lowercase());
        let title_ok = !title.is_empty()
            && title.len() <= MAX_TITLE
            && !title.starts_with('.')
            && title.chars().all(|c| c.is_alphanumeric() || "_-(),.'!:&\"–—’".contains(c));
        (lang_ok && title_ok).then(|| Article { lang: lang.to_string(), title: title.to_string() })
    }

    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    /// `en:Amsterdam_Airport_Schiphol`: what two requests for the same article share.
    pub fn key(&self) -> String {
        format!("{}:{}", self.lang, self.title)
    }

    /// The one host this article's summary is asked of.
    pub fn host(&self) -> String {
        format!("{}.wikipedia.org", self.lang)
    }

    /// The summary API's path for this article.
    pub fn summary_path(&self) -> String {
        format!("/api/rest_v1/page/summary/{}", escape(&self.title))
    }

    /// The article's page: where the credit points, since the picture's author
    /// and licence are on its page.
    pub fn page_url(&self) -> String {
        format!("https://{}/wiki/{}", self.host(), escape(&self.title))
    }
}

/// Percent-escape everything but the unreserved characters, as a path segment.
fn escape(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    for b in title.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Whether an address is one Wikimedia hands out for a picture: `https`, no
/// credentials, no port, and one of Wikimedia's two picture hosts:
/// `upload.wikimedia.org` and `thumb.wikimedia.org` (the summary API now hands
/// out its thumbnails from the latter).
pub fn image_url_ok(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else { return false };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    matches!(host, "upload.wikimedia.org" | "thumb.wikimedia.org")
        && !url.contains(['\\', ' '])
        && !rest.contains('@')
}

#[derive(Deserialize)]
struct Summary {
    thumbnail: Option<Thumbnail>,
}

#[derive(Deserialize)]
struct Thumbnail {
    source: String,
}

/// The thumbnail's address in a summary reply, if there is one and it is
/// [`image_url_ok`]. Anything else — no picture, another host, not JSON — is `None`.
pub fn parse_summary(json: &str) -> Option<String> {
    let s: Summary = serde_json::from_str(json).ok()?;
    s.thumbnail.map(|t| t.source).filter(|u| image_url_ok(u))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUMMARY: &str = r#"{"type":"standard","title":"Amsterdam Airport Schiphol",
        "thumbnail":{"source":"https://upload.wikimedia.org/wikipedia/commons/thumb/a/a1/Schiphol.jpg/320px-Schiphol.jpg","width":320,"height":213},
        "originalimage":{"source":"https://upload.wikimedia.org/wikipedia/commons/a/a1/Schiphol.jpg"}}"#;

    /// Real airport titles carry an en dash or a quotation mark (Atlanta's
    /// `Hartsfield–Jackson…`, Ottawa's `Ottawa_Macdonald–Cartier…`); they are
    /// escaped like any other byte and must not be refused.
    #[test]
    fn titles_with_a_dash_or_a_quote_are_articles() {
        let a = Article::new("en", "Hartsfield–Jackson_Atlanta_International_Airport").unwrap();
        assert_eq!(
            a.summary_path(),
            "/api/rest_v1/page/summary/Hartsfield%E2%80%93Jackson_Atlanta_International_Airport"
        );
        let q = Article::new("fr", "Aéroport_\"X\"").unwrap();
        assert!(q.page_url().ends_with("A%C3%A9roport_%22X%22"), "{}", q.page_url());
    }

    #[test]
    fn the_station_builds_the_address_from_a_language_and_a_title() {
        let a = Article::new("en", "Amsterdam_Airport_Schiphol").unwrap();
        assert_eq!(a.host(), "en.wikipedia.org");
        assert_eq!(a.summary_path(), "/api/rest_v1/page/summary/Amsterdam_Airport_Schiphol");
        assert_eq!(a.page_url(), "https://en.wikipedia.org/wiki/Amsterdam_Airport_Schiphol");
        assert_eq!(a.key(), "en:Amsterdam_Airport_Schiphol");
        let a = Article::new("es", "Aeropuerto_de_Málaga-Costa_del_Sol_(Pablo_Ruiz)").unwrap();
        assert_eq!(
            a.summary_path(),
            "/api/rest_v1/page/summary/Aeropuerto_de_M%C3%A1laga-Costa_del_Sol_%28Pablo_Ruiz%29"
        );
    }

    #[test]
    fn a_language_or_title_that_could_aim_the_request_elsewhere_is_refused() {
        for lang in ["", "e", "eng", "EN", "e1", "en.evil", "en/x", "é"] {
            assert!(Article::new(lang, "Paris").is_none(), "{lang:?}");
        }
        for title in [
            "",
            "..",
            ".hidden",
            "a/b",
            "a?b=1",
            "a#b",
            "a%2Fb",
            "a b",
            "a\\b",
            "a\nb",
            "a@evil.example",
            "<script>",
            &"x".repeat(MAX_TITLE + 1),
        ] {
            assert!(Article::new("en", title).is_none(), "{title:?}");
        }
        assert!(Article::new("en", &"x".repeat(MAX_TITLE)).is_some());
    }

    #[test]
    fn only_a_wikimedia_upload_address_over_https_is_followed() {
        assert!(image_url_ok("https://upload.wikimedia.org/wikipedia/commons/a.jpg"));
        // The summary API's thumbnails now come from this host (checked live).
        assert!(image_url_ok(
            "https://thumb.wikimedia.org/wikipedia/commons/thumb/d/d9/A.jpg/330px-A.jpg?utm_source=en.wikipedia.org"
        ));
        for bad in [
            "http://thumb.wikimedia.org/a.jpg",
            "https://thumb.wikimedia.org.evil.example/a.jpg",
            "https://thumb.wikimedia.org:8443/a.jpg",
            "https://thumb.wikimedia.org@evil.example/a.jpg",
            "https://commons.wikimedia.org/a.jpg",
            "http://upload.wikimedia.org/a.jpg",
            "https://evil.example/a.jpg",
            "https://upload.wikimedia.org.evil.example/a.jpg",
            "https://user@upload.wikimedia.org/a.jpg",
            "https://upload.wikimedia.org:8443/a.jpg",
            "https://upload.wikimedia.org@evil.example/a.jpg",
            "https://en.wikipedia.org/a.jpg",
            "https://upload.wikimedia.org\\@evil.example/a.jpg",
            "//upload.wikimedia.org/a.jpg",
            "",
        ] {
            assert!(!image_url_ok(bad), "{bad}");
        }
    }

    #[test]
    fn the_summary_gives_the_thumbnail_and_nothing_else() {
        assert_eq!(
            parse_summary(SUMMARY).as_deref(),
            Some(
                "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a1/Schiphol.jpg/320px-Schiphol.jpg"
            )
        );
        // No picture on the page, a picture on another host, and not JSON at all.
        assert_eq!(parse_summary(r#"{"title":"X"}"#), None);
        assert_eq!(parse_summary(r#"{"thumbnail":{"source":"https://evil.example/a.jpg"}}"#), None);
        assert_eq!(parse_summary("<html>"), None);
    }
}
