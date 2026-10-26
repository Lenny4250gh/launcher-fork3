//! German is complete: every text enum that has an `english()` has a German arm for each variant.
//!
//! `german()` returns `Option` and ends in `_ => return None` so that a build never breaks on a
//! text someone adds without German; this test is what catches it. It reads the sources, so a
//! new `*_strings.rs` is covered without being listed. Adding a language works the same way.

use std::fs;
use std::path::{Path, PathBuf};

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("the screens folder") {
        let path = entry.expect("an entry").path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The text of the function `name` (from its signature to the next function at the same depth).
fn function<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let at = source.find(&format!("fn {name}(self"))?;
    let rest = &source[at..];
    let end = rest[1..]
        .find("\n    pub ")
        .or_else(|| rest[1..].find("\n    fn "))
        .or_else(|| rest[1..].find("\n    const fn "))
        .or_else(|| rest[1..].find("\n}"))
        .map_or(rest.len(), |e| e + 1);
    Some(&rest[..end])
}

/// Every `Enum::Variant` named in the head of an arm (`A::X | A::Y =>` names two).
fn arms(body: &str) -> Vec<String> {
    let mut found = Vec::new();
    for line in body.lines() {
        let line = line.trim_start();
        let starts_an_arm = line.starts_with('|')
            || line.split_once("::").is_some_and(|(head, _)| {
                head.chars().next().is_some_and(char::is_uppercase)
                    && head.chars().all(|c| c.is_alphanumeric() || c == '_')
            });
        if !starts_an_arm {
            continue;
        }
        let head = line.split("=>").next().unwrap_or(line);
        for part in head.split('|') {
            let part = part.trim();
            let Some((en, variant)) = part.split_once("::") else {
                continue;
            };
            let variant: String = variant
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if !en.is_empty()
                && en.chars().all(|c| c.is_alphanumeric() || c == '_')
                && !variant.is_empty()
            {
                found.push(format!("{en}::{variant}"));
            }
        }
    }
    found
}

#[test]
fn every_text_has_a_german_arm() {
    let mut files = Vec::new();
    sources(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/screens"),
        &mut files,
    );
    let mut checked = 0;
    let mut missing = Vec::new();
    for file in files {
        let source = fs::read_to_string(&file).expect("a source file");
        let (Some(english), Some(german)) =
            (function(&source, "english"), function(&source, "german"))
        else {
            continue;
        };
        let german = arms(german);
        for arm in arms(english) {
            checked += 1;
            if !german.contains(&arm) {
                missing.push(format!(
                    "{}: {arm}",
                    file.file_name().unwrap().to_string_lossy()
                ));
            }
        }
    }
    assert!(checked > 500, "the scan found only {checked} texts");
    assert!(
        missing.is_empty(),
        "no German text for:\n{}",
        missing.join("\n")
    );
}
