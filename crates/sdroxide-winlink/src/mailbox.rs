//! The on-disk mailbox.
//!
//! One message per file, named by its MID, under a directory per folder. That
//! is deliberately dull: a mailbox is the one part of this crate holding
//! something the operator cannot get back if it is lost, so it stores the
//! wire form verbatim — the exact bytes [`Message::encode`] produces — rather
//! than a parsed representation that a future version might read differently.
//!
//! Writes go through a temporary file and a rename, so a crash mid-write
//! leaves either the old message or the new one and never a half of either.

use std::path::{Path, PathBuf};

use crate::message::Message;

/// Where a message sits in its life.
///
/// Defined in `sdroxide-types` rather than here, because the browser client
/// has to name folders too and links none of this crate.
pub use sdroxide_types::MailFolder as Folder;

#[derive(Debug, thiserror::Error)]
pub enum MailboxError {
    #[error("mailbox io at {0}: {1}")]
    Io(PathBuf, #[source] std::io::Error),
    #[error("{0:?} is not a usable message id")]
    BadId(String),
    #[error("no message {0} in {1}")]
    NotFound(String, &'static str),
    #[error("reading {0}: {1}")]
    Parse(String, #[source] crate::message::MessageError),
}

/// A mailbox rooted at a directory.
pub struct Mailbox {
    root: PathBuf,
}

impl Mailbox {
    /// Open (and create) a mailbox under `root`.
    pub fn open(root: impl Into<PathBuf>) -> Result<Mailbox, MailboxError> {
        let root = root.into();
        for folder in Folder::ALL {
            let dir = root.join(folder.dir_name());
            std::fs::create_dir_all(&dir).map_err(|e| MailboxError::Io(dir, e))?;
        }
        Ok(Mailbox { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The path a message id maps to, or `None` if the id could not name a file.
    fn path_for(&self, folder: Folder, id: &str) -> Option<PathBuf> {
        let id = safe_mid(id)?;
        Some(self.root.join(folder.dir_name()).join(format!("{id}.b2f")))
    }

    /// Store a message, overwriting any of the same id in that folder.
    pub fn put(&self, folder: Folder, msg: &Message) -> Result<(), MailboxError> {
        let path =
            self.path_for(folder, &msg.mid).ok_or_else(|| MailboxError::BadId(msg.mid.clone()))?;
        write_atomic(&path, &msg.encode())
    }

    /// Read one message back.
    pub fn get(&self, folder: Folder, id: &str) -> Result<Message, MailboxError> {
        let path = self.path_for(folder, id).ok_or_else(|| MailboxError::BadId(id.into()))?;
        let raw = std::fs::read(&path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                MailboxError::NotFound(id.into(), folder.dir_name())
            } else {
                MailboxError::Io(path.clone(), e)
            }
        })?;
        Message::decode(&raw).map_err(|e| MailboxError::Parse(id.into(), e))
    }

    /// Every message in a folder, newest first.
    ///
    /// A file that will not parse is skipped rather than failing the listing:
    /// one unreadable message must not make the whole mailbox unopenable.
    pub fn list(&self, folder: Folder) -> Result<Vec<Message>, MailboxError> {
        let dir = self.root.join(folder.dir_name());
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(MailboxError::Io(dir, e)),
        };

        let mut out = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("b2f") {
                continue;
            }
            let Ok(raw) = std::fs::read(&path) else { continue };
            match Message::decode(&raw) {
                Ok(msg) => out.push(msg),
                Err(e) => {
                    tracing::warn!(path = %path.display(), "skipping unreadable message: {e}");
                }
            }
        }

        // Newest first. `Date` is `YYYY/MM/DD HH:MM`, which sorts correctly as
        // text — that is the one convenience the format's date layout offers.
        out.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| b.mid.cmp(&a.mid)));
        Ok(out)
    }

