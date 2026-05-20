use std::env;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;
use sqlgrok::{transpile, Dialect};

const SMOKE_CASES: &str = include_str!("../parity/cases/smoke.jsonl");

#[derive(Debug, Deserialize)]
struct ParityCase {
    id: String,
    sql: String,
    read: String,
    write: String,
    accepted_rust: Option<String>,
    note: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PythonOracle {
    ok: bool,
    sql: Option<String>,
    error: Option<String>,
}

#[test]
fn sqlglot_python_smoke_parity() {
    let Some(sqlglot_path) = python_sqlglot_path() else {
        eprintln!(
            "skipping SQLGlot parity smoke: set SQLGLOT_PYTHON_PATH or clone Python SQLGlot beside sqlgrok"
        );
        return;
    };

    let cases = parse_cases(SMOKE_CASES);
    assert!(!cases.is_empty(), "parity smoke corpus should not be empty");

    let mut checked = 0;
    for case in cases {
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

        if let Some(accepted) = case.accepted_rust {
            assert_eq!(
                rust, accepted,
                "{}: known-divergence output changed{}",
                case.id,
                case.note
                    .as_deref()
                    .map(|note| format!(" ({note})"))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(rust, expected, "{}: sqlgrok output differs from SQLGlot", case.id);
        }
        checked += 1;
    }

    eprintln!("checked {checked} SQLGlot parity smoke cases");
}

fn parse_cases(input: &str) -> Vec<ParityCase> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .map(|line| serde_json::from_str(line).expect("valid parity case JSON"))
        .collect()
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
