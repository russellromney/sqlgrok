use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};
use sqlgrok::{Dialect, transpile};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut raw_args = env::args().skip(1);
    let Some(command) = raw_args.next() else {
        return Err(usage());
    };

    match command.as_str() {
        "import-sqlglot-fixtures" => run_import(ImportArgs::parse(raw_args)?),
        "inventory-ast" => run_inventory(InventoryArgs::parse(raw_args)?),
        "summarize-report" => run_summarize_report(SummarizeReportArgs::parse(raw_args)?),
        "-h" | "--help" => Err(usage()),
        _ => Err(format!("unknown command {command:?}\n\n{}", usage())),
    }
}

fn usage() -> String {
    [
        ImportArgs::usage(),
        InventoryArgs::usage(),
        SummarizeReportArgs::usage(),
    ]
    .join("\n")
}

fn run_import(args: ImportArgs) -> Result<(), String> {
    args.validate()?;

    let raw_cases = import_sqlglot_fixtures(&args)?;
    let raw_count = raw_cases.len();
    let mut cases = if args.only_matching || args.report_output.is_some() {
        let outcomes = evaluate_cases(raw_cases, &args)?;
        if let Some(report_output) = &args.report_output {
            write_report(report_output, &outcomes)?;
        }
        filter_matching_outcomes(outcomes)
    } else {
        raw_cases
    };
    validate_cases(&cases)?;

    cases.sort_by(|left, right| left.id.cmp(&right.id));

    let jsonl = render_jsonl(&cases)?;
    eprintln!(
        "imported {} {} cases from {} (read={}, write={}, limit={}, dry_run={}, only_matching={}, raw_candidates={})",
        cases.len(),
        args.family,
        args.sqlglot.display(),
        args.read,
        args.write,
        args.limit,
        args.dry_run,
        args.only_matching,
        raw_count
    );

    if args.dry_run {
        print!("{jsonl}");
        return Ok(());
    }

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(&args.output, jsonl)
        .map_err(|err| format!("failed to write {}: {err}", args.output.display()))?;
    eprintln!("wrote {}", args.output.display());

    Ok(())
}

fn run_inventory(args: InventoryArgs) -> Result<(), String> {
    args.validate()?;

    let markdown = generate_ast_inventory(&args)?;
    if args.dry_run {
        print!("{markdown}");
        return Ok(());
    }

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(&args.output, markdown)
        .map_err(|err| format!("failed to write {}: {err}", args.output.display()))?;
    eprintln!("wrote {}", args.output.display());

    Ok(())
}

fn run_summarize_report(args: SummarizeReportArgs) -> Result<(), String> {
    args.validate()?;

    let summary = summarize_report(&args)?;
    if args.dry_run {
        print!("{summary}");
        return Ok(());
    }

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(&args.output, summary)
        .map_err(|err| format!("failed to write {}: {err}", args.output.display()))?;
    eprintln!("wrote {}", args.output.display());

    Ok(())
}

#[derive(Debug)]
struct ImportArgs {
    sqlglot: PathBuf,
    family: String,
    read: String,
    write: String,
    limit: usize,
    dry_run: bool,
    only_matching: bool,
    report_output: Option<PathBuf>,
    output: PathBuf,
}

impl ImportArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.peekable();
        let mut sqlglot = None;
        let mut family = None;
        let mut read = None;
        let mut write = None;
        let mut limit = 25;
        let mut dry_run = false;
        let mut only_matching = false;
        let mut report_output = None;
        let mut output = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--sqlglot" => sqlglot = Some(next_value(&mut args, "--sqlglot")?.into()),
                "--family" => family = Some(next_value(&mut args, "--family")?),
                "--read" => read = Some(next_value(&mut args, "--read")?),
                "--write" => write = Some(next_value(&mut args, "--write")?),
                "--limit" => {
                    let raw = next_value(&mut args, "--limit")?;
                    limit = raw.parse().map_err(|_| {
                        format!("--limit must be a non-negative integer, got {raw:?}")
                    })?;
                }
                "--all" => limit = 0,
                "--dry-run" => dry_run = true,
                "--only-matching" => only_matching = true,
                "--report-output" => {
                    report_output = Some(next_value(&mut args, "--report-output")?.into())
                }
                "--output" => output = Some(next_value(&mut args, "--output")?.into()),
                "-h" | "--help" => return Err(Self::usage()),
                _ => return Err(format!("unknown argument {arg:?}\n\n{}", Self::usage())),
            }
        }

        let sqlglot = sqlglot.ok_or_else(|| "--sqlglot is required".to_string())?;
        let family = family.unwrap_or_else(|| "transpile".to_string());
        let read = read.ok_or_else(|| "--read is required".to_string())?;
        let write = write.ok_or_else(|| "--write is required".to_string())?;
        let output = output.unwrap_or_else(|| {
            PathBuf::from("parity/cases").join(format!("{}_{}_{}.jsonl", family, read, write))
        });

        Ok(Self {
            sqlglot,
            family,
            read,
            write,
            limit,
            dry_run,
            only_matching,
            report_output,
            output,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if self.family != "transpile" {
            return Err(format!(
                "unsupported fixture family {:?}; currently only \"transpile\" is implemented",
                self.family
            ));
        }
        if self.read.trim().is_empty() {
            return Err("--read must not be empty".to_string());
        }
        if self.write.trim().is_empty() {
            return Err("--write must not be empty".to_string());
        }
        if !self.sqlglot.join("sqlglot/__init__.py").is_file() {
            return Err(format!(
                "{} does not look like a Python SQLGlot checkout",
                self.sqlglot.display()
            ));
        }
        if !self.sqlglot.join("tests").is_dir() {
            return Err(format!(
                "{} is missing the upstream tests directory",
                self.sqlglot.display()
            ));
        }
        Ok(())
    }

    fn usage() -> String {
        "usage: cargo run --bin xtask -- import-sqlglot-fixtures --sqlglot /path/to/sqlglot --family transpile --read mysql --write sqlite [--limit 25|--all] [--dry-run] [--only-matching] [--report-output parity/reports/mysql_sqlite.jsonl] [--output parity/cases/transpile_mysql_sqlite.jsonl]".to_string()
    }
}

