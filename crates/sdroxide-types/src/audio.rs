//! Output-audio routing — which speaker channel the received audio goes to.
//!
//! A listener hearing on one side only (one earpiece, a single monitor speaker)
//! wants the audio on that side rather than split; the setting silences the
//! other channel at the output stage, so nothing else in the chain changes.
//! `Both` is the default and exactly the old behaviour.

use serde::{Deserialize, Serialize};

/// Which output channels the speaker audio reaches (#654).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioRouting {
    /// Left and right, as produced — the default.
    #[default]
    Both,
    /// The left channel only; the right is silenced.
    Left,
    /// The right channel only; the left is silenced.
    Right,
}

impl AudioRouting {
    /// Every choice, in the order a picker lists them.
    pub const ALL: [AudioRouting; 3] =
        [AudioRouting::Both, AudioRouting::Left, AudioRouting::Right];

    pub fn label(self) -> &'static str {
        match self {
            AudioRouting::Both => "Left + right",
            AudioRouting::Left => "Left only",
            AudioRouting::Right => "Right only",
        }
    }

    /// The interleaved `(left, right)` pair that actually reaches the card.
    pub fn apply(self, left: f32, right: f32) -> (f32, f32) {
        match self {
            AudioRouting::Both => (left, right),
            AudioRouting::Left => (left, 0.0),
            AudioRouting::Right => (0.0, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routing_silences_the_other_channel() {
        assert_eq!(AudioRouting::Both.apply(0.5, -0.5), (0.5, -0.5));
        assert_eq!(AudioRouting::Left.apply(0.5, -0.5), (0.5, 0.0));
        assert_eq!(AudioRouting::Right.apply(0.5, -0.5), (0.0, -0.5));
        assert_eq!(AudioRouting::default(), AudioRouting::Both);
    }
}
