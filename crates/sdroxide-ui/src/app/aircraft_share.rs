//! The open aircraft card as plain text, for the clipboard.
//!
//! Pure: it is given the lines the card already shows and returns a string.
//! It reads nothing else, so nothing the card does not show can leave in it.
//! What stays out on purpose: the aircraft's position and its last ACARS
//! message (free text a crew or a system typed), the thumbnail address (the
//! picture is never kept or passed on, only its page and credit), and any
//! reference to the station, the session or the operator.

use super::aircraft_online::{Badge, PhotoView, Row};

/// Lines that are never exported, by label.
const WITHHELD: [&str; 2] = ["Position", "Last message"];

fn source(b: Badge) -> Option<&'static str> {
    match b {
        Badge::Radio => Some("radio"),
        Badge::Online => Some("online"),
        Badge::History => Some("station history"),
        Badge::Missing => None,
    }
}

/// The summary: a title, the card's detail line, every line somebody has, with
/// where it came from, then the photo's page and credit when there is one.
pub(in crate::app) fn summary_text(
    title: &str,
    detail: &str,
    rows: &[Row],
    photo: Option<&PhotoView>,
) -> String {
    let mut out = String::new();
    out.push_str(title);
    out.push('\n');
    if !detail.trim().is_empty() {
        out.push_str(detail.trim());
        out.push('\n');
    }
    let mut any = false;
    for r in rows {
        let Some(src) = source(r.badge) else { continue };
        if WITHHELD.contains(&r.label) {
            continue;
        }
        any = true;
        out.push_str(&format!("{}: {} ({src})\n", r.label, r.value));
    }
    if !any {
        out.push_str("Nothing known yet beyond the identity above.\n");
    }
    if let Some(p) = photo {
        match &p.page_url {
            Some(url) => out.push_str(&format!("{} — {url}\n", p.credit)),
            None => out.push_str(&format!("{}\n", p.credit)),
        }
    }
    out.push_str("Heard with SDR Oxide Brown\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(label: &'static str, value: &str, badge: Badge) -> Row {
        Row { label, value: value.to_string(), badge, note: "5s ago".to_string() }
    }

    fn photo(page: Option<&str>) -> PhotoView {
        PhotoView {
            thumb_url: "https://t.plnspttrs.net/secret-thumb.jpg".to_string(),
            show_image: true,
            credit: "Photo: A. Spotter / Planespotters".to_string(),
            page_url: page.map(str::to_string),
        }
    }

    #[test]
    fn each_known_line_carries_where_it_came_from() {
        let rows = [
            row("Flight", "KLM123", Badge::Radio),
            row("Type", "Boeing 737", Badge::Online),
            row("Track", "2 positions", Badge::History),
        ];
        let t = summary_text("4840D6", "Netherlands", &rows, None);
        assert!(t.starts_with("4840D6\nNetherlands\n"), "{t}");
        assert!(t.contains("Flight: KLM123 (radio)\n"), "{t}");
        assert!(t.contains("Type: Boeing 737 (online)\n"), "{t}");
        assert!(t.contains("Track: 2 positions (station history)\n"), "{t}");
    }

    #[test]
    fn a_missing_line_is_left_out_rather_than_exported_as_a_dash() {
        let rows = [row("Flight", "KLM123", Badge::Radio), row("Speed", "—", Badge::Missing)];
        let t = summary_text("4840D6", "", &rows, None);
        assert!(t.contains("Flight: KLM123"), "{t}");
        assert!(!t.contains("Speed"), "{t}");
        assert!(!t.contains('—'), "{t}");
    }

    #[test]
    fn nothing_known_still_gives_the_identity_and_says_so() {
        let rows = [row("Flight", "—", Badge::Missing)];
        let t = summary_text("4840D6", "", &rows, None);
        assert!(t.starts_with("4840D6\n"), "{t}");
        assert!(t.contains("Nothing known yet"), "{t}");
        assert!(!t.contains("Flight"), "{t}");
    }

    #[test]
    fn the_position_and_the_last_message_never_leave() {
        let rows = [
            row("Position", "52.3700° N, 4.9000° E", Badge::Radio),
            row("Last message", "free text typed by a crew", Badge::Radio),
            row("Altitude", "FL350", Badge::Radio),
        ];
        let t = summary_text("4840D6", "", &rows, None);
        assert!(!t.contains("52.37") && !t.contains("Position"), "{t}");
        assert!(!t.contains("free text") && !t.contains("Last message"), "{t}");
        assert!(t.contains("Altitude: FL350"), "{t}");
    }

    #[test]
    fn the_photo_gives_its_credit_and_page_and_never_the_thumbnail() {
        let page = "https://www.planespotters.net/photo/1/x";
        let t = summary_text("4840D6", "", &[], Some(&photo(Some(page))));
        assert!(t.contains(&format!("Photo: A. Spotter / Planespotters — {page}\n")), "{t}");
        assert!(!t.contains("plnspttrs") && !t.contains("secret-thumb"), "{t}");
        let t = summary_text("4840D6", "", &[], Some(&photo(None)));
        assert!(t.contains("Photo: A. Spotter / Planespotters\n"), "{t}");
        assert!(!t.contains("http"), "{t}");
    }

    #[test]
    fn the_text_has_no_cookie_session_or_station_address() {
        let rows = [row("Flight", "KLM123", Badge::Radio)];
        let t = summary_text("4840D6", "Netherlands", &rows, None).to_lowercase();
        for bad in ["cookie", "session", "http://", "token", "127.0.0.1"] {
            assert!(!t.contains(bad), "{bad} in {t}");
        }
    }

    /// The module is pure: no egui, no clipboard, no network, no storage.
    #[test]
    fn the_module_is_pure() {
        let src = include_str!("aircraft_share.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for bad in ["egui", "copy_text", "ehttp", "fetch", "localStorage", "std::fs"] {
            assert!(!code.contains(bad), "{bad}");
        }
    }

    /// The button copies exactly this text and nothing else.
    #[test]
    fn the_card_copies_through_this_function() {
        let src = include_str!("aircraft.rs");
        assert_eq!(src.matches("aircraft_share::summary_text(").count(), 1);
        assert_eq!(src.matches("copy_text(").count(), 1, "one clipboard write in the card");
    }
}
