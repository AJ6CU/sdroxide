//! **Aoki's package holds the schedule and a user list; the schedule is the one
//! that is imported.**
//!
//! `nzb25.zip` holds `nzb25.txt` (the schedule), `userlist1.txt` (a listener's
//! list in another layout) and an `.xlsx`. Both `.txt` titles read "Shortwave
//! Frequency List", so picking the first title match took whichever came first in
//! the zip. The fixtures are the real files, header plus 700 rows between 5900 and 7500 kHz (the parser keeps only
//! short-wave bands), Shift-JIS bytes as published.

use sdroxide_config::{ImportedFile, import_any_file};
use sdroxide_types::ScheduleSource;

const SCHEDULE: &[u8] = include_bytes!("data/aoki/nzb25.txt");
const USER_LIST: &[u8] = include_bytes!("data/aoki/userlist1.txt");

/// A stored (uncompressed) zip of several entries.
fn stored_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut dir = Vec::new();
    for (name, body) in entries {
        let lho = out.len() as u32;
        out.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
        out.extend_from_slice(&20u16.to_le_bytes()); // version needed
        out.extend_from_slice(&0u16.to_le_bytes()); // flags
        out.extend_from_slice(&0u16.to_le_bytes()); // method: stored
        out.extend_from_slice(&[0, 0, 0, 0]); // time, date
        out.extend_from_slice(&0u32.to_le_bytes()); // crc (not read back)
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // extra length
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(body);

        dir.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        dir.extend_from_slice(&20u16.to_le_bytes()); // version made by
        dir.extend_from_slice(&20u16.to_le_bytes()); // version needed
        dir.extend_from_slice(&0u16.to_le_bytes()); // flags
        dir.extend_from_slice(&0u16.to_le_bytes()); // method
        dir.extend_from_slice(&[0, 0, 0, 0]); // time, date
        dir.extend_from_slice(&0u32.to_le_bytes()); // crc
        dir.extend_from_slice(&(body.len() as u32).to_le_bytes());
        dir.extend_from_slice(&(body.len() as u32).to_le_bytes());
        dir.extend_from_slice(&(name.len() as u16).to_le_bytes());
        dir.extend_from_slice(&[0u8; 8]); // extra, comment, disk-start, internal attrs
        dir.extend_from_slice(&0u32.to_le_bytes()); // external attributes
        dir.extend_from_slice(&lho.to_le_bytes());
        dir.extend_from_slice(name.as_bytes());
    }
    let cen = out.len() as u32;
    let cen_len = dir.len() as u32;
    let count = entries.len() as u16;
    out.extend_from_slice(&dir);
    out.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // disk number
    out.extend_from_slice(&0u16.to_le_bytes()); // disk with the directory
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&cen_len.to_le_bytes());
    out.extend_from_slice(&cen.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // comment length
    out
}

/// `SDROXIDE_CONFIG_DIR` is process-global: the sandboxes take turns.
static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn one_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner())
}

fn sandbox(tag: &str) -> std::path::PathBuf {
    let root =
        std::env::temp_dir().join(format!("sdroxide-aoki-package-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch dir");
    // SAFETY: one test binary, one process, set before anything reads it.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    root
}

#[test]
fn the_schedule_is_picked_even_when_the_user_list_comes_first() {
    let _turn = one_at_a_time();
    let root = sandbox("order");

    // The schedule alone: what the package must give, whatever its order.
    let txt = root.join("nzb25.txt");
    std::fs::write(&txt, SCHEDULE).expect("fixture");
    let alone = match import_any_file(&txt).expect("the .txt is a schedule") {
        ImportedFile::Schedule(got) => {
            assert_eq!(got.source, ScheduleSource::Aoki);
            got.stations
        }
        other => panic!("read as a list: {other:?}"),
    };
    assert!(alone > 400, "the fixture is a schedule: {alone}");

    // The user list first in the zip, the schedule second.
    let zip = root.join("nzb25.zip");
    std::fs::write(
        &zip,
        stored_zip(&[("userlist1.txt", USER_LIST), ("nzb25.txt", SCHEDULE)]),
    )
    .expect("fixture");
    match import_any_file(&zip).expect("the package is accepted") {
        ImportedFile::Schedule(got) => {
            assert_eq!(got.source, ScheduleSource::Aoki);
            assert_eq!(got.stations, alone, "the schedule was read, not the user list");
        }
        other => panic!("read as a list: {other:?}"),
    }

    let _ = std::fs::remove_dir_all(&root);
}
