//! **An HFCC package handed over by hand is read like the download.**
//!
//! HFCC publishes one zip per season (`b26allx2.zip`): the schedule
//! (`B26all00.TXT`) and the two tables that turn its codes into names
//! (`broadcas.txt`, `site.txt`). The fixtures are the real files, the schedule cut
//! to its first 600 rows. What is pinned here is the path the file takes: it is
//! recognised as HFCC by what is inside, it lands in the cache where the reader
//! looks, and the names come out.

use sdroxide_config::{ImportedFile, cached_schedule_for, import_any_file};
use sdroxide_types::ScheduleSource;

const SCHEDULE: &[u8] = include_bytes!("data/hfcc/B26all00.TXT");
const BROADCASTERS: &[u8] = include_bytes!("data/hfcc/broadcas.txt");
const SITES: &[u8] = include_bytes!("data/hfcc/site.txt");

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
        std::env::temp_dir().join(format!("sdroxide-hfcc-import-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch dir");
    // SAFETY: one test binary, one process, set before anything reads it.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    root
}

#[test]
fn an_hfcc_package_imported_by_hand_is_the_schedule_with_its_names() {
    let _turn = one_at_a_time();
    let root = sandbox("package");
    let file = root.join("b26allx2.zip");
    std::fs::write(
        &file,
        stored_zip(&[
            ("B26all00.TXT", SCHEDULE),
            ("broadcas.txt", BROADCASTERS),
            ("site.txt", SITES),
        ]),
    )
    .expect("fixture");

    match import_any_file(&file).expect("the package is a schedule") {
        ImportedFile::Schedule(got) => {
            assert_eq!(got.source, ScheduleSource::Hfcc, "read as HFCC, not Aoki: {got:?}");
            assert!(got.stations > 400, "the rows parse: {got:?}");
        }
        other => panic!("read as a list instead of a schedule: {other:?}"),
    }

    // The proof that matters is the **reader**: what the app reads at startup.
    let read = cached_schedule_for(ScheduleSource::Hfcc).expect("the cache is read back");
    let named = read.iter().filter(|r| r.name.len() > 3).count();
    assert!(named * 10 >= read.len() * 8, "the names resolve: {named} of {}", read.len());

    let _ = std::fs::remove_dir_all(&root);
}

/// A schedule without its tables would be a list of `WCR` and `BRC` codes, which
/// is worse than none: it is refused, and nothing is cached.
#[test]
fn an_hfcc_schedule_without_its_tables_is_refused() {
    let _turn = one_at_a_time();
    let root = sandbox("no-tables");
    let file = root.join("b26allx2.zip");
    std::fs::write(
        &file,
        stored_zip(&[("B26all00.TXT", SCHEDULE), ("broadcas.txt", BROADCASTERS)]),
    )
    .expect("fixture");

    assert!(import_any_file(&file).is_err(), "half a package is not a schedule");
    assert!(cached_schedule_for(ScheduleSource::Hfcc).is_none(), "and nothing was cached");

    let _ = std::fs::remove_dir_all(&root);
}
