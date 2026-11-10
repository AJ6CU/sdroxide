//! The picture composer's **document**: the text layers drawn over a transmit
//! preset's picture.
//!
//! A preset used to carry one caption — one string, one font, one size, pinned
//! under the banner strip. The document replaces it with layers the operator
//! places, each with its own face, size, colour, outline and a strip of colour
//! behind it that may run across the picture or down it.
//!
//! **It lives here, on the wire, rather than in a panel.** Three things need
//! the same answer to "what does this picture say": the console, a browser tab
//! attached to the same radio, and the next session after a restart. A document
//! held in one client's memory gave each of them a different picture; kept in
//! the engine's preset store it is composed from the same fractions and the same
//! faces everywhere, and the browser — which has no filesystem to keep a layout
//! in — gets one at all.
//!
//! Positions are **fractions of the frame**, never pixels: the same document has
//! to land the same way on a 320-pixel Martin frame, a 640-pixel robot frame and
//! a RIFP picture the operator sized themselves.

use serde::{Deserialize, Serialize};

/// A face the composer can print in.
///
/// An enum of **bundled** faces rather than a font name, because the browser
/// client cannot reach the operator's installed fonts: a system face would make
/// the same document compose into two different pictures depending on which
/// screen pressed SEND. The list is appended to, never reordered — a discriminant
/// is on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Face {
    /// Chakra Petch SemiBold — the face the banner has always used.
    #[default]
    ChakraBold,
    /// Chakra Petch Regular, for a lighter line of comment.
    Chakra,
    /// Share Tech Mono, for call signs and reports that read as a log.
    Mono,
}

impl Face {
    pub const ALL: [Face; 3] = [Face::ChakraBold, Face::Chakra, Face::Mono];

    pub fn label(self) -> &'static str {
        match self {
            Face::ChakraBold => "Chakra Petch Bold",
            Face::Chakra => "Chakra Petch",
            Face::Mono => "Share Tech Mono",
        }
    }

    /// The OFL file the face is printed from, for the client that has the
    /// assets. The engine never renders text, so it does not care.
    pub fn file(self) -> &'static str {
        match self {
            Face::ChakraBold => "ChakraPetch-SemiBold.ttf",
            Face::Chakra => "ChakraPetch-Regular.ttf",
            Face::Mono => "ShareTechMono-Regular.ttf",
        }
    }
}

/// A strip of colour behind a layer, running the full width (or height) of the
/// frame and centred on the layer.
///
/// This is the banner, generalised: it can sit anywhere rather than at an edge,
/// it can run down the frame rather than across it, and a picture may have more
/// than one.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Strip {
    /// Thickness in picture pixels, at the frame's own size.
    pub thickness: u16,
    /// Colour at the strip's first edge.
    pub fill: [u8; 3],
    /// The colour it fades to across its thickness. `None` fades to black, which
    /// is the banner's original look.
    pub fill2: Option<[u8; 3]>,
}

impl Default for Strip {
    fn default() -> Self {
        Strip { thickness: 24, fill: [160, 16, 16], fill2: None }
    }
}

/// One line of text placed on the picture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TextLayer {
    pub text: String,
    pub face: Face,
    /// Text height in picture pixels.
    pub px: u16,
    pub ink: [u8; 3],
    /// Outline colour, when the line has to survive a light picture.
    pub outline: Option<[u8; 3]>,
    /// The centre of the line as a fraction of the frame: `(0.5, 0.5)` is the
    /// middle of the picture.
    pub x: f32,
    pub y: f32,
    /// The line runs **down** the frame instead of across it, turned a quarter
    /// turn clockwise.
    pub vertical: bool,
    pub band: Option<Strip>,
    /// Kept in the document but not drawn — a line to bring back for the next
    /// picture without retyping it.
    pub hidden: bool,
}

impl Default for TextLayer {
    fn default() -> Self {
        TextLayer {
            text: String::new(),
            face: Face::default(),
            px: 28,
            ink: [255, 255, 255],
            outline: Some([0, 0, 0]),
            x: 0.5,
            y: 0.5,
            vertical: false,
            band: None,
            hidden: false,
        }
    }
}

/// The document: an ordered list of layers, first drawn first.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Composer {
    pub layers: Vec<TextLayer>,
}

impl Composer {
    /// Whether the document draws anything. A document that draws nothing is
    /// the signal to fall back to the station's banner strip and the preset's
    /// plain caption, so a preset nobody has composed still sends the picture it
    /// always did.
    pub fn is_active(&self) -> bool {
        self.layers.iter().any(|l| !l.hidden && !l.text.trim().is_empty())
    }

    /// The default document for a slot that has none: the station's own banner
    /// as two lines, so opening the editor on a fresh preset starts from what
    /// that preset sends today rather than from an empty picture.
    ///
    /// `banner_px` is the text size to use — the caller knows the frame it is
    /// composing for, and this is a starting point the operator can drag.
    pub fn seeded(
        left: &str,
        right: &str,
        caption: &str,
        face: Face,
        px: u16,
        band: Option<Strip>,
        frame_h: u16,
    ) -> Composer {
        let y = band.map_or(0.08, |b| {
            (f32::from(b.thickness) / 2.0 / f32::from(frame_h.max(1))).clamp(0.02, 0.98)
        });
        let mut layers = Vec::new();
        if !left.trim().is_empty() {
            layers.push(TextLayer {
                text: left.to_string(),
                face,
                px,
                band,
                x: 0.22,
                y,
                ..TextLayer::default()
            });
        }
        if !right.trim().is_empty() {
            layers.push(TextLayer {
                text: right.to_string(),
                face,
                px,
                band,
                x: 0.78,
                y,
                ..TextLayer::default()
            });
        }
        let caption = caption.lines().next().unwrap_or("").trim();
        if !caption.is_empty() {
            layers.push(TextLayer {
                text: caption.to_string(),
                face,
                px,
                x: 0.5,
                y: 0.42,
                ..TextLayer::default()
            });
        }
        Composer { layers }
    }

