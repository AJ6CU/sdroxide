//! Greedy CTC over the model's per-frame log-probabilities.
//!
//! The model emits one distribution per 15 ms frame and never says where a
//! character started; CTC's collapse rule — drop blanks, then drop runs — is
//! what turns that back into text. We keep the frame positions as we go, because
//! the streaming layer needs them: knowing which frame a word gap sits in is the
//! only way to decide how much of a rolling window has stopped changing and can
//! be committed.

/// Model alphabet, in class order. Index 40 is a real space character; the
/// blank at 41 is CTC's "no output here" and is not a character at all.
pub const CHARS: [char; 41] = [
    ',', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '?', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', ' ',
];

/// Class index of the CTC blank.
pub const BLANK: usize = 41;

/// Class index of the word space.
pub const SPACE: usize = 40;

/// Width of the model's output distribution.
pub const CLASSES: usize = CHARS.len() + 1;

/// One decoded character and the frames it was emitted over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharSpan {
    pub ch: char,
    pub start_frame: usize,
    pub end_frame: usize,
}

/// A run of frames the model called word space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameSpan {
    pub start_frame: usize,
    pub end_frame: usize,
}

/// The result of collapsing one window.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Decoded {
    /// Collapsed text, word spaces included.
    pub text: String,
    /// Per-character frame positions, one entry per `char` in `text`.
    pub chars: Vec<CharSpan>,
    /// Runs of word space, used to find a safe place to cut the window.
    pub word_gaps: Vec<FrameSpan>,
}

impl Decoded {
    /// Text with runs of whitespace squeezed to one and the ends trimmed.
    pub fn normalized(&self) -> String {
        let mut out = String::with_capacity(self.text.len());
        let mut space = false;
        for ch in self.text.chars() {
            if ch.is_whitespace() {
                space = !out.is_empty();
                continue;
            }
            if space {
                out.push(' ');
                space = false;
            }
            out.push(ch);
        }
        out
    }

    /// Sending speed in words per minute, or `None` if there is too little to
    /// judge by.
    ///
    /// The model reports characters and when they happened, never elements, so
    /// the dit length is not measured — it is worked back out. The text is known,
    /// so how many dit units it *would* take is known exactly, and dividing that
    /// by how long it actually took gives the speed with no assumption about the
    /// message. Counting characters instead and calling five of them a word
    /// would be the usual shortcut, and it reads a quarter low on a callsign-
    /// heavy exchange, where the letters are far longer than PARIS's average.
    pub fn wpm(&self) -> Option<f32> {
        let first = self.chars.first()?;
        let last = self.chars.last()?;
        let seconds = (last.end_frame + 1).checked_sub(first.start_frame)? as f64 * crate::FRAME_S;
        if seconds < 2.0 {
            return None;
        }

        // Units of the text as sent: elements and the gaps between them, three
        // between characters and seven between words. The gap after the last
        // character was never sent, so it is not counted.
        let mut units = 0u32;
        for (i, span) in self.chars.iter().enumerate() {
            if i > 0 {
                units += if span.ch == ' ' || self.chars[i - 1].ch == ' ' { 0 } else { 3 };
            }
            if span.ch == ' ' {
                units += 7;
                continue;
            }
            let code = sdroxide_dsp::morse_encode(span.ch)?;
            let elements: u32 = code.chars().map(|e| if e == '.' { 1 } else { 3 }).sum();
            units += elements + code.chars().count().saturating_sub(1) as u32;
        }
        if units < 20 {
            return None;
        }
        // A dit is 1.2/wpm seconds, so wpm is 1.2 units per second.
        Some((1.2 * units as f64 / seconds) as f32)
    }

    /// The part of this decode lying wholly within `from..=to` frames.
    ///
    /// Characters straddling either edge are dropped rather than clipped: a
    /// window is decoded with audio either side for context, and the context is
    /// not supposed to contribute text.
    pub fn slice(&self, from: usize, to: usize) -> Decoded {
        let chars: Vec<CharSpan> = self
            .chars
            .iter()
            .filter(|c| c.start_frame >= from && c.end_frame <= to)
            .cloned()
            .collect();
        Decoded {
            text: chars.iter().map(|c| c.ch).collect(),
            word_gaps: self
                .word_gaps
                .iter()
                .copied()
                .filter(|g| g.start_frame >= from && g.end_frame <= to)
                .collect(),
            chars,
        }
    }
}

