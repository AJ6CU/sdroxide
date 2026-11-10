//! The interface is English only. A text scan of the non-test part of every
//! UI source file: no string literal may carry a French or Dutch accented
//! letter or a known foreign word. Test fixtures (after `#[cfg(test)]`) and
//! comments are out of scope: they hold real-world sample data on purpose.

use std::path::{Path, PathBuf};

const ACCENTED: &str = "àâçéèêëîïôûùüÿœÀÂÇÉÈÊËÎÏÔÛÙÜŒ";
const FOREIGN: &[&str] = &[
    "copié",
    "enregistrer",
    "annuler",
    "fenêtre",
    "paramètres",
    "kopiëren",
    "indisponible",
    "chargement",
    "aucun ",
    "introuvable",
];

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

fn literals(line: &str) -> Vec<&str> {
    let mut v = Vec::new();
    let mut rest = line;
    while let Some(a) = rest.find('"') {
        let tail = &rest[a + 1..];
        let mut end = None;
        let mut esc = false;
        for (i, c) in tail.char_indices() {
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                end = Some(i);
                break;
            }
        }
        match end {
            Some(i) => {
                v.push(&tail[..i]);
                rest = &tail[i + 1..];
            }
            None => break,
        }
    }
    v
}

#[test]
fn no_foreign_text_in_ui_strings() {
    let mut files = Vec::new();
    sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut files);
    assert!(files.len() > 50, "the scan found the sources");
    let mut bad = Vec::new();
    for f in &files {
        let name = f.file_name().unwrap().to_string_lossy();
        if name.ends_with("_tests.rs") || name == "test_support.rs" {
            continue;
        }
        // `i18n.rs` holds the **pseudo language**, whose whole job is to turn
        // ASCII into accented characters so a string that bypassed `tr!` stands
        // out on screen and in a diff. Its literals are the one place in the
        // interface where non-English text is correct.
        if name == "i18n.rs" {
            continue;
        }
        let text = std::fs::read_to_string(f).unwrap();
        for (n, line) in text.lines().enumerate() {
            if line.trim() == "#[cfg(test)]" {
                break;
            }
            if line.trim_start().starts_with("//") {
                continue;
            }
            for s in literals(line) {
                let low = s.to_lowercase();
                if s.chars().any(|c| ACCENTED.contains(c))
                    || FOREIGN.iter().any(|w| low.contains(w))
                {
                    bad.push(format!("{}:{}: {s}", f.display(), n + 1));
                }
            }
        }
    }
    assert!(bad.is_empty(), "non-English UI text:\n{}", bad.join("\n"));
}

/// A browser that translates the page by itself turns fixed English labels
/// into a mix of two languages, so the page must opt out.
#[test]
fn the_web_page_declares_english_and_opts_out_of_translation() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../sdroxide-web/index.html");
    let html = std::fs::read_to_string(&path).expect("read index.html");
    assert!(html.contains("<html lang=\"en\" translate=\"no\">"), "html element");
    assert!(html.contains("name=\"google\" content=\"notranslate\""), "notranslate meta");
}