    /// **Two documents over one picture**: the station's overlay first, then
    /// this preset's own layers on top.
    ///
    /// Neither replaces the other — that is the operator's requirement, and it is
    /// the right one: the station overlay is the identification every picture
    /// needs, and the preset's document is what *this* picture says. A preset
    /// already carrying its identification can switch the overlay off with
    /// [`ImageSlotInfo::apply_overlay`](crate::ImageSlotInfo) and keep its own.
    ///
    /// The overlay is drawn first so a preset's line always wins where they
    /// overlap: the more specific document is the one the operator is looking at
    /// when they place it.
    pub fn over(&self, overlay: &Composer) -> Composer {
        let mut layers = Vec::with_capacity(overlay.layers.len() + self.layers.len());
        layers.extend(overlay.layers.iter().cloned());
        layers.extend(self.layers.iter().cloned());
        Composer { layers }
    }

    /// Every layer's text, for a summary line in a panel that has no room for
    /// the picture.
    pub fn summary(&self) -> String {
        let drawn: Vec<&str> =
            self.layers.iter().filter(|l| !l.hidden).map(|l| l.text.trim()).collect();
        drawn.join(" · ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_document_draws_nothing() {
        assert!(!Composer::default().is_active());
        let mut c = Composer::default();
        c.layers.push(TextLayer::default());
        assert!(!c.is_active(), "a layer with no text draws nothing");
        c.layers[0].text = "HI".into();
        assert!(c.is_active());
        c.layers[0].hidden = true;
        assert!(!c.is_active(), "a hidden line is not drawn");
    }

    /// The seed is what the editor opens on: it must not invent a line for a
    /// callsign the operator has not set, and it must keep the caption.
    #[test]
    fn a_seed_carries_the_banner_and_the_caption_without_inventing_either() {
        let c = Composer::seeded("", "SDRoxide v2", "CQ CQ", Face::Mono, 24, None, 256);
        assert_eq!(c.layers.len(), 2, "the empty left template is not a line");
        assert_eq!(c.layers[0].text, "SDRoxide v2");
        assert_eq!(c.layers[1].text, "CQ CQ");
        assert!(c.is_active());

        let empty = Composer::seeded("", "", "", Face::default(), 24, None, 256);
        assert!(empty.layers.is_empty());
        assert!(!empty.is_active());
    }

    /// **The station overlay and the preset's own document compose; neither
    /// replaces the other.** The overlay goes underneath, so a preset's line wins
    /// where they overlap.
    #[test]
    fn an_overlay_and_a_presets_own_layers_compose_together() {
        let overlay = Composer::seeded("19DCG373", "", "", Face::default(), 20, None, 256);
        let own = Composer::seeded("", "", "CQ CQ", Face::default(), 24, None, 256);
        let both = own.over(&overlay);
        assert_eq!(both.layers.len(), 2, "one layer lost: {both:?}");
        assert_eq!(both.layers[0].text, "19DCG373", "the overlay is drawn first");
        assert_eq!(both.layers[1].text, "CQ CQ");
        // An empty overlay changes nothing, which is the default station.
        assert_eq!(own.over(&Composer::default()), own);
        // And an inactive overlay is still "underneath" rather than absent.
        assert!(own.over(&Composer::default()).is_active());
    }

    /// A template name is trimmed and capped, so it can be keyed by the store
    /// and shown on a chip without either surprising the other.
    #[test]
    fn a_template_name_is_trimmed_and_capped() {
        assert_eq!(LayoutTemplate::clean_name("  CQ layout  "), "CQ layout");
        let long = "x".repeat(TEMPLATE_NAME_MAX + 20);
        assert_eq!(LayoutTemplate::clean_name(&long).chars().count(), TEMPLATE_NAME_MAX);
        assert_eq!(LayoutTemplate::clean_name("   "), "");
    }

    /// A band on the top edge must not have half of itself above the picture.
    #[test]
    fn a_seeded_band_fits_inside_the_frame() {
        let band = Strip { thickness: 40, fill: [200, 0, 0], fill2: None };
        let c = Composer::seeded("A", "B", "", Face::default(), 30, Some(band), 256);
        for l in &c.layers {
            let top = l.y * 256.0 - 20.0;
            assert!(top >= -0.5, "line at y={} starts above the picture", l.y);
        }
    }
}

/// **A named layout the operator keeps** — "CQ", "Reply", "Mosaic retransmission".
///
/// Held by the engine beside the presets rather than in a panel, for the reason
/// the documents themselves are: a template is only useful if the browser and
/// the console and next week's session all have the same one.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LayoutTemplate {
    /// What the operator called it. Unique among the templates (the store
    /// replaces on the same name), and the handle a client loads it by.
    pub name: String,
    pub composer: Composer,
}

/// The longest template name accepted from a client.
pub const TEMPLATE_NAME_MAX: usize = 48;

/// How many templates the store will keep. A layout is a few kilobytes; the
/// cap is there so a hostile or broken client cannot grow the engine's memory
/// without bound, not because a station needs a limit.
pub const TEMPLATE_MAX: usize = 40;

impl LayoutTemplate {
    /// The name as it is stored: trimmed, and shortened to
    /// [`TEMPLATE_NAME_MAX`] characters.
    pub fn clean_name(name: &str) -> String {
        name.trim().chars().take(TEMPLATE_NAME_MAX).collect()
    }
}