/// Collapse `log_probs`, laid out `[frames][CLASSES]` row-major, into text.
pub fn greedy(log_probs: &[f32], frames: usize) -> Decoded {
    debug_assert_eq!(log_probs.len(), frames * CLASSES);

    let mut out = Decoded::default();
    let mut previous = usize::MAX;
    let mut active: Option<usize> = None;
    let mut gap_start: Option<usize> = None;

    for frame in 0..frames {
        let row = &log_probs[frame * CLASSES..][..CLASSES];
        let best = row
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .map(|(i, _)| i)
            .unwrap_or(BLANK);

        // Word gaps are tracked on the raw per-frame path, before collapsing:
        // it is the *duration* of the space that says how firm a boundary it is.
        match (best == SPACE, gap_start) {
            (true, None) => gap_start = Some(frame),
            (false, Some(start)) => {
                out.word_gaps.push(FrameSpan { start_frame: start, end_frame: frame - 1 });
                gap_start = None;
            }
            _ => {}
        }

        if best == BLANK {
            previous = usize::MAX;
            active = None;
            continue;
        }

        if best == previous {
            // A repeat with no blank between is the same character still being
            // emitted, so extend it rather than writing it again.
            if let Some(i) = active {
                out.chars[i].end_frame = frame;
            }
            continue;
        }

        previous = best;
        out.chars.push(CharSpan { ch: CHARS[best], start_frame: frame, end_frame: frame });
        active = Some(out.chars.len() - 1);
    }

    if let Some(start) = gap_start {
        out.word_gaps.push(FrameSpan { start_frame: start, end_frame: frames.saturating_sub(1) });
    }

    out.text = out.chars.iter().map(|c| c.ch).collect();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build log-probs that put all the mass on `path`, one class per frame.
    fn path(path: &[usize]) -> (Vec<f32>, usize) {
        let mut lp = vec![-10.0f32; path.len() * CLASSES];
        for (frame, &class) in path.iter().enumerate() {
            lp[frame * CLASSES + class] = 0.0;
        }
        (lp, path.len())
    }

    fn class_of(ch: char) -> usize {
        CHARS.iter().position(|&c| c == ch).unwrap()
    }

    #[test]
    fn the_alphabet_matches_the_model_metadata() {
        let meta = include_str!("../assets/model.onnx.json");
        assert!(meta.contains("\"blank_index\": 41"));
        assert!(meta.contains("\"num_classes\": 42"));
        assert_eq!(CLASSES, 42);
        assert_eq!(BLANK, 41);
        assert_eq!(CHARS[SPACE], ' ');
        // The order the model was trained with: punctuation, digits, ?, letters.
        assert_eq!(CHARS[0], ',');
        assert_eq!(CHARS[13], '?');
        assert_eq!(CHARS[14], 'A');
        assert_eq!(CHARS[39], 'Z');
    }

    #[test]
    fn blanks_separate_repeated_characters() {
        let e = class_of('E');
        // E E with a blank between is two characters; E E adjacent is one.
        let (lp, n) = path(&[e, BLANK, e]);
        assert_eq!(greedy(&lp, n).text, "EE");
        let (lp, n) = path(&[e, e, e]);
        assert_eq!(greedy(&lp, n).text, "E");
    }

    #[test]
    fn a_held_character_keeps_one_span() {
        let k = class_of('K');
        let (lp, n) = path(&[BLANK, k, k, k, BLANK]);
        let d = greedy(&lp, n);
        assert_eq!(d.chars, vec![CharSpan { ch: 'K', start_frame: 1, end_frame: 3 }]);
    }

    #[test]
    fn word_gaps_are_recorded_as_runs() {
        let (lp, n) = path(&[class_of('A'), SPACE, SPACE, SPACE, class_of('B')]);
        let d = greedy(&lp, n);
        assert_eq!(d.text, "A B");
        assert_eq!(d.word_gaps, vec![FrameSpan { start_frame: 1, end_frame: 3 }]);
    }

    #[test]
    fn a_gap_still_open_at_the_end_is_closed() {
        let (lp, n) = path(&[class_of('A'), SPACE, SPACE]);
        assert_eq!(greedy(&lp, n).word_gaps, vec![FrameSpan { start_frame: 1, end_frame: 2 }]);
    }

    #[test]
    fn slicing_keeps_whole_characters_only() {
        let (lp, n) =
            path(&[class_of('C'), BLANK, class_of('Q'), class_of('Q'), BLANK, class_of('K')]);
        let d = greedy(&lp, n);
        assert_eq!(d.text, "CQK");
        // Q spans frames 2..3, so a cut at 2 clips it and it is dropped.
        assert_eq!(d.slice(0, 2).text, "C");
        assert_eq!(d.slice(0, 3).text, "CQ");
        assert_eq!(d.slice(0, 99).text, "CQK");
        // Leading context is excluded the same way, from the other side.
        assert_eq!(d.slice(2, 99).text, "QK");
        assert_eq!(d.slice(3, 99).text, "K");
    }

    #[test]
    fn normalizing_squeezes_and_trims_spaces() {
        let d = Decoded { text: "  CQ   DE  W1AW ".into(), ..Default::default() };
        assert_eq!(d.normalized(), "CQ DE W1AW");
    }

    /// Lay `text` out with the timing a key would actually produce at `wpm`.
    fn timed(text: &str, wpm: f32) -> Decoded {
        let frames_per_unit = (1.2 / wpm as f64) / crate::FRAME_S;
        let letters: Vec<char> = text.chars().collect();
        let mut chars = Vec::new();
        let mut at = 0.0f64; // position in dit units
        for (i, &ch) in letters.iter().enumerate() {
            if i > 0 && ch != ' ' && letters[i - 1] != ' ' {
                at += 3.0;
            }
            let own = if ch == ' ' {
                7.0
            } else {
                let code = sdroxide_dsp::morse_encode(ch).expect("in the Morse table");
                let elements: usize = code.chars().map(|e| if e == '.' { 1 } else { 3 }).sum();
                (elements + code.chars().count() - 1) as f64
            };
            let start = (at * frames_per_unit).round() as usize;
            let end = ((at + own) * frames_per_unit).round() as usize;
            chars.push(CharSpan {
                ch,
                start_frame: start,
                end_frame: end.saturating_sub(1).max(start),
            });
            at += own;
        }
        Decoded { text: text.into(), chars, word_gaps: Vec::new() }
    }

    #[test]
    fn speed_is_read_back_from_the_text_and_its_timing() {
        for wpm in [12.0, 20.0, 35.0] {
            let got = timed("PARIS PARIS PARIS", wpm).wpm().expect("enough to judge");
            assert!((got - wpm).abs() < 1.0, "PARIS at {wpm} WPM read as {got:.1}");
        }
    }

    #[test]
    fn speed_does_not_depend_on_what_was_sent() {
        // A callsign exchange is far heavier per character than PARIS. Counting
        // characters and calling five a word reads it about a quarter low;
        // counting units has to read it right.
        let heavy = timed("VK3XY VK3XY K6XX", 24.0).wpm().expect("enough to judge");
        assert!((heavy - 24.0).abs() < 1.0, "callsigns at 24 WPM read as {heavy:.1}");
        let light = timed("E E E E T T T T", 24.0).wpm().expect("enough to judge");
        assert!((light - 24.0).abs() < 1.5, "single letters at 24 WPM read as {light:.1}");
    }

    #[test]
    fn a_short_burst_has_no_speed_to_report() {
        let (lp, n) = path(&[class_of('E'), BLANK, class_of('E')]);
        assert!(greedy(&lp, n).wpm().is_none());
    }

    #[test]
    fn an_all_blank_path_decodes_to_nothing() {
        let (lp, n) = path(&[BLANK; 20]);
        let d = greedy(&lp, n);
        assert!(d.text.is_empty());
        assert!(d.word_gaps.is_empty());
    }
}