#[derive(Debug)]
struct InventoryArgs {
    sqlglot: PathBuf,
    rust_ast: PathBuf,
    output: PathBuf,
    dry_run: bool,
}

impl InventoryArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.peekable();
        let mut sqlglot = None;
        let mut rust_ast = PathBuf::from("src/ast/types.rs");
        let mut output = PathBuf::from("docs/AST_INVENTORY.md");
        let mut dry_run = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--sqlglot" => sqlglot = Some(next_value(&mut args, "--sqlglot")?.into()),
                "--rust-ast" => rust_ast = next_value(&mut args, "--rust-ast")?.into(),
                "--output" => output = next_value(&mut args, "--output")?.into(),
                "--dry-run" => dry_run = true,
                "-h" | "--help" => return Err(Self::usage()),
                _ => return Err(format!("unknown argument {arg:?}\n\n{}", Self::usage())),
            }
        }

        let sqlglot = sqlglot.ok_or_else(|| "--sqlglot is required".to_string())?;
        Ok(Self {
            sqlglot,
            rust_ast,
            output,
            dry_run,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if !self.sqlglot.join("sqlglot/__init__.py").is_file() {
            return Err(format!(
                "{} does not look like a Python SQLGlot checkout",
                self.sqlglot.display()
            ));
        }
        let expressions_dir = self.sqlglot.join("sqlglot/expressions");
        if !expressions_dir.is_dir() {
            return Err(format!(
                "{} is missing the SQLGlot expressions package",
                expressions_dir.display()
            ));
        }
        if !self.rust_ast.is_file() {
            return Err(format!("{} does not exist", self.rust_ast.display()));
        }
        Ok(())
    }

    fn usage() -> String {
        "usage: cargo run --bin xtask -- inventory-ast --sqlglot /path/to/sqlglot [--rust-ast src/ast/types.rs] [--output docs/AST_INVENTORY.md] [--dry-run]".to_string()
    }
}

#[derive(Debug)]
struct SummarizeReportArgs {
    input: PathBuf,
    output: PathBuf,
    dry_run: bool,
}

impl SummarizeReportArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.peekable();
        let mut input: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut dry_run = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--input" => input = Some(next_value(&mut args, "--input")?.into()),
                "--output" => output = Some(next_value(&mut args, "--output")?.into()),
                "--dry-run" => dry_run = true,
                "-h" | "--help" => return Err(Self::usage()),
                _ => return Err(format!("unknown argument {arg:?}\n\n{}", Self::usage())),
            }
        }

        let input = input.ok_or_else(|| "--input is required".to_string())?;
        let output = output.unwrap_or_else(|| {
            let stem = input
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("report");
            PathBuf::from("parity/reports").join(format!("{stem}.md"))
        });

        Ok(Self {
            input,
            output,
            dry_run,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if !self.input.is_file() {
            return Err(format!("{} does not exist", self.input.display()));
        }
        Ok(())
    }

    fn usage() -> String {
        "usage: cargo run --bin xtask -- summarize-report --input parity/reports/transpile_mysql_sqlite.jsonl [--output parity/reports/transpile_mysql_sqlite.md] [--dry-run]".to_string()
    }
}

