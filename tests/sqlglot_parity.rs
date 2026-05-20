use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;
use sqlgrok::{Dialect, transpile};

const SMOKE_CASES: &str = include_str!("../parity/cases/smoke.jsonl");

#[derive(Debug, Deserialize)]
struct ParityCase {
    id: String,
    sql: String,
    read: String,
    write: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    mode: Option<String>,
    #[serde(default)]
    skip_reason: Option<String>,
    #[serde(default)]
    accepted_rust: Option<String>,
    #[serde(default)]
    note: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PythonOracle {
    ok: bool,
    sql: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Default)]
struct ParitySummary {
    selected: usize,
    checked: usize,
    exact_matches: usize,
    accepted_divergences: usize,
    skipped: usize,
}

#[test]
fn sqlglot_python_smoke_parity() {
    let Some(sqlglot_path) = python_sqlglot_path() else {
        eprintln!(
            "skipping SQLGlot parity smoke: set SQLGLOT_PYTHON_PATH or clone Python SQLGlot beside sqlgrok"
        );
        return;
    };

    let filters = ParityFilters::from_env();
    let cases = parse_cases(SMOKE_CASES);
    assert!(!cases.is_empty(), "parity smoke corpus should not be empty");

    let mut summary = ParitySummary::default();
    for case in cases {
        if !filters.matches(&case) {
            continue;
        }

        summary.selected += 1;

        if let Some(mode) = &case.mode {
            assert_eq!(
                mode, "transpile",
                "{}: unsupported parity mode {mode:?}",
                case.id
            );
        }

        if let Some(reason) = &case.skip_reason {
            eprintln!("skipping {}: {reason}", case.id);
            summary.skipped += 1;
            continue;
        }

        let python = python_transpile(&sqlglot_path, &case)
            .unwrap_or_else(|err| panic!("{}: Python SQLGlot oracle failed: {err}", case.id));
        assert!(
            python.ok,
            "{}: Python SQLGlot returned error: {}",
            case.id,
            python.error.unwrap_or_default()
        );
        let expected = python.sql.expect("oracle success should include SQL");

        let read = Dialect::from_str(&case.read)
            .unwrap_or_else(|| panic!("{}: unknown read dialect {}", case.id, case.read));
        let write = Dialect::from_str(&case.write)
            .unwrap_or_else(|| panic!("{}: unknown write dialect {}", case.id, case.write));
        let rust = transpile(&case.sql, read, write)
            .unwrap_or_else(|err| panic!("{}: sqlgrok failed: {err}", case.id));

        summary.checked += 1;

        if let Some(accepted) = &case.accepted_rust {
            if rust != *accepted {
                panic!(
                    "{}: known-divergence output changed{}\nsource: {}\nread: {}\nwrite: {}\nsql: {}\nexpected accepted Rust: {}\nactual Rust: {}",
                    case.id,
                    case.note
                        .as_deref()
                        .map(|note| format!(" ({note})"))
                        .unwrap_or_default(),
                    case.source.as_deref().unwrap_or("<unspecified>"),
                    case.read,
                    case.write,
                    case.sql,
                    accepted,
                    rust
                );
            }
            summary.accepted_divergences += 1;
        } else {
            if rust != expected {
                panic!(
                    "{}: sqlgrok output differs from SQLGlot\nsource: {}\nread: {}\nwrite: {}\ntags: {}\nsql: {}\npython SQLGlot: {}\nsqlgrok: {}",
                    case.id,
                    case.source.as_deref().unwrap_or("<unspecified>"),
                    case.read,
                    case.write,
                    case.tags.join(","),
                    case.sql,
                    expected,
                    rust
                );
            }
            summary.exact_matches += 1;
        }
    }

    assert!(
        summary.selected > 0,
        "no SQLGlot parity cases matched filters: {:?}",
        filters
    );

    eprintln!(
        "SQLGlot parity summary: selected={}, checked={}, exact_matches={}, accepted_divergences={}, skipped={}",
        summary.selected,
        summary.checked,
        summary.exact_matches,
        summary.accepted_divergences,
        summary.skipped
    );
}

#[derive(Debug, Default)]
struct ParityFilters {
    id: Option<String>,
    tag: Option<String>,
    read: Option<String>,
    write: Option<String>,
}

impl ParityFilters {
    fn from_env() -> Self {
        Self {
            id: env::var("SQLGROK_PARITY_ID").ok(),
            tag: env::var("SQLGROK_PARITY_TAG").ok(),
            read: env::var("SQLGROK_PARITY_READ").ok(),
            write: env::var("SQLGROK_PARITY_WRITE").ok(),
        }
    }

    fn matches(&self, case: &ParityCase) -> bool {
        if self.id.as_deref().is_some_and(|id| id != case.id) {
            return false;
        }
        if self
            .tag
            .as_deref()
            .is_some_and(|tag| !case.tags.iter().any(|case_tag| case_tag == tag))
        {
            return false;
        }
        if self.read.as_deref().is_some_and(|read| read != case.read) {
            return false;
        }
        if self
            .write
            .as_deref()
            .is_some_and(|write| write != case.write)
        {
            return false;
        }
        true
    }
}

fn parse_cases(input: &str) -> Vec<ParityCase> {
    let cases: Vec<ParityCase> = input
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .map(|line| serde_json::from_str(line).expect("valid parity case JSON"))
        .collect();
    assert_unique_ids(&cases);
    cases
}

fn assert_unique_ids(cases: &[ParityCase]) {
    let mut ids = HashSet::new();
    for case in cases {
        assert!(
            ids.insert(&case.id),
            "duplicate parity case id: {}",
            case.id
        );
        for tag in &case.tags {
            assert!(
                is_valid_tag(tag),
                "{}: invalid parity tag {:?}; use lowercase kebab-case",
                case.id,
                tag
            );
        }
    }
}

fn is_valid_tag(tag: &str) -> bool {
    !tag.is_empty()
        && tag
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn python_sqlglot_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("SQLGLOT_PYTHON_PATH") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }

    let sibling = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|parent| parent.join("sqlglot"))?;
    sibling.exists().then_some(sibling)
}

fn python_transpile(path: &PathBuf, case: &ParityCase) -> Result<PythonOracle, String> {
    let script = r#"
import json
import sys

import sqlglot

sql, read, write = sys.argv[1:4]
try:
    out = sqlglot.transpile(sql, read=read, write=write)[0]
    print(json.dumps({"ok": True, "sql": out}))
except Exception as exc:
    print(json.dumps({"ok": False, "error": str(exc)}))
"#;

    let output = Command::new("python3")
        .arg("-c")
        .arg(script)
        .arg(&case.sql)
        .arg(&case.read)
        .arg(&case.write)
        .env("PYTHONPATH", path)
        .output()
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "python3 exited with {}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|err| {
        format!(
            "invalid oracle JSON: {err}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}
