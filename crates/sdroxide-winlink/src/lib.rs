//! Winlink radio email for sdroxide.
//!
//! NATIVE ONLY — this crate does blocking network I/O on its own threads and
//! must never be a dependency of any wasm-targeted crate. The GUI keeps its
//! threadless model: the engine owns the session worker and only drains it
//! (non-blocking) on its existing poll loop, exactly as it does the spot feeds
//! in `sdroxide-net`.
//!
//! What is here is the *client* half of Winlink, which is entirely publicly
//! specified and needs no radio: the B2F/FBB forwarding protocol, LZHUF
//! compression, the secure-login challenge, the message format, and a mailbox.
//! The transport underneath is deliberately a trait — today the CMS telnet
//! gateway over the internet, later a packet or ARDOP link — so that none of
//! the protocol work has to be revisited when a radio transport arrives.

pub mod b2f;
pub mod lzhuf;
pub mod mailbox;
pub mod manager;
pub mod message;
pub mod secure;
pub mod session;
pub mod transport;

pub use b2f::{Answer, Proposal, Sid};
pub use lzhuf::{LzhufError, compress, decompress};
pub use mailbox::{Folder, Mailbox, MailboxError};
pub use manager::{WinlinkManager, WinlinkRoute};
pub use message::{Attachment, Message, MessageError};
pub use secure::login_response;
pub use session::{SessionConfig, SessionError, SessionOutcome};
pub use transport::{TelnetTransport, Transport, TransportError};
