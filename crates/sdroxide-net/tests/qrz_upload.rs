//! Live QRZ Logbook upload round-trip: INSERT a throwaway QSO with our real
//! upload path, then DELETE it so nothing is left behind. The API key is read
//! from the `QRZ_KEY` env var (never committed); the test is skipped if unset.
//!
//!   QRZ_KEY=XXXX-XXXX-XXXX-XXXX MY_CALL=OE3JJS cargo test -p sdroxide-net --test qrz_upload -- --ignored --nocapture

use sdroxide_net::upload_qso;
use sdroxide_types::{NetworkConfig, QsoRecord, UploadTarget, qso_log_to_adif};

#[test]
#[ignore = "uploads to the live QRZ logbook (needs QRZ_KEY and MY_CALL)"]
fn qrz_insert_then_delete() {
    let Ok(key) = std::env::var("QRZ_KEY") else {
        eprintln!("QRZ_KEY not set — skipping");
        return;
    };
    let Ok(my_call) = std::env::var("MY_CALL") else {
        eprintln!("MY_CALL not set — skipping");
        return;
    };

    // A clearly-marked test QSO on today's date.
    let rec = QsoRecord {
        call: "TE1ST".into(),
        rst_sent: Some(59),
        rst_rcvd: Some(59),
        freq_hz: 14_250_000.0,
        mode: "SSB".into(),
        band: "20m".into(),
        start_utc: 1_700_000_000, // 2023-11-14, fixed so it's easy to spot/remove
        end_utc: 1_700_000_000,
        my_call: my_call.clone(),
        comment: "sdroxide upload test — safe to delete".into(),
        ..Default::default()
    };
    let adif = qso_log_to_adif(std::slice::from_ref(&rec));

    let cfg = NetworkConfig { qrz_logbook_key: key.clone(), ..Default::default() };
    // The operator callsign is only used by the Club Log target, which this
    // test does not exercise.
    let result = upload_qso(&cfg, &my_call, UploadTarget::QrzLogbook, &adif);
    println!("QRZ INSERT result: {result:?}");
    assert!(result.is_ok(), "QRZ upload failed: {result:?}");

    // Now delete it so the account is left clean. Fetch the LOGID(s) for the
    // test call and delete them.
    let logids = qrz_logids_for(&key, "TE1ST");
    println!("QRZ LOGIDs for TE1ST: {logids:?}");
    for id in &logids {
        let del = qrz_delete(&key, id);
        println!("QRZ DELETE {id}: {del:?}");
        assert!(del.is_ok(), "QRZ delete failed: {del:?}");
    }
    assert!(!logids.is_empty(), "no LOGID found to delete — manual cleanup may be needed");
}

/// Query the logbook for a call and return the matching LOGIDs.
fn qrz_logids_for(key: &str, call: &str) -> Vec<String> {
    let body = ureq::post("https://logbook.qrz.com/api")
        .send_form([("KEY", key), ("ACTION", "FETCH"), ("OPTION", &format!("CALL:{call}"))])
        .and_then(|mut r| r.body_mut().read_to_string())
        .unwrap_or_default();
    // The FETCH response embeds ADIF with HTML-entity-encoded tags, e.g.
    // `&lt;app_qrzlog_logid:10&gt;1482246002`. Pull the digits after each.
    let mut out = Vec::new();
    let low = body.to_ascii_lowercase();
    let mut idx = 0;
    while let Some(pos) = low[idx..].find("app_qrzlog_logid:") {
        let start = idx + pos;
        if let Some(gt) = low[start..].find("gt;") {
            let after = start + gt + 3;
            let end = body[after..]
                .find(|c: char| !c.is_ascii_digit())
                .map(|e| after + e)
                .unwrap_or(body.len());
            if after < end {
                out.push(body[after..end].to_string());
            }
            idx = end;
        } else {
            break;
        }
    }
    out
}

fn qrz_delete(key: &str, logid: &str) -> Result<String, String> {
    ureq::post("https://logbook.qrz.com/api")
        .send_form([("KEY", key), ("ACTION", "DELETE"), ("LOGIDS", logid)])
        .map_err(|e| e.to_string())?
        .body_mut()
        .read_to_string()
        .map_err(|e| e.to_string())
}