    /// How many messages a folder holds, without parsing any of them.
    pub fn count(&self, folder: Folder) -> usize {
        let dir = self.root.join(folder.dir_name());
        std::fs::read_dir(dir)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("b2f"))
                    .count()
            })
            .unwrap_or(0)
    }

    /// True when the folder already holds this id — how an already-seen
    /// message is recognised, so a redelivery does not duplicate.
    pub fn contains(&self, folder: Folder, id: &str) -> bool {
        self.path_for(folder, id).is_some_and(|p| p.is_file())
    }

    /// Delete a message. Missing is not an error: the caller wanted it gone.
    pub fn remove(&self, folder: Folder, id: &str) -> Result<(), MailboxError> {
        let path = self.path_for(folder, id).ok_or_else(|| MailboxError::BadId(id.into()))?;
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(MailboxError::Io(path, e)),
        }
    }

    /// Move a message between folders — outbox to sent, once a session has
    /// handed it over. Implemented as copy-then-delete so a failure leaves the
    /// message in the source folder rather than nowhere.
    pub fn move_to(&self, from: Folder, to: Folder, id: &str) -> Result<(), MailboxError> {
        let msg = self.get(from, id)?;
        self.put(to, &msg)?;
        self.remove(from, id)
    }
}

/// Longest message id we will store. The protocol caps a MID at 12, but a
/// forwarding partner is not obliged to be careful and the cap is what keeps a
/// hostile one from generating pathologically long filenames.
const MAX_ID_LEN: usize = 32;

/// Validate a message id before it is allowed to name a file.
///
/// A MID is chosen by whoever sent the message and reaches us over a socket
/// nothing authenticates, so this is a path-traversal rail, not a formality.
/// It is an allow-list — ASCII letters, digits, `-` and `_` — which refuses
/// `.`, `..`, `/`, `\`, NUL and every Unicode separator look-alike in one go,
/// rather than a blocklist that has to keep up with them.
///
/// The picture store's `sdroxide_types::safe_name` looks like the same job and
/// is not: it additionally requires a `.png` extension, so it rejects every
/// valid MID.
fn safe_mid(id: &str) -> Option<&str> {
    if id.is_empty() || id.len() > MAX_ID_LEN {
        return None;
    }
    id.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_')).then_some(id)
}

