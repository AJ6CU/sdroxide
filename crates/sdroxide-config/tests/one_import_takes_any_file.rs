//! **One import button takes whatever file the operator has.**
//!
//! "Importer ma liste" used to read the file as a listener's own list through a
//! strict UTF-8 read, so EiBi's `sked-b26.csv` — Latin-1, and a published
//! schedule, not a list — was refused as unreadable. `import_any_file` sends a
//! schedule down the schedule path and anything else down the list path, and the
//! list path reads Latin-1 too.

use sdroxide_config::{ImportedFile, import_any_file};
use sdroxide_types::ScheduleSource;

/// EiBi's header and one real-shaped row (Latin-1 `é` in the name, CRLF line
/// ends, as the published file has), repeated past the 500-row floor.
fn eibi_latin1() -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(
        b"kHz:75;Time(UTC):93;Days:59;ITU:49;Station:201;Lng:49;Target:62;Remarks:135;P:35;Start:60;Stop:60;\r\n",
    );
    for _ in 0..600 {
        out.extend_from_slice(b"5900;1930-1945;;BUL;Radio Dardasha\xe9;A;ME;;1;;\r\n");
    }
    out
}

/// `SDROXIDE_CONFIG_DIR` is process-global: the sandboxes take turns.
static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn one_at_a_time() -> std::sync::MutexGuard<'static, ()> {
    ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner())
}

fn sandbox(tag: &str) -> std::path::PathBuf {
    let root =
        std::env::temp_dir().join(format!("sdroxide-import-any-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch dir");
    // SAFETY: one test binary, one process, set before anything reads it.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    root
}

#[test]
fn an_eibi_csv_handed_to_the_one_button_is_a_schedule() {
    let _turn = one_at_a_time();
    let root = sandbox("eibi");
    let file = root.join("sked-b26.csv");
    std::fs::write(&file, eibi_latin1()).expect("fixture");
    match import_any_file(&file).expect("an EiBi file is accepted") {
        ImportedFile::Schedule(got) => {
            assert_eq!(got.source, ScheduleSource::Eibi);
            assert_eq!(got.stations, 600);
        }
        other => panic!("read as a list instead of a schedule: {other:?}"),
    }
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_latin1_own_list_is_read_not_refused() {
    let _turn = one_at_a_time();
    let root = sandbox("list");
    let file = root.join("mes-stations.csv");
    // "Radio Dardasha\xe9" in Latin-1 is not valid UTF-8.
    std::fs::write(&file, b"5900,Radio Dardasha\xe9\n6070,Radio Caroline\n").expect("fixture");
    assert_eq!(import_any_file(&file), Ok(ImportedFile::List(2)));
    let _ = std::fs::remove_dir_all(&root);
}
