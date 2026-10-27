//! Every language is complete: every text enum that has an `english()` has an arm for each variant
//! in `german()`, `spanish()`, `french()` and `portuguese()`.
//!
//! Those return `Option` and end in `_ => return None` so that a build never breaks on a text
//! someone adds without a translation; this test is what catches it, so no text reaches a player
//! in English by accident. It reads the sources, so a new `*_strings.rs` is covered without being
//! listed. Adding a language works the same way: add its method name to `TRANSLATED`.

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

/// The languages that translate through a method named after them next to `english()`.
const TRANSLATED: [&str; 4] = ["german", "spanish", "french", "portuguese"];

#[test]
fn every_text_has_an_arm_in_every_language() {
    let mut files = Vec::new();
    sources(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/screens"),
        &mut files,
    );
    let mut checked = 0;
    let mut missing = Vec::new();
    for file in files {
        let source = fs::read_to_string(&file).expect("a source file");
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        let Some(english) = function(&source, "english") else {
            continue;
        };
        let english = arms(english);
        for language in TRANSLATED {
            let Some(translated) = function(&source, language) else {
                missing.push(format!("{name}: no fn {language}"));
                continue;
            };
            let translated = arms(translated);
            for arm in &english {
                checked += 1;
                if !translated.contains(arm) {
                    missing.push(format!("{name}: {arm} ({language})"));
                }
            }
        }
    }
    assert!(checked > 1500, "the scan found only {checked} texts");
    assert!(
        missing.is_empty(),
        "no text for:
{}",
        missing.join(
            "
"
        )
    );
}