/// Write via a temporary file and a rename, so the destination is only ever
/// the whole old content or the whole new content.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), MailboxError> {
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes).map_err(|e| MailboxError::Io(tmp.clone(), e))?;
    std::fs::rename(&tmp, path).map_err(|e| {
        // Leave nothing behind if the rename failed.
        let _ = std::fs::remove_file(&tmp);
        MailboxError::Io(path.to_path_buf(), e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Attachment;

    struct TempDir(PathBuf);
    impl TempDir {
        fn new(tag: &str) -> TempDir {
            let p =
                std::env::temp_dir().join(format!("sdroxide-mailbox-{tag}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&p);
            TempDir(p)
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn msg(mid: &str, date: &str) -> Message {
        Message {
            mid: mid.into(),
            date: date.into(),
            msg_type: "Private".into(),
            from: "OE1XYZ".into(),
            to: vec!["OE3JJS".into()],
            cc: vec![],
            subject: format!("subject {mid}"),
            mbo: "OE1XYZ".into(),
            body: "body text\r\n".into(),
            attachments: vec![],
        }
    }

    #[test]
    fn round_trips_a_message() {
        let dir = TempDir::new("rt");
        let mb = Mailbox::open(&dir.0).unwrap();
        let mut m = msg("ABCDEFGHIJKL", "2026/08/14 09:30");
        m.attachments = vec![Attachment { name: "a.bin".into(), data: vec![0, 1, 2, 255] }];

        mb.put(Folder::Inbox, &m).unwrap();
        assert_eq!(mb.get(Folder::Inbox, &m.mid).unwrap(), m);
        assert!(mb.contains(Folder::Inbox, &m.mid));
        assert_eq!(mb.count(Folder::Inbox), 1);
    }

    #[test]
    fn lists_newest_first() {
        let dir = TempDir::new("list");
        let mb = Mailbox::open(&dir.0).unwrap();
        mb.put(Folder::Inbox, &msg("AAAAAAAAAAAA", "2026/08/12 10:00")).unwrap();
        mb.put(Folder::Inbox, &msg("BBBBBBBBBBBB", "2026/08/14 09:30")).unwrap();
        mb.put(Folder::Inbox, &msg("CCCCCCCCCCCC", "2026/08/13 23:59")).unwrap();

        let mids: Vec<String> =
            mb.list(Folder::Inbox).unwrap().into_iter().map(|m| m.mid).collect();
        assert_eq!(mids, ["BBBBBBBBBBBB", "CCCCCCCCCCCC", "AAAAAAAAAAAA"]);
    }

    #[test]
    fn folders_are_separate() {
        let dir = TempDir::new("sep");
        let mb = Mailbox::open(&dir.0).unwrap();
        let m = msg("ABCDEFGHIJKL", "2026/08/14 09:30");
        mb.put(Folder::Outbox, &m).unwrap();
        assert!(mb.contains(Folder::Outbox, &m.mid));
        assert!(!mb.contains(Folder::Inbox, &m.mid));
    }

    #[test]
    fn moves_between_folders() {
        let dir = TempDir::new("move");
        let mb = Mailbox::open(&dir.0).unwrap();
        let m = msg("ABCDEFGHIJKL", "2026/08/14 09:30");
        mb.put(Folder::Outbox, &m).unwrap();

        mb.move_to(Folder::Outbox, Folder::Sent, &m.mid).unwrap();
        assert!(!mb.contains(Folder::Outbox, &m.mid));
        assert_eq!(mb.get(Folder::Sent, &m.mid).unwrap(), m);
    }

    #[test]
    fn removing_something_absent_is_fine() {
        let dir = TempDir::new("rm");
        let mb = Mailbox::open(&dir.0).unwrap();
        assert!(mb.remove(Folder::Inbox, "NOSUCHMESSAG").is_ok());
    }

    #[test]
    fn rejects_ids_that_would_escape_the_folder() {
        // A MID is chosen by whoever sent the message and arrives over a
        // socket. Every one of these must be refused rather than resolved.
        let dir = TempDir::new("escape");
        let mb = Mailbox::open(&dir.0).unwrap();
        let long = "A".repeat(MAX_ID_LEN + 1);
        for bad in
            ["../../etc/passwd", "..", ".hidden", "a/b", "a\\b", "", "a.b", "a\0b", "ü", &long]
        {
            assert!(mb.path_for(Folder::Inbox, bad).is_none(), "{bad:?} should be refused");
            assert!(matches!(
                mb.get(Folder::Inbox, bad),
                Err(MailboxError::BadId(_)) | Err(MailboxError::NotFound(..))
            ));
            assert!(!mb.contains(Folder::Inbox, bad));
        }
    }

    #[test]
    fn an_unreadable_file_does_not_break_the_listing() {
        let dir = TempDir::new("corrupt");
        let mb = Mailbox::open(&dir.0).unwrap();
        mb.put(Folder::Inbox, &msg("ABCDEFGHIJKL", "2026/08/14 09:30")).unwrap();
        std::fs::write(dir.0.join("inbox").join("GARBAGE.b2f"), b"not a message").unwrap();

        // One good message still lists; the junk is skipped, not fatal.
        assert_eq!(mb.list(Folder::Inbox).unwrap().len(), 1);
    }

    #[test]
    fn leaves_no_temp_files_behind() {
        let dir = TempDir::new("tmp");
        let mb = Mailbox::open(&dir.0).unwrap();
        mb.put(Folder::Inbox, &msg("ABCDEFGHIJKL", "2026/08/14 09:30")).unwrap();
        let strays: Vec<_> = std::fs::read_dir(dir.0.join("inbox"))
            .unwrap()
            .flatten()
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("tmp"))
            .collect();
        assert!(strays.is_empty(), "temporary files left behind: {strays:?}");
    }
}