fn next_value(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
    flag: &str,
) -> Result<String, String> {
    let Some(value) = args.next() else {
        return Err(format!("{flag} requires a value"));
    };
    if value.starts_with("--") {
        return Err(format!("{flag} requires a value, got flag {value:?}"));
    }
    Ok(value)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct FixtureCase {
    id: String,
    sql: String,
    read: String,
    write: String,
    tags: Vec<String>,
    source: String,
    source_file: String,
    source_line: usize,
    test_name: String,
    mode: String,
}

#[derive(Debug, Deserialize)]
struct OracleOutput {
    ok: bool,
    sql: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CaseOutcome {
    #[serde(flatten)]
    case: FixtureCase,
    status: OutcomeStatus,
    expected: Option<String>,
    actual: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum OutcomeStatus {
    Match,
    Mismatch,
    RustError,
    OracleError,
}

fn import_sqlglot_fixtures(args: &ImportArgs) -> Result<Vec<FixtureCase>, String> {
    let output = Command::new("python3")
        .arg("-c")
        .arg(SQLGLOT_FIXTURE_IMPORTER)
        .arg(&args.sqlglot)
        .arg(&args.family)
        .arg(&args.read)
        .arg(&args.write)
        .arg(args.limit.to_string())
        .output()
        .map_err(|err| format!("failed to run python3 fixture importer: {err}"))?;

    if !output.status.success() {
        return Err(format!(
            "python3 fixture importer exited with {}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|err| {
        format!(
            "invalid fixture importer JSON: {err}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn evaluate_cases(cases: Vec<FixtureCase>, args: &ImportArgs) -> Result<Vec<CaseOutcome>, String> {
    let read = Dialect::from_str(&args.read)
        .ok_or_else(|| format!("unknown read dialect {:?}", args.read))?;
    let write = Dialect::from_str(&args.write)
        .ok_or_else(|| format!("unknown write dialect {:?}", args.write))?;

    let mut outcomes = Vec::new();
    let mut matched = 0usize;
    let mut rust_errors = 0usize;
    let mut oracle_errors = 0usize;
    let mut mismatches = 0usize;

    for case in cases {
        let oracle = python_oracle_transpile(&args.sqlglot, &case)?;
        if !oracle.ok {
            oracle_errors += 1;
            if let Some(error) = &oracle.error {
                eprintln!("{}: skipping oracle error: {error}", case.id);
            }
            outcomes.push(CaseOutcome {
                case,
                status: OutcomeStatus::OracleError,
                expected: None,
                actual: None,
                error: oracle.error,
            });
            continue;
        }
        let Some(expected) = oracle.sql else {
            oracle_errors += 1;
            outcomes.push(CaseOutcome {
                case,
                status: OutcomeStatus::OracleError,
                expected: None,
                actual: None,
                error: Some("oracle returned ok without sql".to_string()),
            });
            continue;
        };

        match transpile(&case.sql, read, write) {
            Ok(actual) if actual == expected => {
                matched += 1;
                outcomes.push(CaseOutcome {
                    case,
                    status: OutcomeStatus::Match,
                    expected: Some(expected),
                    actual: Some(actual),
                    error: None,
                });
            }
            Ok(actual) => {
                mismatches += 1;
                outcomes.push(CaseOutcome {
                    case,
                    status: OutcomeStatus::Mismatch,
                    expected: Some(expected),
                    actual: Some(actual),
                    error: None,
                });
            }
            Err(err) => {
                rust_errors += 1;
                outcomes.push(CaseOutcome {
                    case,
                    status: OutcomeStatus::RustError,
                    expected: Some(expected),
                    actual: None,
                    error: Some(err.to_string()),
                });
            }
        }
    }

    eprintln!(
        "only-matching filter: kept={}, mismatches={}, rust_errors={}, oracle_errors={}",
        matched, mismatches, rust_errors, oracle_errors
    );

    Ok(outcomes)
}

fn filter_matching_outcomes(outcomes: Vec<CaseOutcome>) -> Vec<FixtureCase> {
    outcomes
        .into_iter()
        .filter_map(|outcome| match outcome.status {
            OutcomeStatus::Match => Some(outcome.case),
            OutcomeStatus::Mismatch | OutcomeStatus::RustError | OutcomeStatus::OracleError => None,
        })
        .collect()
}

fn write_report(path: &PathBuf, outcomes: &[CaseOutcome]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    let mut output = String::new();
    for outcome in outcomes {
        let line = serde_json::to_string(outcome)
            .map_err(|err| format!("failed to serialize {}: {err}", outcome.case.id))?;
        output.push_str(&line);
        output.push('\n');
    }
    fs::write(path, output).map_err(|err| format!("failed to write {}: {err}", path.display()))?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

fn python_oracle_transpile(sqlglot: &PathBuf, case: &FixtureCase) -> Result<OracleOutput, String> {
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
        .env("PYTHONPATH", sqlglot)
        .output()
        .map_err(|err| format!("failed to run Python SQLGlot oracle: {err}"))?;

    if !output.status.success() {
        return Err(format!(
            "Python SQLGlot oracle exited with {}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|err| {
        format!(
            "invalid Python SQLGlot oracle JSON: {err}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn validate_cases(cases: &[FixtureCase]) -> Result<(), String> {
    if cases.is_empty() {
        return Err("no importable fixture cases matched the requested filters".to_string());
    }

    let mut ids = HashSet::new();
    for case in cases {
        if case.id.trim().is_empty() {
            return Err(format!("case with SQL {:?} has an empty id", case.sql));
        }
        if !ids.insert(&case.id) {
            return Err(format!("duplicate imported case id {:?}", case.id));
        }
        if case.sql.trim().is_empty() {
            return Err(format!("{}: sql must not be empty", case.id));
        }
        if case.read.trim().is_empty() || case.write.trim().is_empty() {
            return Err(format!(
                "{}: read/write dialects must not be empty",
                case.id
            ));
        }
        if case.mode != "transpile" {
            return Err(format!("{}: unsupported mode {:?}", case.id, case.mode));
        }
        for tag in &case.tags {
            if !is_valid_tag(tag) {
                return Err(format!(
                    "{}: invalid tag {:?}; use lowercase kebab-case",
                    case.id, tag
                ));
            }
        }
    }

    Ok(())
}

fn is_valid_tag(tag: &str) -> bool {
    !tag.is_empty()
        && tag
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn render_jsonl(cases: &[FixtureCase]) -> Result<String, String> {
    let mut output = String::new();
    for case in cases {
        let line = serde_json::to_string(case)
            .map_err(|err| format!("failed to serialize {}: {err}", case.id))?;
        output.push_str(&line);
        output.push('\n');
    }
    Ok(output)
}

fn summarize_report(args: &SummarizeReportArgs) -> Result<String, String> {
    let text = fs::read_to_string(&args.input)
        .map_err(|err| format!("failed to read {}: {err}", args.input.display()))?;
    let mut outcomes = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let outcome: CaseOutcome = serde_json::from_str(line).map_err(|err| {
            format!(
                "{}:{}: invalid report JSON: {err}",
                args.input.display(),
                index + 1
            )
        })?;
        outcomes.push(outcome);
    }

    let mut status_counts = BTreeMap::<String, usize>::new();
    let mut feature_counts = BTreeMap::<(String, String), usize>::new();
    let mut source_counts = BTreeMap::<(String, String, String), usize>::new();
    let mut examples = BTreeMap::<String, Vec<&CaseOutcome>>::new();

    for outcome in &outcomes {
        let status = outcome.status.as_str().to_string();
        *status_counts.entry(status.clone()).or_default() += 1;
        *feature_counts
            .entry((status.clone(), feature_key(&outcome.case.sql)))
            .or_default() += 1;
        *source_counts
            .entry((
                status.clone(),
                outcome.case.source_file.clone(),
                outcome.case.test_name.clone(),
            ))
            .or_default() += 1;
        if status != "match" {
            let bucket = examples.entry(status).or_default();
            if bucket.len() < 5 {
                bucket.push(outcome);
            }
        }
    }

    let mut output = String::new();
    output.push_str("# SQLGlot Import Report\n\n");
    output.push_str(&format!("Source: `{}`\n\n", args.input.display()));
    output.push_str(&format!("Total candidates: `{}`\n\n", outcomes.len()));

    output.push_str("## Status Counts\n\n");
    output.push_str("| Status | Count |\n| --- | ---: |\n");
    for (status, count) in &status_counts {
        output.push_str(&format!("| `{status}` | {count} |\n"));
    }

    output.push_str("\n## Top Feature Buckets\n\n");
    output.push_str("| Status | Feature | Count |\n| --- | --- | ---: |\n");
    for ((status, feature), count) in top_counts(&feature_counts, 25) {
        output.push_str(&format!("| `{status}` | `{feature}` | {count} |\n"));
    }

    output.push_str("\n## Top Source Buckets\n\n");
    output.push_str("| Status | Source | Test | Count |\n| --- | --- | --- | ---: |\n");
    for ((status, source_file, test_name), count) in top_counts(&source_counts, 25) {
        output.push_str(&format!(
            "| `{status}` | `{source_file}` | `{test_name}` | {count} |\n"
        ));
    }

    output.push_str("\n## Non-Matching Examples\n\n");
    if examples.is_empty() {
        output.push_str("All imported candidates match.\n");
    } else {
        for (status, rows) in examples {
            output.push_str(&format!("### `{status}`\n\n"));
            for outcome in rows {
                output.push_str(&format!(
                    "- `{}`: {}\n",
                    outcome.case.id,
                    code_span(&outcome.case.sql)
                ));
                if let Some(expected) = &outcome.expected {
                    output.push_str(&format!("  - expected: {}\n", code_span(expected)));
                }
                if let Some(actual) = &outcome.actual {
                    output.push_str(&format!("  - actual: {}\n", code_span(actual)));
                }
                if let Some(error) = &outcome.error {
                    output.push_str(&format!("  - error: {}\n", code_span(error)));
                }
            }
            output.push('\n');
        }
    }

    Ok(output)
}

impl OutcomeStatus {
    fn as_str(&self) -> &'static str {
        match self {
            OutcomeStatus::Match => "match",
            OutcomeStatus::Mismatch => "mismatch",
            OutcomeStatus::RustError => "rust-error",
            OutcomeStatus::OracleError => "oracle-error",
        }
    }
}

fn top_counts<K>(counts: &BTreeMap<K, usize>, limit: usize) -> Vec<(K, usize)>
where
    K: Clone + Ord,
{
    let mut rows: Vec<_> = counts
        .iter()
        .map(|(key, count)| (key.clone(), *count))
        .collect();
    rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    rows.truncate(limit);
    rows
}

fn feature_key(sql: &str) -> String {
    let trimmed = sql.trim();
    let upper = trimmed.to_ascii_uppercase();
    if upper.starts_with("CREATE TABLE") {
        return "CREATE TABLE".to_string();
    }
    if upper.starts_with("CREATE INDEX") {
        return "CREATE INDEX".to_string();
    }
    if upper.starts_with("DROP INDEX") {
        return "DROP INDEX".to_string();
    }
    if upper.starts_with("ALTER TABLE") {
        return "ALTER TABLE".to_string();
    }
    if let Some(function) = leading_function_name(trimmed) {
        return format!("{function}()");
    }
    if upper.starts_with("SELECT ") {
        return "SELECT".to_string();
    }
    upper
        .split_whitespace()
        .next()
        .unwrap_or("UNKNOWN")
        .to_string()
}

fn leading_function_name(sql: &str) -> Option<String> {
    let mut chars = sql.trim_start().chars().peekable();
    let mut name = String::new();
    while let Some(ch) = chars.peek().copied() {
        if ch == '_' || ch.is_ascii_alphanumeric() {
            name.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    if name.is_empty() {
        return None;
    }
    while matches!(chars.peek(), Some(ch) if ch.is_whitespace()) {
        chars.next();
    }
    if matches!(chars.peek(), Some('(')) {
        Some(name.to_ascii_uppercase())
    } else {
        None
    }
}

fn one_line(text: &str) -> String {
    let mut sanitized = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if matches!(chars.peek(), Some('[')) {
                chars.next();
                for next in chars.by_ref() {
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            continue;
        }
        if ch.is_control() && ch != '\n' && ch != '\t' {
            continue;
        }
        sanitized.push(ch);
    }
    sanitized.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn code_span(text: &str) -> String {
    let text = one_line(text);
    let max_backticks = text.split(|ch| ch != '`').map(str::len).max().unwrap_or(0);
    let fence = "`".repeat(max_backticks + 1);
    if text.contains('`') {
        format!("{fence} {text} {fence}")
    } else {
        format!("{fence}{text}{fence}")
    }
}

fn generate_ast_inventory(args: &InventoryArgs) -> Result<String, String> {
    let output = Command::new("python3")
        .arg("-c")
        .arg(SQLGLOT_AST_INVENTORY)
        .arg(&args.sqlglot)
        .arg(&args.rust_ast)
        .output()
        .map_err(|err| format!("failed to run python3 AST inventory: {err}"))?;

    if !output.status.success() {
        return Err(format!(
            "python3 AST inventory exited with {}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    String::from_utf8(output.stdout).map_err(|err| format!("invalid UTF-8 inventory output: {err}"))
}

const SQLGLOT_FIXTURE_IMPORTER: &str = r#"
import ast
import json
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
family = sys.argv[2]
read = sys.argv[3]
write = sys.argv[4]
limit = int(sys.argv[5])

if family != "transpile":
    raise SystemExit(f"unsupported family: {family}")

test_files = [
    root / "tests" / "test_transpile.py",
    root / "tests" / "dialects" / f"test_{read}.py",
    root / "tests" / "dialects" / f"test_{write}.py",
]
test_files = [path for path in dict.fromkeys(test_files) if path.exists()]

def literal(node):
    if node is None:
        return None
    try:
        return ast.literal_eval(node)
    except Exception:
        return None

def method_name(call):
    func = call.func
    if isinstance(func, ast.Attribute):
        return func.attr
    return None

def keyword_map(call):
    return {kw.arg: kw.value for kw in call.keywords if kw.arg}

def slug(text):
    text = re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
    return text[:64].strip("-") or "case"

def enclosing_tests(module):
    parents = {}
    for node in ast.walk(module):
        for child in ast.iter_child_nodes(node):
            parents[child] = node
    return parents

def test_name_for(parents, node):
    current = node
    while current in parents:
        current = parents[current]
        if isinstance(current, ast.FunctionDef) and current.name.startswith("test_"):
            return current.name
    return "unknown"

def class_dialect_for(parents, node):
    current = node
    while current in parents:
        current = parents[current]
        if isinstance(current, ast.ClassDef):
            for stmt in current.body:
                if isinstance(stmt, ast.Assign):
                    for target in stmt.targets:
                        if isinstance(target, ast.Name) and target.id == "dialect":
                            value = literal(stmt.value)
                            if isinstance(value, str):
                                return value
    return None

def feature_tags(sql):
    normalized = sql.strip().lower()
    tags = ["transpile", read, write, "imported"]
    if normalized.startswith(("create table", "create index", "drop index", "alter table", "drop table")):
        tags.append("ddl")
    if " index " in f" {normalized} " or normalized.startswith(("create index", "drop index")):
        tags.append("index")
    if "constraint" in normalized or " foreign key " in f" {normalized} " or " check " in f" {normalized} ":
        tags.append("constraint")
    return tags

def make_case(path, lineno, test_name, sql, source_note):
    rel = path.relative_to(root).as_posix()
    case_id = f"sqlglot-{read}-to-{write}-{slug(rel.replace('/', '-').replace('.py', ''))}-{lineno:04d}-{slug(test_name)}"
    return {
        "id": case_id,
        "sql": sql,
        "read": read,
        "write": write,
        "tags": feature_tags(sql),
        "source": f"sqlglot:{rel}:{lineno}:{test_name}:{source_note}",
        "source_file": rel,
        "source_line": lineno,
        "test_name": test_name,
        "mode": "transpile",
    }

def cases_from_validate(path, parents, call, kwargs):
    sql = literal(call.args[0]) if call.args else None
    if not isinstance(sql, str):
        return []

    call_read = literal(kwargs.get("read")) or class_dialect_for(parents, call)
    call_write = literal(kwargs.get("write"))
    if call_read == read and call_write == write:
        return [make_case(path, call.lineno, test_name_for(parents, call), sql, "validate")]
    return []

def cases_from_validate_all(path, parents, call, kwargs):
    base_sql = literal(call.args[0]) if call.args else None
    if not isinstance(base_sql, str):
        return []

    read_map = literal(kwargs.get("read")) or {}
    write_map = literal(kwargs.get("write")) or {}
    if read_map and not isinstance(read_map, dict):
        return []
    if write_map and not isinstance(write_map, dict):
        return []
    if write not in write_map:
        return []

    source_sql = read_map.get(read, base_sql)
    if not isinstance(source_sql, str):
        return []

    return [make_case(path, call.lineno, test_name_for(parents, call), source_sql, "validate_all")]

def cases_from_validate_identity(path, parents, call, _kwargs):
    class_dialect = class_dialect_for(parents, call)
    if class_dialect != read or read != write:
        return []
    sql = literal(call.args[0]) if call.args else None
    if not isinstance(sql, str):
        return []
    return [make_case(path, call.lineno, test_name_for(parents, call), sql, "validate_identity")]

cases = []
for path in sorted(test_files):
    module = ast.parse(path.read_text(), filename=str(path))
    parents = enclosing_tests(module)
    for call in ast.walk(module):
        if not isinstance(call, ast.Call):
            continue
        name = method_name(call)
        kwargs = keyword_map(call)
        if name == "validate":
            cases.extend(cases_from_validate(path, parents, call, kwargs))
        elif name == "validate_all":
            cases.extend(cases_from_validate_all(path, parents, call, kwargs))
        elif name == "validate_identity":
            cases.extend(cases_from_validate_identity(path, parents, call, kwargs))

cases.sort(key=lambda case: (case["source"], case["id"]))
deduped = []
seen_sql = set()
for case in cases:
    key = (case["sql"], case["read"], case["write"])
    if key in seen_sql:
        continue
    seen_sql.add(key)
    deduped.append(case)
    if limit and len(deduped) >= limit:
        break

print(json.dumps(deduped, sort_keys=False))
"#;

const SQLGLOT_AST_INVENTORY: &str = r###"
import ast
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

root = Path(sys.argv[1])
rust_ast = Path(sys.argv[2])
expressions_dir = root / "sqlglot" / "expressions"

def bases_for(node):
    bases = []
    for base in node.bases:
        if isinstance(base, ast.Name):
            bases.append(base.id)
        elif isinstance(base, ast.Attribute):
            bases.append(base.attr)
        elif isinstance(base, ast.Subscript):
            value = base.value
            if isinstance(value, ast.Name):
                bases.append(value.id)
            elif isinstance(value, ast.Attribute):
                bases.append(value.attr)
    return bases

classes = {}
for path in sorted(expressions_dir.glob("*.py")):
    if path.name == "__init__.py":
        continue
    module = ast.parse(path.read_text(), filename=str(path))
    for node in module.body:
        if isinstance(node, ast.ClassDef):
            classes[node.name] = {
                "name": node.name,
                "bases": bases_for(node),
                "module": path.name,
                "line": node.lineno,
            }

expression_classes = []
memo = {}
def is_expression_class(name):
    if name in memo:
        return memo[name]
    if name in {"Expression", "Expr"}:
        memo[name] = True
        return True
    info = classes.get(name)
    if not info:
        memo[name] = False
        return False
    result = any(base in {"Expression", "Expr"} or is_expression_class(base) for base in info["bases"])
    memo[name] = result
    return result

for name in sorted(classes):
    if is_expression_class(name) and name not in {"Expression", "Expr"}:
        expression_classes.append(classes[name])

rust_text = rust_ast.read_text()
def enum_variants(enum_name):
    match = re.search(r"pub enum " + re.escape(enum_name) + r"\s*\{(?P<body>.*?)\n\}", rust_text, re.S)
    if not match:
        return []
    variants = []
    for line in match.group("body").splitlines():
        line = line.strip()
        if line.startswith(("//", "///", "#")):
            continue
        variant = re.match(r"([A-Z][A-Za-z0-9_]*)\b", line)
        if variant:
            variants.append(variant.group(1))
    return variants

rust_enums = {
    "Statement": enum_variants("Statement"),
    "Expr": enum_variants("Expr"),
    "TypedFunction": enum_variants("TypedFunction"),
    "DataType": enum_variants("DataType"),
    "JoinType": enum_variants("JoinType"),
    "TableSource": enum_variants("TableSource"),
}
expression_enums = ["Statement", "Expr", "TypedFunction", "JoinType", "TableSource"]
expression_variants = {
    variant
    for enum_name in expression_enums
    for variant in rust_enums[enum_name]
}
normalized_expression_rust = {
    re.sub(r"[^a-z0-9]", "", variant.lower()): variant
    for variant in expression_variants
}
normalized_data_types = {
    re.sub(r"[^a-z0-9]", "", variant.lower()): variant
    for variant in rust_enums["DataType"]
}

supported = {
    "Alias": "Expr::Alias",
    "All": "Expr::AllOp",
    "And": "Expr::BinaryOp",
    "Any": "Expr::AnyOp",
    "Array": "Expr::ArrayLiteral",
    "Between": "Expr::Between",
    "Boolean": "Expr::Boolean",
    "Case": "Expr::Case",
    "Cast": "Expr::Cast",
    "Coalesce": "Expr::Coalesce",
    "Column": "Expr::Column",
    "Create": "Statement::CreateTable/CreateView/CreateIndex",
    "DataType": "DataType",
    "Delete": "Statement::Delete",
    "Drop": "Statement::DropTable/DropView/DropIndex",
    "EQ": "Expr::BinaryOp",
    "Exists": "Expr::Exists",
    "Extract": "Expr::Extract",
    "From": "FromClause/TableSource",
    "Func": "Expr::Function",
    "Group": "SelectStatement::group_by",
    "If": "Expr::If",
    "ILike": "Expr::ILike",
    "In": "Expr::InList/InSubquery",
    "Insert": "Statement::Insert",
    "Interval": "Expr::Interval",
    "Is": "Expr::IsNull/IsBool",
    "Join": "JoinClause",
    "Lambda": "Expr::Lambda",
    "Like": "Expr::Like",
    "Literal": "Expr::Number/StringLiteral/Boolean/Null",
    "Merge": "Statement::Merge",
    "Null": "Expr::Null",
    "Nullif": "Expr::NullIf",
    "Or": "Expr::BinaryOp",
    "Order": "SelectStatement::order_by",
    "Parameter": "Expr::Parameter",
    "Paren": "Expr::Nested",
    "Pivot": "TableSource::Pivot",
    "Select": "Statement::Select",
    "Star": "Expr::Star/Wildcard",
    "Subquery": "Expr::Subquery/TableSource::Subquery",
    "Table": "TableSource::Table",
    "TryCast": "Expr::TryCast",
    "Tuple": "Expr::Tuple",
    "Union": "Statement::SetOperation",
    "Unnest": "TableSource::Unnest",
    "Unpivot": "TableSource::Unpivot",
    "Update": "Statement::Update",
    "Use": "Statement::Use",
    "Window": "WindowSpec/Expr::TypedFunction.over",
    "With": "SelectStatement::ctes",
}

partial = {
    "Add": "covered by Expr::BinaryOp, but SQLGlot has operator-specific classes",
    "AggFunc": "many common aggregates are typed; long tail falls back to Expr::Function",
    "Alter": "Statement::AlterTable exists, but operation coverage is shallow",
    "ArrayAgg": "TypedFunction::ArrayAgg, limited ordered/filter/null behavior",
    "ArrayContains": "TypedFunction::ArrayContains, dialect coverage partial",
    "ArraySize": "TypedFunction::ArraySize, dialect coverage partial",
    "BitwiseAnd": "BinaryOperator coverage, limited dialect spelling",
    "BitwiseOr": "BinaryOperator coverage, limited dialect spelling",
    "BitwiseXor": "BinaryOperator coverage, limited dialect spelling",
    "Command": "some commands map to dedicated statements; many are unsupported",
    "Count": "TypedFunction::Count, limited ordered/filter variants",
    "DateAdd": "TypedFunction::DateAdd handles common forms",
    "DateDiff": "TypedFunction::DateDiff handles common forms",
    "DateStrToDate": "temporal functions mostly normalize through TypedFunction",
    "DateTrunc": "TypedFunction::DateTrunc handles common forms",
    "DDL": "core DDL exists; options/constraints are partial",
    "DML": "core DML exists; vendor-specific clauses are partial",
    "Explode": "TypedFunction::Explode, generator support partial",
    "FirstValue": "TypedFunction::FirstValue, window/null treatment partial",
    "GroupConcat": "supported through parsed GROUP_CONCAT function; AST lacks dedicated variant",
    "Index": "standalone CREATE/DROP INDEX exists with basic expression and sort-direction parameters; included columns, predicates, and dialect-specific options remain shallow",
    "JSONExtract": "TypedFunction::JSONExtract, path/operator coverage partial",
    "JSONExtractScalar": "TypedFunction::JSONExtractScalar, path/operator coverage partial",
    "Lag": "TypedFunction::Lag, window defaults partial",
    "LastValue": "TypedFunction::LastValue, window/null treatment partial",
    "Lead": "TypedFunction::Lead, window defaults partial",
    "Limit": "SelectStatement::limit, fetch/offset variants partial",
    "Offset": "SelectStatement::offset",
    "RegexpExtract": "TypedFunction::RegexpExtract, dialect coverage partial",
    "RegexpLike": "TypedFunction::RegexpLike, dialect coverage partial",
    "RegexpReplace": "TypedFunction::RegexpReplace, dialect coverage partial",
    "Returning": "DML returning fields exist, but coverage varies",
    "Rollup": "Expr::Rollup, group-by integration partial",
    "Cube": "Expr::Cube, group-by integration partial",
    "GroupingSets": "Expr::GroupingSets, group-by integration partial",
    "Struct": "DataType::Struct exists; expression-level struct literals are limited",
    "TableAlias": "aliases exist on TableRef/TableSource, SQLGlot's richer alias node is partial",
    "TimeToStr": "TypedFunction::TimeToStr handles common forms",
    "TsOrDsToDate": "TypedFunction::TsOrDsToDate handles common forms",
}

out_of_scope = {
    "AIAgg": "vendor/AI aggregate long tail",
    "AISummarizeAgg": "vendor/AI aggregate long tail",
    "BlockchainTable": "specialized source construct",
    "CollateProperty": "DDL property long tail",
    "EngineProperty": "DDL property long tail",
    "FileFormatProperty": "DDL property long tail",
    "IcebergProperty": "DDL property long tail",
    "ModelProperty": "DDL/model property long tail",
    "OpenAIRespond": "vendor AI function",
    "OpenAIRespondWithContext": "vendor AI function",
}

def classify(info):
    name = info["name"]
    norm = re.sub(r"[^a-z0-9]", "", name.lower())
    module = info["module"]
    if name in supported:
        return "supported", supported[name]
    if name in partial:
        return "partial", partial[name]
    if name in out_of_scope:
        return "out-of-scope", out_of_scope[name]
    if norm in normalized_expression_rust:
        return "supported", normalized_expression_rust[norm]
    if module == "datatypes.py" and norm in normalized_data_types:
        return "supported", f"DataType::{normalized_data_types[norm]}"
    bases = set(info["bases"])
    if bases & {"Binary", "Predicate", "Unary", "Condition", "Connector"} or name in {"Binary", "Predicate", "Unary", "Condition", "Connector"}:
        return "partial", "represented by generic operator/predicate nodes where parsed"
    if "Func" in bases or "AggFunc" in bases or module in {"functions.py", "aggregate.py", "temporal.py"}:
        return "partial", "generic Expr::Function fallback or selected TypedFunction variant"
    if module in {"properties.py", "constraints.py"}:
        return "partial", "DDL metadata is represented only for common cases"
    return "unsupported", "no clear Rust AST representation yet"

for info in expression_classes:
    status, notes = classify(info)
    info["status"] = status
    info["notes"] = notes

status_counts = Counter(info["status"] for info in expression_classes)
module_counts = defaultdict(Counter)
for info in expression_classes:
    module_counts[info["module"]][info["status"]] += 1

priority_gaps = [
    ("DDL properties and constraints", "partial", "First imported MySQL->SQLite batch fails on DDL/type normalization before deeper fixtures run."),
    ("Dedicated GroupConcat AST", "partial", "Current parser/generator handles key MySQL cases, but SQLGlot has GroupConcat as a first-class aggregate."),
    ("Ordered/filter aggregate modifiers", "partial", "Needed for broader aggregate fixture imports beyond simple COUNT/SUM/AVG/MIN/MAX."),
    ("Struct/map/object literals", "partial", "Data types exist, but expression-level constructors and dialect generation are thin."),
    ("JSON path/operator model", "partial", "Common JSON functions exist; SQLGlot has richer paths, operators, and dialect spellings."),
    ("Table alias and lateral richness", "partial", "Aliases exist, but SQLGlot models richer alias columns, lateral views, and source metadata."),
    ("Set operation options", "partial", "UNION/INTERSECT/EXCEPT exist; modifiers and nested ordering need more parity coverage."),
    ("Window frame/null treatment", "partial", "Window specs exist, but SQLGlot models IGNORE/RESPECT NULLS and ordered aggregate nuance."),
    ("Command/cache/export/load statements", "unsupported", "SQLGlot supports operational statements outside current core parser scope."),
    ("Long-tail function variants", "partial", "Hundreds of SQLGlot Func/AggFunc classes currently collapse to generic function or unsupported typed variants."),
]

def row(cols):
    return "| " + " | ".join(str(c).replace("\n", " ") for c in cols) + " |"

lines = []
lines.append("# AST Inventory")
lines.append("")
lines.append("Generated from Python SQLGlot's `sqlglot/expressions/` package and sqlgrok's `src/ast/types.rs`.")
lines.append("")
lines.append("This is a planning document, not a conformance claim. `supported` means sqlgrok has a clear AST home for the construct. `partial` means there is a home but missing SQLGlot behavior is likely. `unsupported` means imported fixtures need AST design before implementation. `out-of-scope` means the construct is not currently on the parity critical path.")
lines.append("")
lines.append("## Snapshot")
lines.append("")
lines.append(f"- SQLGlot expression classes: `{len(expression_classes)}`")
lines.append(f"- SQLGlot expression files: `{len([p for p in expressions_dir.glob('*.py') if p.name != '__init__.py'])}`")
lines.append(f"- Rust AST enums inspected: `{', '.join(rust_enums)}`")
lines.append(f"- Supported: `{status_counts['supported']}`")
lines.append(f"- Partial: `{status_counts['partial']}`")
lines.append(f"- Unsupported: `{status_counts['unsupported']}`")
lines.append(f"- Out of scope: `{status_counts['out-of-scope']}`")
lines.append("")
lines.append("## Priority Gaps")
lines.append("")
lines.append(row(["Gap", "Status", "Why it matters"]))
lines.append(row(["---", "---", "---"]))
for gap in priority_gaps:
    lines.append(row(gap))
lines.append("")
lines.append("## Rust AST Surface")
lines.append("")
for enum_name, variants in rust_enums.items():
    lines.append(f"- `{enum_name}`: {', '.join('`' + v + '`' for v in variants)}")
lines.append("")
lines.append("## Coverage By SQLGlot Module")
lines.append("")
lines.append(row(["Module", "Supported", "Partial", "Unsupported", "Out of scope", "Total"]))
lines.append(row(["---", "---:", "---:", "---:", "---:", "---:"]))
for module in sorted(module_counts):
    counts = module_counts[module]
    total = sum(counts.values())
    lines.append(row([module, counts["supported"], counts["partial"], counts["unsupported"], counts["out-of-scope"], total]))
lines.append("")
lines.append("## Full Generated Inventory")
lines.append("")
lines.append(row(["SQLGlot class", "Module", "Bases", "Status", "Rust representation / notes"]))
lines.append(row(["---", "---", "---", "---", "---"]))
status_rank = {"unsupported": 0, "partial": 1, "supported": 2, "out-of-scope": 3}
for info in sorted(expression_classes, key=lambda i: (status_rank[i["status"]], i["module"], i["name"])):
    lines.append(row([
        info["name"],
        f"{info['module']}:{info['line']}",
        ", ".join(info["bases"]) or "-",
        info["status"],
        info["notes"],
    ]))
lines.append("")
lines.append("## Update Command")
lines.append("")
lines.append("```bash")
lines.append("cargo run --bin xtask -- inventory-ast --sqlglot /path/to/sqlglot")
lines.append("```")
lines.append("")

print("\n".join(lines))
"###;
