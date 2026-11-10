//! Errors, written for the operator who has to act on them.
//!
//! The board's own refusal codes are the interesting half: each one names a
//! configuration that cannot work, and each has a different fix. Passing the
//! number through would leave the operator with "error 3" and a board that has
//! quietly stayed where it was.

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot open the LimeRFE serial port {path}: {source}")]
    Open { path: String, source: std::io::Error },

    #[error("LimeRFE serial I/O failed: {0}")]
    Io(#[from] std::io::Error),

    /// Nothing answered the hello handshake.
    #[error(
        "no LimeRFE answered on {path} — check it is the board's own USB port and not the \
         radio's, and that nothing else has it open"
    )]
    NoAnswer { path: String },

    #[error("the LimeRFE replied with {got} bytes, expected {want} — the link is out of sync")]
    ShortReply { want: usize, got: usize },

    /// A refusal from the board, already turned into the fix for it.
    #[error("{0}")]
    Board(String),

    /// The board stopped answering and has been given up on for this session.
    #[error("the LimeRFE stopped answering and has been left alone: {0}")]
    Absent(String),

    /// The board/I²C path was asked for but the LimeSuite in use has no
    /// LimeRFE support compiled in.
    #[error(
        "this LimeSuite build has no LimeRFE support (it arrived in 20.01) — upgrade it, or \
         connect the LimeRFE to its own USB port and choose that link instead"
    )]
    NoLibrarySupport,
}

impl Error {
    /// Turn one of the board's `RFE_ERROR_*` codes into a sentence with the fix
    /// in it. The numbers are from `/usr/include/lime/limeRFE.h`.
    pub fn from_board(code: u8) -> Error {
        Error::Board(match code {
            1 => "the LimeRFE refused the transmit connector: this channel's transmit path \
                      is not wired to that port. HF and 6 m transmit only through J5; \
                      everything else transmits through J3 or J4."
                .to_string(),
            2 => "the LimeRFE refused the receive connector: this channel's receive path is \
                      not wired to that port. J5 receives only up to 70 cm."
                .to_string(),
            3 => "the LimeRFE refused receive-and-transmit-at-once: that needs receive and \
                      transmit on different connectors. Wire transmit to J4, or let the mode \
                      switch at key-down."
                .to_string(),
            4 => "the LimeRFE refused the mode for a cellular channel: the FDD bands (1, 2, \
                      3, 7) must be receive-and-transmit, and TDD band 38 must not be."
                .to_string(),
            5 => "the LimeRFE requires the same cellular channel for receive and transmit."
                .to_string(),
            6 => "the LimeRFE does not know that channel code — a newer channel than this \
                      board's firmware has."
                .to_string(),
            // Negative codes arrive here as their two's complement.
            252 => "the LimeRFE could not synchronise: the link is out of step. Unplug and \
                        replug the board."
                .to_string(),
            253 => "the LimeRFE refused that GPIO pin — only pins 4 and 5 are configurable."
                .to_string(),
            255 => "the LimeRFE reported a communication error.".to_string(),
            other => format!(
                "the LimeRFE refused the request with code {other}, which this build does \
                     not have a description for"
            ),
        })
    }
}
