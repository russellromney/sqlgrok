use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn sqlgrok() -> Command {
    Command::cargo_bin("sqlgrok").unwrap()
}

// ─── Transpile command ──────────────────────────────────────────────────────

#[test]
fn transpile_stdin_to_stdout() {
    sqlgrok()
        .args(["transpile", "--read", "mysql", "--write", "postgres"])
        .write_stdin("SELECT CAST(x AS INT) FROM t")
        .assert()
        .success()
        .stdout(predicate::str::contains("SELECT x::INT FROM t"));
}

#[test]
fn transpile_pretty() {
    sqlgrok()
        .args(["transpile", "--pretty"])
        .write_stdin("SELECT a, b FROM t WHERE x > 1")
        .assert()
        .success()
        .stdout(predicate::str::contains("SELECT\n"));
}

#[test]
fn transpile_with_optimize() {
    sqlgrok()
        .args(["transpile", "--optimize"])
        .write_stdin("SELECT * FROM t WHERE 1 = 1 AND a > 5")
        .assert()
        .success()
        .stdout(predicate::str::contains("a > 5"))
        .stdout(predicate::str::contains("1 = 1").not());
}

#[test]
fn transpile_from_file() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "SELECT 1").unwrap();

    sqlgrok()
        .args(["transpile", "--input", f.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("SELECT 1"));
}

#[test]
fn transpile_to_file() {
    let out = NamedTempFile::new().unwrap();
    let out_path = out.path().to_str().unwrap().to_string();
    // Close so the CLI can write to it.
    drop(out);

    sqlgrok()
        .args(["transpile", "--output", &out_path])
        .write_stdin("SELECT 42")
        .assert()
        .success();

    let content = std::fs::read_to_string(&out_path).unwrap();
    assert!(content.contains("SELECT 42"));
    std::fs::remove_file(&out_path).ok();
}

#[test]
fn transpile_unknown_dialect_fails() {
    sqlgrok()
        .args(["transpile", "--read", "nosuchdialect"])
        .write_stdin("SELECT 1")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown dialect"));
}

// ─── Parse command ──────────────────────────────────────────────────────────

#[test]
fn parse_outputs_json() {
    sqlgrok()
        .args(["parse"])
        .write_stdin("SELECT a FROM t")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Select\""));
}

#[test]
fn parse_pretty_json() {
    sqlgrok()
        .args(["parse", "--pretty"])
        .write_stdin("SELECT a FROM t")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Select\": {"));
}

#[test]
fn parse_valid_json() {
    let output = sqlgrok()
        .args(["parse"])
        .write_stdin("SELECT 1")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout.trim()).unwrap();
    assert!(parsed.is_array());
}

// ─── Format command ─────────────────────────────────────────────────────────

#[test]
fn format_pretty_prints() {
    sqlgrok()
        .args(["format"])
        .write_stdin("select a,b from t where x>1")
        .assert()
        .success()
        .stdout(predicate::str::contains("SELECT\n"))
        .stdout(predicate::str::contains("FROM\n"));
}

#[test]
fn format_from_file() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "select a from t").unwrap();

    sqlgrok()
        .args(["format", "--input", f.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("SELECT\n"));
}

// ─── Help and version ───────────────────────────────────────────────────────

#[test]
fn help_flag() {
    sqlgrok()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("transpile"))
        .stdout(predicate::str::contains("parse"))
        .stdout(predicate::str::contains("format"));
}

#[test]
fn version_flag() {
    sqlgrok()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("sqlgrok"));
}

// ─── Multiple statements ────────────────────────────────────────────────────

#[test]
fn transpile_multiple_statements() {
    sqlgrok()
        .args(["transpile"])
        .write_stdin("SELECT 1; SELECT 2")
        .assert()
        .success()
        .stdout(predicate::str::contains("SELECT 1"))
        .stdout(predicate::str::contains("SELECT 2"));
}

// ─── All dialects round-trip ────────────────────────────────────────────────

#[test]
fn transpile_all_dialects() {
    let dialects = [
        "ansi",
        "athena",
        "bigquery",
        "clickhouse",
        "databricks",
        "duckdb",
        "hive",
        "mysql",
        "oracle",
        "postgres",
        "presto",
        "redshift",
        "snowflake",
        "spark",
        "sqlite",
        "starrocks",
        "trino",
        "tsql",
        "doris",
        "dremio",
        "drill",
        "druid",
        "exasol",
        "fabric",
        "materialize",
        "risingwave",
        "singlestore",
        "tableau",
        "teradata",
    ];

    for dialect in dialects {
        sqlgrok()
            .args(["transpile", "--read", dialect, "--write", "ansi"])
            .write_stdin("SELECT 1")
            .assert()
            .success();
    }
}
