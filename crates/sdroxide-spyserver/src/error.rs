//! Errors from a SpyServer link.
//!
//! Every variant carries a *sentence*, not an errno, for the same reason the
//! `rtl_tcp` client does: what goes wrong here is a hostname, a port, a
//! firewall, a server that is not running, or a server that will not let this
//! client do what it asked — and none of those are visible in an error code.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    /// The link: could not resolve, could not connect, or lost the connection.
    #[error("{0}")]
    Net(String),
    /// Something arrived that this protocol does not allow. Never survivable —
    /// there is no resync marker to recover on.
    #[error("{0}")]
    Proto(String),
    /// The server works, but wants something this client cannot do (a 24-bit
    /// stream, a protocol major version it has not seen).
    #[error("{0}")]
    Unsupported(String),
    /// The server refused, or another client owns what was asked for.
    #[error("{0}")]
    Access(String),
}
