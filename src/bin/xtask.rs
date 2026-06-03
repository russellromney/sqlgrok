use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

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
        "check-sqlite-correctness" => {
            run_check_sqlite_correctness(CheckSqliteCorrectnessArgs::parse(raw_args)?)
        }
        "inventory-ast" => run_inventory(InventoryArgs::parse(raw_args)?),
        "summarize-report" => run_summarize_report(SummarizeReportArgs::parse(raw_args)?),
        "bucket-suite-report" => run_bucket_suite_report(BucketSuiteReportArgs::parse(raw_args)?),
        "bench-sqlglot" => run_bench_sqlglot(BenchSqlglotArgs::parse(raw_args)?),
        "run-sqlglot-suite" => run_sqlglot_suite(SqlglotSuiteArgs::parse(raw_args)?),
        "-h" | "--help" => Err(usage()),
        _ => Err(format!("unknown command {command:?}\n\n{}", usage())),
    }
}

fn usage() -> String {
    [
        ImportArgs::usage(),
        CheckSqliteCorrectnessArgs::usage(),
        InventoryArgs::usage(),
        SummarizeReportArgs::usage(),
        BucketSuiteReportArgs::usage(),
        BenchSqlglotArgs::usage(),
        SqlglotSuiteArgs::usage(),
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

fn run_check_sqlite_correctness(args: CheckSqliteCorrectnessArgs) -> Result<(), String> {
    args.validate()?;

    let cases = read_correctness_cases(&args.input)?;
    let mut outcomes = Vec::new();
    let mut status_counts = BTreeMap::<String, usize>::new();

    for case in cases {
        let oracle = python_oracle_transpile_sql(&args.sqlglot, &case.sql, &case.read, "sqlite")?;
        let outcome = if !oracle.ok {
            CorrectnessOutcome {
                case,
                oracle_sql: None,
                sqlgrok_sql: None,
                parity_status: CorrectnessParityStatus::OracleError,
                status: CorrectnessStatus::OracleError,
                sqlite_stdout: None,
                sqlite_stderr: oracle.error,
            }
        } else if let Some(oracle_sql) = oracle.sql {
            let (sqlgrok_sql, parity_status) =
                evaluate_correctness_parity(&case.sql, &case.read, &oracle_sql);
            let sqlite = run_sqlite(&case.setup_sql, &oracle_sql)?;
            let status = if sqlite.ok {
                CorrectnessStatus::SqliteOk
            } else {
                CorrectnessStatus::SqliteError
            };
            CorrectnessOutcome {
                case,
                oracle_sql: Some(oracle_sql),
                sqlgrok_sql,
                parity_status,
                status,
                sqlite_stdout: sqlite.stdout,
                sqlite_stderr: sqlite.stderr,
            }
        } else {
            CorrectnessOutcome {
                case,
                oracle_sql: None,
                sqlgrok_sql: None,
                parity_status: CorrectnessParityStatus::OracleError,
                status: CorrectnessStatus::OracleError,
                sqlite_stdout: None,
                sqlite_stderr: Some("oracle returned ok without sql".to_string()),
            }
        };
        *status_counts
            .entry(outcome.status.as_str().to_string())
            .or_default() += 1;
        outcomes.push(outcome);
    }

    if !args.dry_run
        && let Some(jsonl_output) = &args.jsonl_output
    {
        write_correctness_jsonl(jsonl_output, &outcomes)?;
    }

    let markdown = summarize_correctness(&args.input, &outcomes);
    if args.dry_run {
        print!("{markdown}");
    } else {
        if let Some(parent) = args.markdown_output.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
        }
        fs::write(&args.markdown_output, markdown)
            .map_err(|err| format!("failed to write {}: {err}", args.markdown_output.display()))?;
        eprintln!("wrote {}", args.markdown_output.display());
    }

    eprintln!(
        "checked {} SQLite correctness cases: {}",
        outcomes.len(),
        status_counts
            .iter()
            .map(|(status, count)| format!("{status}={count}"))
            .collect::<Vec<_>>()
            .join(", ")
    );

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

fn run_bucket_suite_report(args: BucketSuiteReportArgs) -> Result<(), String> {
    args.validate()?;

    let summary = bucket_suite_report(&args.input)?;
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

fn run_bench_sqlglot(args: BenchSqlglotArgs) -> Result<(), String> {
    args.validate()?;

    let cases = if let Some(path) = &args.cases {
        read_bench_cases(path)?
    } else {
        bench_cases()
    };
    if cases.is_empty() {
        return Err("benchmark workload must contain at least one case".to_string());
    }
    validate_bench_cases(&args.sqlglot, &cases)?;

    let operations = args.iterations * cases.len();
    let (baseline, candidate) = run_benchmark_samples(&args, &cases, operations, "aggregate")?;
    let per_case = if args.per_case {
        Some(run_per_case_benchmarks(&args, &cases)?)
    } else {
        None
    };
    let report = render_benchmark_report(&args, &cases, &baseline, &candidate, per_case.as_deref());
    let json_report =
        render_benchmark_json(&args, &cases, &baseline, &candidate, per_case.as_deref())?;
    if args.dry_run {
        print!("{report}");
        return Ok(());
    }

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(&args.output, report)
        .map_err(|err| format!("failed to write {}: {err}", args.output.display()))?;
    eprintln!("wrote {}", args.output.display());
    if let Some(json_output) = &args.json_output {
        if let Some(parent) = json_output.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
        }
        fs::write(json_output, json_report)
            .map_err(|err| format!("failed to write {}: {err}", json_output.display()))?;
        eprintln!("wrote {}", json_output.display());
    }

    Ok(())
}

fn run_sqlglot_suite(args: SqlglotSuiteArgs) -> Result<(), String> {
    args.validate()?;

    if let Some(parent) = args.report_output.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }

    let mut command = Command::new(&args.python);
    command.args(&args.python_args);
    let display_command = std::iter::once(args.python.display().to_string())
        .chain(args.python_args.iter().cloned())
        .collect::<Vec<_>>()
        .join(" ");
    command
        .arg("-m")
        .arg("sqlgrok.sqlglot_bridge")
        .arg("--sqlglot")
        .arg(&args.sqlglot)
        .arg("--family")
        .arg(&args.family)
        .arg("--read")
        .arg(&args.read)
        .arg("--write")
        .arg(&args.write)
        .arg("--report-output")
        .arg(&args.report_output);
    if args.force_pair {
        command.arg("--force-pair");
    }
    for module in &args.modules {
        command.arg("--module").arg(module);
    }

    if args.check_budget {
        command.arg("--check-budget");
        command.arg("--budget").arg(&args.budget);
    }
    command.args(&args.pytest_args);
    if let Some(python_dir) = args.python.parent() {
        let path = env::var_os("PATH").unwrap_or_default();
        let mut paths = vec![python_dir.to_path_buf()];
        paths.extend(env::split_paths(&path));
        let joined = env::join_paths(paths).map_err(|err| {
            format!(
                "failed to prepend {} to PATH for SQLGlot bridge: {err}",
                python_dir.display()
            )
        })?;
        command.env("PATH", joined);
    }
    let pythonpath = env::var_os("PYTHONPATH").unwrap_or_default();
    let mut pythonpaths = vec![args.sqlglot.clone()];
    pythonpaths.extend(env::split_paths(&pythonpath));
    let joined_pythonpath = env::join_paths(pythonpaths)
        .map_err(|err| format!("failed to set PYTHONPATH for SQLGlot bridge: {err}"))?;
    command.env("PYTHONPATH", joined_pythonpath);

    let status = command
        .status()
        .map_err(|err| format!("failed to run SQLGlot bridge with {display_command}: {err}",))?;

    if !status.success() {
        return Err(format!("SQLGlot bridge exited with {status}"));
    }

    eprintln!("wrote {}", args.report_output.display());
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
struct CheckSqliteCorrectnessArgs {
    sqlglot: PathBuf,
    input: PathBuf,
    jsonl_output: Option<PathBuf>,
    markdown_output: PathBuf,
    dry_run: bool,
}

impl CheckSqliteCorrectnessArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.peekable();
        let mut sqlglot = None;
        let mut input: Option<PathBuf> = None;
        let mut jsonl_output: Option<PathBuf> = None;
        let mut markdown_output: Option<PathBuf> = None;
        let mut dry_run = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--sqlglot" => sqlglot = Some(next_value(&mut args, "--sqlglot")?.into()),
                "--input" => input = Some(next_value(&mut args, "--input")?.into()),
                "--jsonl-output" => {
                    jsonl_output = Some(next_value(&mut args, "--jsonl-output")?.into())
                }
                "--markdown-output" => {
                    markdown_output = Some(next_value(&mut args, "--markdown-output")?.into())
                }
                "--dry-run" => dry_run = true,
                "-h" | "--help" => return Err(Self::usage()),
                _ => return Err(format!("unknown argument {arg:?}\n\n{}", Self::usage())),
            }
        }

        let sqlglot = sqlglot.ok_or_else(|| "--sqlglot is required".to_string())?;
        let input = input.unwrap_or_else(|| PathBuf::from("correctness/cases/sqlite_compat.jsonl"));
        let markdown_output = markdown_output.unwrap_or_else(|| {
            let stem = input
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("sqlite_correctness");
            PathBuf::from("correctness/reports").join(format!("{stem}.md"))
        });

        Ok(Self {
            sqlglot,
            input,
            jsonl_output,
            markdown_output,
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
        if !self.input.is_file() {
            return Err(format!("{} does not exist", self.input.display()));
        }
        let sqlite = Command::new("sqlite3")
            .arg("-version")
            .output()
            .map_err(|err| format!("failed to run sqlite3 -version: {err}"))?;
        if !sqlite.status.success() {
            return Err(format!(
                "sqlite3 -version exited with {}\nstderr:\n{}",
                sqlite.status,
                String::from_utf8_lossy(&sqlite.stderr)
            ));
        }
        Ok(())
    }

    fn usage() -> String {
        "usage: cargo run --bin xtask -- check-sqlite-correctness --sqlglot /path/to/sqlglot [--input correctness/cases/sqlite_compat.jsonl] [--jsonl-output correctness/reports/sqlite_compat.jsonl] [--markdown-output correctness/reports/sqlite_compat.md] [--dry-run]".to_string()
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
        let mut output = PathBuf::from("parity/reports/ast_inventory.md");
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
        "usage: cargo run --bin xtask -- inventory-ast --sqlglot /path/to/sqlglot [--rust-ast src/ast/types.rs] [--output parity/reports/ast_inventory.md] [--dry-run]".to_string()
    }
}

#[derive(Debug)]
struct SummarizeReportArgs {
    input: PathBuf,
    output: PathBuf,
    dry_run: bool,
}

#[derive(Debug)]
struct BucketSuiteReportArgs {
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

#[derive(Debug)]
struct BenchSqlglotArgs {
    sqlglot: PathBuf,
    mode: BenchMode,
    cases: Option<PathBuf>,
    iterations: usize,
    warmup: usize,
    samples: usize,
    per_case: bool,
    output: PathBuf,
    json_output: Option<PathBuf>,
    python: PathBuf,
    python_args: Vec<String>,
    dry_run: bool,
}

#[derive(Debug)]
struct SqlglotSuiteArgs {
    sqlglot: PathBuf,
    family: String,
    read: String,
    write: String,
    modules: Vec<String>,
    report_output: PathBuf,
    budget: PathBuf,
    check_budget: bool,
    force_pair: bool,
    python: PathBuf,
    python_args: Vec<String>,
    pytest_args: Vec<String>,
}

impl SqlglotSuiteArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.peekable();
        let mut sqlglot = None;
        let mut family = None;
        let mut read = None;
        let mut write = None;
        let mut modules = Vec::new();
        let mut report_output = None;
        let mut budget = None;
        let mut check_budget = false;
        let mut force_pair = false;
        let mut python = None;
        let mut pytest_args = Vec::new();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--sqlglot" => sqlglot = Some(next_value(&mut args, "--sqlglot")?.into()),
                "--family" => family = Some(next_value(&mut args, "--family")?),
                "--read" => read = Some(next_value(&mut args, "--read")?),
                "--write" => write = Some(next_value(&mut args, "--write")?),
                "--module" => modules.push(next_value(&mut args, "--module")?),
                "--report-output" => {
                    report_output = Some(next_value(&mut args, "--report-output")?.into())
                }
                "--budget" => budget = Some(next_value(&mut args, "--budget")?.into()),
                "--check-budget" => check_budget = true,
                "--force-pair" => force_pair = true,
                "--python" => python = Some(next_value(&mut args, "--python")?.into()),
                "--pytest-arg" => pytest_args.push(next_value(&mut args, "--pytest-arg")?),
                "-h" | "--help" => return Err(Self::usage()),
                _ => return Err(format!("unknown argument {arg:?}\n\n{}", Self::usage())),
            }
        }

        let sqlglot = sqlglot.unwrap_or_else(default_sqlglot_path);
        let family = family.unwrap_or_else(|| "transpile".to_string());
        let read = read.ok_or_else(|| "--read is required".to_string())?;
        let write = write.ok_or_else(|| "--write is required".to_string())?;
        if modules.is_empty() {
            modules = default_sqlglot_test_modules(&sqlglot);
        }
        let report_prefix = if force_pair {
            "sqlglot_suite_forced"
        } else {
            "sqlglot_suite"
        };
        let report_output = report_output.unwrap_or_else(|| {
            PathBuf::from("parity/reports").join(format!(
                "{}_{}_{}_{}.jsonl",
                report_prefix, family, read, write
            ))
        });
        let budget = budget.unwrap_or_else(|| {
            PathBuf::from("parity/budgets").join(format!(
                "{}_{}_{}_{}.json",
                report_prefix, family, read, write
            ))
        });
        let (python, python_args) = if let Some(python) = python {
            (python, Vec::new())
        } else if let Some(python) = env::var_os("PYTHON").map(PathBuf::from) {
            (python, Vec::new())
        } else {
            (
                PathBuf::from("uv"),
                vec![
                    "run".to_string(),
                    "--project".to_string(),
                    "python".to_string(),
                    "--with".to_string(),
                    "pytest".to_string(),
                    "--with".to_string(),
                    "pytz".to_string(),
                    "python".to_string(),
                ],
            )
        };

        Ok(Self {
            sqlglot,
            family,
            read,
            write,
            modules,
            report_output,
            budget,
            check_budget,
            force_pair,
            python,
            python_args,
            pytest_args,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if self.family != "transpile" {
            return Err(format!(
                "unsupported SQLGlot suite family {:?}; currently only \"transpile\" is implemented",
                self.family
            ));
        }
        if !self.sqlglot.join("sqlglot/__init__.py").is_file() {
            return Err(format!(
                "{} does not look like a Python SQLGlot checkout",
                self.sqlglot.display()
            ));
        }
        for module in &self.modules {
            if !self.sqlglot.join(module).is_file() {
                return Err(format!(
                    "{} is not a SQLGlot test module",
                    self.sqlglot.join(module).display()
                ));
            }
        }
        if self.check_budget && !self.budget.is_file() {
            return Err(format!(
                "--check-budget requested but {} does not exist",
                self.budget.display()
            ));
        }
        Ok(())
    }

    fn usage() -> String {
        "usage: cargo run --bin xtask -- run-sqlglot-suite --sqlglot /path/to/sqlglot --family transpile --read postgres --write sqlite [--force-pair] [--module tests/dialects/test_postgres.py ...] [--report-output parity/reports/sqlglot_suite_transpile_postgres_sqlite.jsonl] [--check-budget] [--budget parity/budgets/sqlglot_suite_transpile_postgres_sqlite.json] [--python python3] [--pytest-arg -q]".to_string()
    }
}

impl BucketSuiteReportArgs {
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
                .unwrap_or("sqlglot_suite_report");
            PathBuf::from("parity/reports").join(format!("{stem}_buckets.md"))
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
        "usage: cargo run --bin xtask -- bucket-suite-report --input parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl [--output parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite_buckets.md] [--dry-run]".to_string()
    }
}

impl BenchSqlglotArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.peekable();
        let mut sqlglot = None;
        let mut mode = BenchMode::Core;
        let mut cases = None;
        let mut iterations = 2_000;
        let mut warmup = 100;
        let mut samples = 5;
        let mut per_case = false;
        let mut output = None;
        let mut json_output = None;
        let mut python = None;
        let mut dry_run = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--sqlglot" => sqlglot = Some(next_value(&mut args, "--sqlglot")?.into()),
                "--mode" => mode = BenchMode::parse(&next_value(&mut args, "--mode")?)?,
                "--cases" => cases = Some(next_value(&mut args, "--cases")?.into()),
                "--iterations" => {
                    let raw = next_value(&mut args, "--iterations")?;
                    iterations = raw.parse().map_err(|_| {
                        format!("--iterations must be a positive integer, got {raw:?}")
                    })?;
                }
                "--warmup" => {
                    let raw = next_value(&mut args, "--warmup")?;
                    warmup = raw.parse().map_err(|_| {
                        format!("--warmup must be a non-negative integer, got {raw:?}")
                    })?;
                }
                "--samples" => {
                    let raw = next_value(&mut args, "--samples")?;
                    samples = raw.parse().map_err(|_| {
                        format!("--samples must be a positive integer, got {raw:?}")
                    })?;
                }
                "--per-case" => per_case = true,
                "--output" => output = Some(next_value(&mut args, "--output")?.into()),
                "--json-output" => {
                    json_output = Some(next_value(&mut args, "--json-output")?.into())
                }
                "--python" => python = Some(next_value(&mut args, "--python")?.into()),
                "--dry-run" => dry_run = true,
                "-h" | "--help" => return Err(Self::usage()),
                _ => return Err(format!("unknown argument {arg:?}\n\n{}", Self::usage())),
            }
        }

        let output = output.unwrap_or_else(|| {
            PathBuf::from("benchmarks/reports")
                .join(format!("sqlglot_comparison_{}.md", mode.as_str()))
        });
        let json_output = Some(json_output.unwrap_or_else(|| output.with_extension("json")));
        let (python, python_args) = if let Some(python) = python {
            (python, Vec::new())
        } else if let Some(python) = env::var_os("PYTHON").map(PathBuf::from) {
            (python, Vec::new())
        } else {
            default_uv_python_command()
        };

        Ok(Self {
            sqlglot: sqlglot.unwrap_or_else(default_sqlglot_path),
            mode,
            cases,
            iterations,
            warmup,
            samples,
            per_case,
            output,
            json_output,
            python,
            python_args,
            dry_run,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if !self.sqlglot.is_dir() {
            return Err(format!("{} does not exist", self.sqlglot.display()));
        }
        if self.iterations == 0 {
            return Err("--iterations must be greater than zero".to_string());
        }
        if self.samples == 0 {
            return Err("--samples must be greater than zero".to_string());
        }
        if let Some(cases) = &self.cases
            && !cases.is_file()
        {
            return Err(format!("{} does not exist", cases.display()));
        }
        Ok(())
    }

    fn usage() -> String {
        "usage: cargo run --release --bin xtask -- bench-sqlglot --sqlglot /path/to/sqlglot [--mode core|python-binding|python-binding-batch] [--cases benchmarks/cases/postgres_sqlite.jsonl] [--iterations 2000] [--warmup 100] [--samples 5] [--per-case] [--output benchmarks/reports/sqlglot_comparison_core.md] [--json-output benchmarks/reports/sqlglot_comparison_core.json] [--python python3] [--dry-run]".to_string()
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

fn default_sqlglot_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(
            || PathBuf::from("../sqlglot"),
            |parent| parent.join("sqlglot"),
        )
}

fn default_uv_python_command() -> (PathBuf, Vec<String>) {
    (
        PathBuf::from("uv"),
        vec![
            "run".to_string(),
            "--project".to_string(),
            "python".to_string(),
            "--with".to_string(),
            "pytest".to_string(),
            "--with".to_string(),
            "pytz".to_string(),
            "python".to_string(),
        ],
    )
}

fn python_command(python: &Path, python_args: &[String]) -> Command {
    let mut command = Command::new(python);
    command.args(python_args);
    command
}

fn default_sqlglot_test_modules(sqlglot: &Path) -> Vec<String> {
    let mut modules = vec!["tests/test_transpile.py".to_string()];
    let dialects_dir = sqlglot.join("tests/dialects");
    let mut dialect_modules = Vec::new();
    if let Ok(entries) = fs::read_dir(dialects_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
                continue;
            };
            if file_name.starts_with("test_") && file_name.ends_with(".py") {
                dialect_modules.push(format!("tests/dialects/{file_name}"));
            }
        }
    }
    dialect_modules.sort();
    modules.extend(dialect_modules);
    modules
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
struct CorrectnessCase {
    id: String,
    sql: String,
    read: String,
    #[serde(default)]
    setup_sql: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    source: String,
    #[serde(default)]
    note: String,
}

#[derive(Debug, Serialize)]
struct CorrectnessOutcome {
    #[serde(flatten)]
    case: CorrectnessCase,
    oracle_sql: Option<String>,
    sqlgrok_sql: Option<String>,
    parity_status: CorrectnessParityStatus,
    status: CorrectnessStatus,
    sqlite_stdout: Option<String>,
    sqlite_stderr: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SuiteReportCase {
    status: String,
    source_file: String,
    source_line: usize,
    test_name: String,
    helper: String,
    sql: String,
    read: Option<String>,
    write: Option<String>,
    expected: Option<String>,
    actual: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum CorrectnessParityStatus {
    Match,
    Mismatch,
    RustError,
    OracleError,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum CorrectnessStatus {
    SqliteOk,
    SqliteError,
    OracleError,
}

struct SqliteRun {
    ok: bool,
    stdout: Option<String>,
    stderr: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum OutcomeStatus {
    Match,
    Mismatch,
    RustError,
    OracleError,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BenchCase {
    id: String,
    sql: String,
    read: String,
    write: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    feature: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BenchResult {
    elapsed_ns: u128,
    checksum: usize,
}

#[derive(Debug, Clone, Serialize)]
struct BenchSample {
    sample: usize,
    elapsed_ns: u128,
    ns_per_op: f64,
    checksum: usize,
}

impl BenchSample {
    fn from_result(sample: usize, operations: usize, result: BenchResult) -> Self {
        Self {
            sample,
            elapsed_ns: result.elapsed_ns,
            ns_per_op: result.elapsed_ns as f64 / operations as f64,
            checksum: result.checksum,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct BenchStats {
    samples: Vec<BenchSample>,
    checksum: usize,
    min_ns_per_op: f64,
    max_ns_per_op: f64,
    mean_ns_per_op: f64,
    median_ns_per_op: f64,
    p95_ns_per_op: f64,
}

#[derive(Debug, Clone, Serialize)]
struct BenchCaseComparison {
    id: String,
    read: String,
    write: String,
    sql: String,
    tags: Vec<String>,
    feature: String,
    baseline: BenchStats,
    candidate: BenchStats,
    median_speedup: f64,
}

impl BenchStats {
    fn from_samples(samples: Vec<BenchSample>) -> Result<Self, String> {
        let checksum = samples
            .first()
            .map(|sample| sample.checksum)
            .ok_or_else(|| "benchmark statistics require at least one sample".to_string())?;
        if let Some(sample) = samples.iter().find(|sample| sample.checksum != checksum) {
            return Err(format!(
                "benchmark checksum changed inside stats: expected {}, got {} on sample {}",
                checksum, sample.checksum, sample.sample
            ));
        }

        let mut values = samples
            .iter()
            .map(|sample| sample.ns_per_op)
            .collect::<Vec<_>>();
        values.sort_by(|left, right| left.total_cmp(right));

        let min_ns_per_op = values[0];
        let max_ns_per_op = values[values.len() - 1];
        let mean_ns_per_op = values.iter().sum::<f64>() / values.len() as f64;
        let median_ns_per_op = median_sorted(&values);
        let p95_ns_per_op = percentile_sorted(&values, 0.95);

        Ok(Self {
            samples,
            checksum,
            min_ns_per_op,
            max_ns_per_op,
            mean_ns_per_op,
            median_ns_per_op,
            p95_ns_per_op,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum BenchMode {
    Core,
    PythonBinding,
    PythonBindingBatch,
}

impl BenchMode {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "core" => Ok(Self::Core),
            "python-binding" => Ok(Self::PythonBinding),
            "python-binding-batch" => Ok(Self::PythonBindingBatch),
            _ => Err(format!(
                "unknown benchmark mode {value:?}; expected core, python-binding, or python-binding-batch"
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::PythonBinding => "python-binding",
            Self::PythonBindingBatch => "python-binding-batch",
        }
    }

    fn candidate_label(self) -> &'static str {
        match self {
            Self::Core => "sqlgrok direct Rust",
            Self::PythonBinding => "sqlgrok PyO3 single-call",
            Self::PythonBindingBatch => "sqlgrok PyO3 batch",
        }
    }
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

fn bench_cases() -> Vec<BenchCase> {
    [
        (
            "mysql-group-concat",
            "SELECT GROUP_CONCAT(v ORDER BY v SEPARATOR '|') FROM gc",
            "mysql",
            "aggregate",
        ),
        (
            "mysql-json-extract",
            "SELECT JSON_EXTRACT(data, '$.k') FROM events WHERE id = 1",
            "mysql",
            "json",
        ),
        (
            "mysql-limit-offset",
            "SELECT a, b FROM t WHERE a > 10 ORDER BY b DESC LIMIT 5, 10",
            "mysql",
            "limit",
        ),
        (
            "mysql-date-format",
            "SELECT DATE_FORMAT(created_at, '%Y-%m-%d') FROM users",
            "mysql",
            "datetime",
        ),
        (
            "mysql-if-cast-division",
            "SELECT IF(a > 0, CAST(a AS SIGNED INTEGER), 7 DIV 2), x / y FROM metrics",
            "mysql",
            "expression",
        ),
        (
            "postgres-distinct-on",
            "SELECT DISTINCT ON (account_id) account_id, created_at FROM events ORDER BY account_id, created_at DESC",
            "postgres",
            "rewrite",
        ),
        (
            "postgres-json-path",
            "SELECT payload #>> '{customer,0,name}' FROM events WHERE payload ->> 'kind' = 'signup'",
            "postgres",
            "json",
        ),
        (
            "postgres-extract-cast",
            "SELECT EXTRACT(YEAR FROM CAST(created_at AS DATE)), DATE_TRUNC('month', created_at) FROM events",
            "postgres",
            "datetime",
        ),
        (
            "postgres-rollup",
            "SELECT region, product, SUM(revenue) FROM sales GROUP BY ROLLUP(region, product)",
            "postgres",
            "grouping",
        ),
        (
            "postgres-window-nulls",
            "SELECT user_id, ROW_NUMBER() OVER (PARTITION BY account_id ORDER BY created_at) FROM events",
            "postgres",
            "window",
        ),
    ]
    .into_iter()
    .map(|(id, sql, read, feature)| BenchCase {
        id: id.to_string(),
        sql: sql.to_string(),
        read: read.to_string(),
        write: "sqlite".to_string(),
        tags: vec![feature.to_string()],
        feature: feature.to_string(),
    })
    .collect()
}

fn read_bench_cases(path: &Path) -> Result<Vec<BenchCase>, String> {
    let text = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let mut cases = Vec::new();
    let mut ids = HashSet::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let mut case: BenchCase = serde_json::from_str(line)
            .map_err(|err| format!("{}:{}: invalid JSON: {err}", path.display(), index + 1))?;
        if case.id.trim().is_empty() {
            return Err(format!(
                "{}:{}: id must not be empty",
                path.display(),
                index + 1
            ));
        }
        if !ids.insert(case.id.clone()) {
            return Err(format!("duplicate benchmark case id {:?}", case.id));
        }
        if case.sql.trim().is_empty() {
            return Err(format!("{}: sql must not be empty", case.id));
        }
        if case.read.trim().is_empty() {
            return Err(format!("{}: read must not be empty", case.id));
        }
        if case.write.trim().is_empty() {
            return Err(format!("{}: write must not be empty", case.id));
        }
        if case.feature.is_empty() {
            case.feature = case
                .tags
                .first()
                .cloned()
                .unwrap_or_else(|| "uncategorized".to_string());
        }
        cases.push(case);
    }
    if cases.is_empty() {
        return Err(format!("{} contains no benchmark cases", path.display()));
    }
    Ok(cases)
}

fn validate_bench_cases(sqlglot: &PathBuf, cases: &[BenchCase]) -> Result<(), String> {
    for case in cases {
        let oracle = python_oracle_transpile_sql(sqlglot, &case.sql, &case.read, &case.write)?;
        if !oracle.ok {
            return Err(format!(
                "{}: Python SQLGlot oracle error: {}",
                case.id,
                oracle.error.unwrap_or_default()
            ));
        }
        let Some(expected) = oracle.sql else {
            return Err(format!(
                "{}: Python SQLGlot returned ok without SQL",
                case.id
            ));
        };
        let read = Dialect::from_str(&case.read)
            .ok_or_else(|| format!("{}: unknown read dialect {:?}", case.id, case.read))?;
        let write = Dialect::from_str(&case.write)
            .ok_or_else(|| format!("{}: unknown write dialect {:?}", case.id, case.write))?;
        let actual = transpile(&case.sql, read, write)
            .map_err(|err| format!("{}: sqlgrok failed: {err}", case.id))?;
        if actual != expected {
            return Err(format!(
                "{}: benchmark case is not parity-clean\npython SQLGlot: {}\nsqlgrok: {}",
                case.id, expected, actual
            ));
        }
    }
    Ok(())
}

fn run_rust_benchmark(args: &BenchSqlglotArgs, cases: &[BenchCase]) -> Result<BenchResult, String> {
    let dialects = cases
        .iter()
        .map(|case| {
            let read = Dialect::from_str(&case.read)
                .ok_or_else(|| format!("{}: unknown read dialect {:?}", case.id, case.read))?;
            let write = Dialect::from_str(&case.write)
                .ok_or_else(|| format!("{}: unknown write dialect {:?}", case.id, case.write))?;
            Ok((read, write))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let mut checksum = 0usize;
    for _ in 0..args.warmup {
        for (case, (read, write)) in cases.iter().zip(&dialects) {
            let output = transpile(std::hint::black_box(&case.sql), *read, *write)
                .map_err(|err| format!("{}: sqlgrok warmup failed: {err}", case.id))?;
            checksum = checksum.wrapping_add(output.len());
        }
    }

    checksum = 0;
    let started = Instant::now();
    for _ in 0..args.iterations {
        for (case, (read, write)) in cases.iter().zip(&dialects) {
            let output = transpile(std::hint::black_box(&case.sql), *read, *write)
                .map_err(|err| format!("{}: sqlgrok benchmark failed: {err}", case.id))?;
            checksum = checksum.wrapping_add(output.len());
        }
    }

    Ok(BenchResult {
        elapsed_ns: started.elapsed().as_nanos(),
        checksum,
    })
}

fn run_candidate_benchmark(
    args: &BenchSqlglotArgs,
    cases: &[BenchCase],
) -> Result<BenchResult, String> {
    match args.mode {
        BenchMode::Core => run_rust_benchmark(args, cases),
        BenchMode::PythonBinding => run_python_binding_benchmark(args, cases),
        BenchMode::PythonBindingBatch => run_python_binding_batch_benchmark(args, cases),
    }
}

fn run_benchmark_samples(
    args: &BenchSqlglotArgs,
    cases: &[BenchCase],
    operations: usize,
    label: &str,
) -> Result<(BenchStats, BenchStats), String> {
    let mut baseline_samples = Vec::with_capacity(args.samples);
    let mut candidate_samples = Vec::with_capacity(args.samples);
    let mut expected_checksum = None;

    for sample_index in 0..args.samples {
        eprintln!(
            "running {label} benchmark sample {}/{} ({})",
            sample_index + 1,
            args.samples,
            if sample_index % 2 == 0 {
                "python first"
            } else {
                "candidate first"
            }
        );

        let (baseline, candidate) = if sample_index % 2 == 0 {
            let baseline = run_python_benchmark(args, cases)?;
            let candidate = run_candidate_benchmark(args, cases)?;
            (baseline, candidate)
        } else {
            let candidate = run_candidate_benchmark(args, cases)?;
            let baseline = run_python_benchmark(args, cases)?;
            (baseline, candidate)
        };

        if baseline.checksum != candidate.checksum {
            return Err(format!(
                "{label}: benchmark checksum mismatch on sample {}: python={}, {}={}",
                sample_index + 1,
                baseline.checksum,
                args.mode.candidate_label(),
                candidate.checksum
            ));
        }
        if let Some(checksum) = expected_checksum {
            if baseline.checksum != checksum {
                return Err(format!(
                    "{label}: benchmark checksum changed on sample {}: expected {}, got {}",
                    sample_index + 1,
                    checksum,
                    baseline.checksum
                ));
            }
        } else {
            expected_checksum = Some(baseline.checksum);
        }

        baseline_samples.push(BenchSample::from_result(
            sample_index + 1,
            operations,
            baseline,
        ));
        candidate_samples.push(BenchSample::from_result(
            sample_index + 1,
            operations,
            candidate,
        ));
    }

    Ok((
        BenchStats::from_samples(baseline_samples)?,
        BenchStats::from_samples(candidate_samples)?,
    ))
}

fn run_per_case_benchmarks(
    args: &BenchSqlglotArgs,
    cases: &[BenchCase],
) -> Result<Vec<BenchCaseComparison>, String> {
    let mut results = Vec::with_capacity(cases.len());
    for case in cases {
        let label = format!("case {}", case.id);
        let one_case = std::slice::from_ref(case);
        let (baseline, candidate) = run_benchmark_samples(args, one_case, args.iterations, &label)?;
        results.push(BenchCaseComparison {
            id: case.id.clone(),
            read: case.read.clone(),
            write: case.write.clone(),
            sql: case.sql.clone(),
            tags: case.tags.clone(),
            feature: case.feature.clone(),
            median_speedup: baseline.median_ns_per_op / candidate.median_ns_per_op,
            baseline,
            candidate,
        });
    }
    Ok(results)
}

fn run_python_benchmark(
    args: &BenchSqlglotArgs,
    cases: &[BenchCase],
) -> Result<BenchResult, String> {
    let payload = serde_json::json!({
        "cases": cases,
        "iterations": args.iterations,
        "warmup": args.warmup,
    });

    let mut command = python_command(&args.python, &args.python_args);
    command
        .arg("-c")
        .arg(SQLGLOT_BENCHMARK_SCRIPT)
        .arg(&args.sqlglot)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|err| format!("failed to run Python SQLGlot benchmark: {err}"))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "failed to open Python benchmark stdin".to_string())?;
        stdin
            .write_all(payload.to_string().as_bytes())
            .map_err(|err| format!("failed to write Python benchmark stdin: {err}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("failed to read Python benchmark output: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "Python SQLGlot benchmark exited with {}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|err| {
        format!(
            "invalid Python benchmark JSON: {err}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn run_python_binding_benchmark(
    args: &BenchSqlglotArgs,
    cases: &[BenchCase],
) -> Result<BenchResult, String> {
    let payload = serde_json::json!({
        "cases": cases,
        "iterations": args.iterations,
        "warmup": args.warmup,
    });

    let mut command = python_command(&args.python, &args.python_args);
    command
        .arg("-c")
        .arg(SQLGROK_PYTHON_BINDING_BENCHMARK_SCRIPT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|err| format!("failed to run sqlgrok Python binding benchmark: {err}"))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "failed to open sqlgrok Python binding benchmark stdin".to_string())?;
        stdin
            .write_all(payload.to_string().as_bytes())
            .map_err(|err| format!("failed to write sqlgrok Python binding stdin: {err}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("failed to read sqlgrok Python binding benchmark output: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "sqlgrok Python binding benchmark exited with {}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|err| {
        format!(
            "invalid sqlgrok Python binding benchmark JSON: {err}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn run_python_binding_batch_benchmark(
    args: &BenchSqlglotArgs,
    cases: &[BenchCase],
) -> Result<BenchResult, String> {
    let payload = serde_json::json!({
        "cases": cases,
        "iterations": args.iterations,
        "warmup": args.warmup,
    });

    let mut command = python_command(&args.python, &args.python_args);
    command
        .arg("-c")
        .arg(SQLGROK_PYTHON_BINDING_BATCH_BENCHMARK_SCRIPT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|err| format!("failed to run sqlgrok Python binding batch benchmark: {err}"))?;

    {
        let stdin = child.stdin.as_mut().ok_or_else(|| {
            "failed to open sqlgrok Python binding batch benchmark stdin".to_string()
        })?;
        stdin
            .write_all(payload.to_string().as_bytes())
            .map_err(|err| format!("failed to write sqlgrok Python binding batch stdin: {err}"))?;
    }

    let output = child.wait_with_output().map_err(|err| {
        format!("failed to read sqlgrok Python binding batch benchmark output: {err}")
    })?;
    if !output.status.success() {
        return Err(format!(
            "sqlgrok Python binding batch benchmark exited with {}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|err| {
        format!(
            "invalid sqlgrok Python binding batch benchmark JSON: {err}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn median_sorted(values: &[f64]) -> f64 {
    let middle = values.len() / 2;
    if values.len().is_multiple_of(2) {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    }
}

fn percentile_sorted(values: &[f64], percentile: f64) -> f64 {
    let index = ((values.len() as f64 * percentile).ceil() as usize)
        .saturating_sub(1)
        .min(values.len() - 1);
    values[index]
}

fn render_benchmark_report(
    args: &BenchSqlglotArgs,
    cases: &[BenchCase],
    baseline: &BenchStats,
    candidate: &BenchStats,
    per_case: Option<&[BenchCaseComparison]>,
) -> String {
    let operations = args.iterations * cases.len();
    let speedup = baseline.median_ns_per_op / candidate.median_ns_per_op;

    let mut out = String::new();
    out.push_str("# SQLGlot Performance Comparison\n\n");
    out.push_str("Compares Python SQLGlot against a parity-clean sqlgrok execution path.\n\n");
    out.push_str("Run with a release build for meaningful numbers:\n\n");
    out.push_str("```bash\n");
    out.push_str(&format!(
        "cargo run --release --bin xtask -- bench-sqlglot --mode {} --sqlglot /path/to/sqlglot\n",
        args.mode.as_str()
    ));
    out.push_str("```\n\n");
    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "- SQLGlot checkout: `{}`\n",
        args.sqlglot.display()
    ));
    if let Some(cases) = &args.cases {
        out.push_str(&format!("- Case file: `{}`\n", cases.display()));
    }
    out.push_str(&format!("- Mode: `{}`\n", args.mode.as_str()));
    out.push_str("- Baseline: `Python SQLGlot`\n");
    out.push_str(&format!("- Candidate: `{}`\n", args.mode.candidate_label()));
    out.push_str(&format!("- Cases: `{}`\n", cases.len()));
    out.push_str(&format!("- Iterations per case: `{}`\n", args.iterations));
    out.push_str(&format!(
        "- Warmup iterations per case: `{}`\n",
        args.warmup
    ));
    out.push_str(&format!("- Samples: `{}`\n", args.samples));
    out.push_str(&format!("- Total measured operations: `{operations}`\n"));
    out.push_str(&format!(
        "- Python SQLGlot median: `{:.3} us/op` (p95 `{:.3}`)\n",
        baseline.median_ns_per_op / 1_000.0,
        baseline.p95_ns_per_op / 1_000.0
    ));
    out.push_str(&format!(
        "- {} median: `{:.3} us/op` (p95 `{:.3}`)\n",
        args.mode.candidate_label(),
        candidate.median_ns_per_op / 1_000.0,
        candidate.p95_ns_per_op / 1_000.0
    ));
    out.push_str(&format!("- Median speedup: `{speedup:.2}x`\n"));
    out.push_str(&format!("- Output checksum: `{}`\n\n", candidate.checksum));

    let caveats = benchmark_caveats(args, per_case.is_some());
    if !caveats.is_empty() {
        out.push_str("## Measurement Caveats\n\n");
        for caveat in &caveats {
            out.push_str(&format!("- {caveat}\n"));
        }
        out.push('\n');
    }

    out.push_str("## Distribution\n\n");
    out.push_str("| runner | min us/op | mean us/op | median us/op | p95 us/op | max us/op |\n");
    out.push_str("| --- | ---: | ---: | ---: | ---: | ---: |\n");
    out.push_str(&format!(
        "| Python SQLGlot | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |\n",
        baseline.min_ns_per_op / 1_000.0,
        baseline.mean_ns_per_op / 1_000.0,
        baseline.median_ns_per_op / 1_000.0,
        baseline.p95_ns_per_op / 1_000.0,
        baseline.max_ns_per_op / 1_000.0
    ));
    out.push_str(&format!(
        "| {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |\n\n",
        args.mode.candidate_label(),
        candidate.min_ns_per_op / 1_000.0,
        candidate.mean_ns_per_op / 1_000.0,
        candidate.median_ns_per_op / 1_000.0,
        candidate.p95_ns_per_op / 1_000.0,
        candidate.max_ns_per_op / 1_000.0
    ));

    out.push_str("## Samples\n\n");
    out.push_str("| sample | Python SQLGlot us/op | candidate us/op | speedup |\n");
    out.push_str("| ---: | ---: | ---: | ---: |\n");
    for (baseline, candidate) in baseline.samples.iter().zip(&candidate.samples) {
        out.push_str(&format!(
            "| {} | {:.3} | {:.3} | {:.2}x |\n",
            baseline.sample,
            baseline.ns_per_op / 1_000.0,
            candidate.ns_per_op / 1_000.0,
            baseline.ns_per_op / candidate.ns_per_op
        ));
    }
    out.push('\n');

    if let Some(per_case) = per_case {
        let mut rows = per_case.iter().collect::<Vec<_>>();
        rows.sort_by(|left, right| {
            right
                .candidate
                .median_ns_per_op
                .total_cmp(&left.candidate.median_ns_per_op)
        });
        out.push_str("## Per-Case Breakdown\n\n");
        if args.mode == BenchMode::Core {
            out.push_str(
                "`core` per-case speedups are diagnostic only: the Rust candidate runs in-process, while the Python oracle runs through a subprocess for each one-row case. Use the candidate column to find slow Rust rows.\n\n",
            );
        }
        out.push_str("| id | Python us/op | candidate us/op | speedup | tags |\n");
        out.push_str("| --- | ---: | ---: | ---: | --- |\n");
        for row in rows {
            let tags = if row.tags.is_empty() {
                row.feature.clone()
            } else {
                row.tags.join(",")
            };
            out.push_str(&format!(
                "| `{}` | {:.3} | {:.3} | {:.2}x | `{}` |\n",
                row.id,
                row.baseline.median_ns_per_op / 1_000.0,
                row.candidate.median_ns_per_op / 1_000.0,
                row.median_speedup,
                tags
            ));
        }
        out.push('\n');
    }

    out.push_str("## Workload\n\n");
    out.push_str("| id | read | write | tags | SQL |\n");
    out.push_str("| --- | --- | --- | --- | --- |\n");
    for case in cases {
        let tags = if case.tags.is_empty() {
            case.feature.clone()
        } else {
            case.tags.join(",")
        };
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            case.id,
            case.read,
            case.write,
            tags,
            case.sql.replace('|', "\\|")
        ));
    }

    out
}

fn benchmark_caveats(args: &BenchSqlglotArgs, has_per_case: bool) -> Vec<&'static str> {
    let mut caveats = Vec::new();
    if has_per_case && args.mode == BenchMode::Core {
        caveats.push(
            "`--per-case --mode core` is a Rust profiler view, not a fair headline speedup: one-row Python oracle timings include poorly amortized subprocess startup/import cost.",
        );
    }
    if args.samples < 10 {
        caveats.push(
            "Fewer than 10 samples is directional evidence; rerun on a quiet machine with more samples before publishing performance claims.",
        );
    }
    caveats
}

fn render_benchmark_json(
    args: &BenchSqlglotArgs,
    cases: &[BenchCase],
    baseline: &BenchStats,
    candidate: &BenchStats,
    per_case: Option<&[BenchCaseComparison]>,
) -> Result<String, String> {
    let operations = args.iterations * cases.len();
    let caveats = benchmark_caveats(args, per_case.is_some());
    let report = serde_json::json!({
        "mode": args.mode,
        "sqlglot": args.sqlglot,
        "case_file": args.cases,
        "cases": cases,
        "case_count": cases.len(),
        "iterations": args.iterations,
        "warmup": args.warmup,
        "samples": args.samples,
        "operations": operations,
        "baseline": {
            "name": "Python SQLGlot",
            "stats": baseline,
        },
        "candidate": {
            "name": args.mode.candidate_label(),
            "stats": candidate,
        },
        "measurement_caveats": caveats,
        "median_speedup": baseline.median_ns_per_op / candidate.median_ns_per_op,
        "per_case": per_case,
    });
    serde_json::to_string_pretty(&report)
        .map_err(|err| format!("failed to serialize benchmark JSON: {err}"))
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
    python_oracle_transpile_sql(sqlglot, &case.sql, &case.read, &case.write)
}

fn python_oracle_transpile_sql(
    sqlglot: &PathBuf,
    sql: &str,
    read: &str,
    write: &str,
) -> Result<OracleOutput, String> {
    let script = r#"
import json
import sys

import sqlglot

payload = json.load(sys.stdin)
sql = payload["sql"]
read = payload["read"]
write = payload["write"]
try:
    out = sqlglot.transpile(sql, read=read, write=write)[0]
    print(json.dumps({"ok": True, "sql": out}))
except Exception as exc:
    print(json.dumps({"ok": False, "error": str(exc)}))
"#;

    let payload = serde_json::json!({
        "sql": sql,
        "read": read,
        "write": write,
    });
    let mut child = Command::new("python3")
        .arg("-c")
        .arg(script)
        .env("PYTHONPATH", sqlglot)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("failed to run Python SQLGlot oracle: {err}"))?;

    {
        let Some(mut stdin) = child.stdin.take() else {
            return Err("failed to open Python SQLGlot oracle stdin".to_string());
        };
        stdin
            .write_all(payload.to_string().as_bytes())
            .map_err(|err| format!("failed to write Python SQLGlot oracle stdin: {err}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("failed to wait for Python SQLGlot oracle: {err}"))?;

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

fn read_correctness_cases(path: &PathBuf) -> Result<Vec<CorrectnessCase>, String> {
    let text = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let mut cases = Vec::new();
    let mut ids = HashSet::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let case: CorrectnessCase = serde_json::from_str(line)
            .map_err(|err| format!("{}:{}: invalid JSON: {err}", path.display(), index + 1))?;
        if case.id.trim().is_empty() {
            return Err(format!(
                "{}:{}: id must not be empty",
                path.display(),
                index + 1
            ));
        }
        if !ids.insert(case.id.clone()) {
            return Err(format!("duplicate correctness case id {:?}", case.id));
        }
        if case.sql.trim().is_empty() {
            return Err(format!("{}: sql must not be empty", case.id));
        }
        if case.read.trim().is_empty() {
            return Err(format!("{}: read must not be empty", case.id));
        }
        for tag in &case.tags {
            if !is_valid_tag(tag) {
                return Err(format!(
                    "{}: invalid tag {:?}; use lowercase kebab-case",
                    case.id, tag
                ));
            }
        }
        cases.push(case);
    }
    if cases.is_empty() {
        return Err(format!("{} contains no correctness cases", path.display()));
    }
    Ok(cases)
}

fn evaluate_correctness_parity(
    sql: &str,
    read: &str,
    oracle_sql: &str,
) -> (Option<String>, CorrectnessParityStatus) {
    let Some(read) = Dialect::from_str(read) else {
        return (None, CorrectnessParityStatus::RustError);
    };
    let Some(write) = Dialect::from_str("sqlite") else {
        return (None, CorrectnessParityStatus::RustError);
    };

    match transpile(sql, read, write) {
        Ok(sqlgrok_sql) if sqlgrok_sql == oracle_sql => {
            (Some(sqlgrok_sql), CorrectnessParityStatus::Match)
        }
        Ok(sqlgrok_sql) => (Some(sqlgrok_sql), CorrectnessParityStatus::Mismatch),
        Err(_) => (None, CorrectnessParityStatus::RustError),
    }
}

fn run_sqlite(setup_sql: &str, oracle_sql: &str) -> Result<SqliteRun, String> {
    let mut input = String::new();
    if !setup_sql.trim().is_empty() {
        input.push_str(setup_sql.trim());
        if !input.ends_with(';') {
            input.push(';');
        }
        input.push('\n');
    }
    input.push_str(oracle_sql.trim());
    if !input.ends_with(';') {
        input.push(';');
    }
    input.push('\n');

    let output = Command::new("sqlite3")
        .arg(":memory:")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })
        .map_err(|err| format!("failed to run sqlite3: {err}"))?;

    let stdout = one_line(&String::from_utf8_lossy(&output.stdout));
    let stderr = one_line(&String::from_utf8_lossy(&output.stderr));
    Ok(SqliteRun {
        ok: output.status.success(),
        stdout: if stdout.is_empty() {
            None
        } else {
            Some(stdout)
        },
        stderr: if stderr.is_empty() {
            None
        } else {
            Some(stderr)
        },
    })
}

fn write_correctness_jsonl(path: &PathBuf, outcomes: &[CorrectnessOutcome]) -> Result<(), String> {
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

fn bucket_suite_report(input: &Path) -> Result<String, String> {
    let text = fs::read_to_string(input)
        .map_err(|err| format!("failed to read {}: {err}", input.display()))?;
    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let row: SuiteReportCase = serde_json::from_str(line).map_err(|err| {
            format!(
                "{}:{}: invalid suite report JSON: {err}",
                input.display(),
                index + 1
            )
        })?;
        rows.push(row);
    }

    let mut status_counts = BTreeMap::<String, usize>::new();
    let mut route_counts = BTreeMap::<(String, String, String), usize>::new();
    let mut helper_counts = BTreeMap::<(String, String), usize>::new();
    let mut source_counts = BTreeMap::<(String, String, String), usize>::new();
    let mut shape_counts = BTreeMap::<(String, String), usize>::new();
    let mut error_counts = BTreeMap::<(String, String), usize>::new();
    let mut mismatch_counts = BTreeMap::<(String, String), usize>::new();
    let mut examples = BTreeMap::<(String, String), Vec<&SuiteReportCase>>::new();

    for row in &rows {
        *status_counts.entry(row.status.clone()).or_default() += 1;
        *route_counts
            .entry((
                row.status.clone(),
                row.read.clone().unwrap_or_else(|| "none".to_string()),
                row.write.clone().unwrap_or_else(|| "none".to_string()),
            ))
            .or_default() += 1;
        *helper_counts
            .entry((row.status.clone(), row.helper.clone()))
            .or_default() += 1;
        *source_counts
            .entry((
                row.status.clone(),
                row.source_file.clone(),
                row.test_name.clone(),
            ))
            .or_default() += 1;
        *shape_counts
            .entry((row.status.clone(), suite_shape_key(&row.sql)))
            .or_default() += 1;

        if row.status == "rust-error" {
            let bucket = normalize_suite_error(row.error.as_deref());
            *error_counts
                .entry((row.status.clone(), bucket.clone()))
                .or_default() += 1;
            push_suite_example(&mut examples, row, bucket);
        } else if row.status == "mismatch" {
            let bucket = mismatch_signature(row);
            *mismatch_counts
                .entry((row.status.clone(), bucket.clone()))
                .or_default() += 1;
            push_suite_example(&mut examples, row, bucket);
        } else if row.status != "match" {
            let bucket = normalize_suite_error(row.error.as_deref());
            *error_counts
                .entry((row.status.clone(), bucket.clone()))
                .or_default() += 1;
            push_suite_example(&mut examples, row, bucket);
        }
    }

    let mut output = String::new();
    output.push_str("# SQLGlot Suite Bucket Report\n\n");
    output.push_str(&format!("Source: `{}`\n\n", input.display()));
    output.push_str(&format!("Total rows: `{}`\n\n", rows.len()));

    let top_routes = top_counts(&route_counts, 25);
    let top_helpers = top_counts(&helper_counts, 25);
    let top_shapes = top_counts(&shape_counts, 40);
    let top_errors = top_counts(&error_counts, 40);
    let top_mismatches = top_counts(&mismatch_counts, 40);
    let top_sources = top_counts(&source_counts, 40);

    output.push_str("## Status Counts\n\n");
    output.push_str("| Status | Count |\n| --- | ---: |\n");
    for (status, count) in &status_counts {
        output.push_str(&format!("| `{status}` | {count} |\n"));
    }

    write_count_table(
        &mut output,
        "Route Buckets",
        "| Status | Read | Write | Count |\n| --- | --- | --- | ---: |\n",
        top_routes.iter().map(|((status, read, write), count)| {
            format!("| `{status}` | `{read}` | `{write}` | {count} |")
        }),
    );
    write_count_table(
        &mut output,
        "Helper Buckets",
        "| Status | Helper | Count |\n| --- | --- | ---: |\n",
        top_helpers
            .iter()
            .map(|((status, helper), count)| format!("| `{status}` | `{helper}` | {count} |")),
    );
    write_count_table(
        &mut output,
        "SQL Shape Buckets",
        "| Status | Shape | Count |\n| --- | --- | ---: |\n",
        top_shapes
            .iter()
            .map(|((status, shape), count)| format!("| `{status}` | `{shape}` | {count} |")),
    );
    write_count_table(
        &mut output,
        "Rust/Oracle/Unsupported Error Buckets",
        "| Status | Error Bucket | Count |\n| --- | --- | ---: |\n",
        top_errors.iter().map(|((status, error), count)| {
            format!(
                "| `{status}` | `{}` | {count} |",
                markdown_escape_cell(error)
            )
        }),
    );
    write_count_table(
        &mut output,
        "Mismatch Signature Buckets",
        "| Status | Signature | Count |\n| --- | --- | ---: |\n",
        top_mismatches.iter().map(|((status, signature), count)| {
            format!(
                "| `{status}` | `{}` | {count} |",
                markdown_escape_cell(signature)
            )
        }),
    );
    write_count_table(
        &mut output,
        "Source Test Buckets",
        "| Status | Source | Test | Count |\n| --- | --- | --- | ---: |\n",
        top_sources.iter().map(|((status, source, test), count)| {
            format!("| `{status}` | `{source}` | `{test}` | {count} |")
        }),
    );

    output.push_str("\n## Bucket Examples\n\n");
    if examples.is_empty() {
        output.push_str("No non-matching examples.\n");
    } else {
        let mut example_keys = Vec::new();
        for ((status, bucket), _) in top_errors.iter().take(20) {
            example_keys.push((status.clone(), bucket.clone()));
        }
        for ((status, bucket), _) in top_mismatches.iter().take(20) {
            example_keys.push((status.clone(), bucket.clone()));
        }
        example_keys.sort();
        example_keys.dedup();

        for (status, bucket) in example_keys {
            let Some(rows) = examples.get(&(status.clone(), bucket.clone())) else {
                continue;
            };
            output.push_str(&format!(
                "### `{status}` `{}`\n\n",
                markdown_escape_cell(&bucket)
            ));
            for row in rows {
                output.push_str(&format!(
                    "- `{}`:{} `{}` via `{}`: {}\n",
                    row.source_file,
                    row.source_line,
                    row.test_name,
                    row.helper,
                    code_span(&row.sql)
                ));
                if let Some(expected) = &row.expected {
                    output.push_str(&format!("  - expected: {}\n", code_span(expected)));
                }
                if let Some(actual) = &row.actual {
                    output.push_str(&format!("  - actual: {}\n", code_span(actual)));
                }
                if let Some(error) = &row.error {
                    output.push_str(&format!("  - error: {}\n", code_span(error)));
                }
            }
            output.push('\n');
        }
    }

    Ok(output)
}

fn write_count_table(
    output: &mut String,
    title: &str,
    header: &str,
    rows: impl Iterator<Item = String>,
) {
    output.push_str(&format!("\n## {title}\n\n"));
    output.push_str(header);
    let mut wrote = false;
    for row in rows {
        output.push_str(&row);
        output.push('\n');
        wrote = true;
    }
    if !wrote {
        output.push_str("| _none_ | 0 |\n");
    }
}

fn push_suite_example<'a>(
    examples: &mut BTreeMap<(String, String), Vec<&'a SuiteReportCase>>,
    row: &'a SuiteReportCase,
    bucket: String,
) {
    let rows = examples.entry((row.status.clone(), bucket)).or_default();
    if rows.len() < 3 {
        rows.push(row);
    }
}

fn normalize_suite_error(error: Option<&str>) -> String {
    let Some(error) = error else {
        return "no error text".to_string();
    };
    let first_line = error.lines().next().unwrap_or(error).trim();
    if let Some(message) = first_line.strip_prefix("ValueError: Parser error: ") {
        return format!("parser: {}", normalize_parser_error(message));
    }
    if let Some(message) = first_line.strip_prefix("ParseError: ") {
        return format!("oracle parse: {}", normalize_parser_error(message));
    }
    if let Some(message) = first_line.strip_prefix("TokenError: ") {
        return format!("oracle token: {}", normalize_parser_error(message));
    }
    first_line.to_string()
}

fn normalize_parser_error(message: &str) -> String {
    let message = message.split(" at line ").next().unwrap_or(message);
    let message = message.split(". Line ").next().unwrap_or(message);
    if message.starts_with("Expected identifier") {
        return "Expected identifier".to_string();
    }
    if message.starts_with("Expected expression") {
        return "Expected expression".to_string();
    }
    if message.starts_with("Expected token") {
        return "Expected token".to_string();
    }
    if message.starts_with("Unexpected token") {
        return "Unexpected token".to_string();
    }
    if message.starts_with("Required keyword") {
        return "Required keyword missing".to_string();
    }
    message.to_string()
}

fn mismatch_signature(row: &SuiteReportCase) -> String {
    let expected = row.expected.as_deref().unwrap_or("");
    let actual = row.actual.as_deref().unwrap_or("");
    if actual.is_empty() {
        return "empty actual output".to_string();
    }
    if expected.eq_ignore_ascii_case(actual) && expected != actual {
        return "case-only rendering difference".to_string();
    }
    if unquote_sql(expected) == unquote_sql(actual) && expected != actual {
        return "quote-style difference".to_string();
    }
    if normalize_whitespace(expected) == normalize_whitespace(actual) && expected != actual {
        return "whitespace-only difference".to_string();
    }
    if expected.contains(" AS ") && !actual.contains(" AS ") {
        return "missing AS or alias rendering".to_string();
    }
    if expected.contains('"') && !actual.contains('"') {
        return "missing quoted identifier".to_string();
    }
    if expected.contains("CAST(") || actual.contains("CAST(") {
        return format!("cast/type rendering: {}", suite_shape_key(&row.sql));
    }
    if expected.contains("CREATE TABLE") || actual.contains("CREATE TABLE") {
        return "DDL/create-table rendering".to_string();
    }
    if expected.contains("CREATE INDEX") || actual.contains("CREATE INDEX") {
        return "DDL/create-index rendering".to_string();
    }
    if expected.contains("DATE")
        || expected.contains("TIME")
        || actual.contains("DATE")
        || actual.contains("TIME")
    {
        return format!("date/time rendering: {}", suite_shape_key(&row.sql));
    }
    if expected.contains("JSON")
        || actual.contains("JSON")
        || row.sql.to_ascii_uppercase().contains("JSON")
    {
        return format!("json rendering: {}", suite_shape_key(&row.sql));
    }
    suite_shape_key(&row.sql)
}

fn suite_shape_key(sql: &str) -> String {
    let trimmed = sql.trim();
    let upper = trimmed.to_ascii_uppercase();
    if upper.starts_with("SELECT ") {
        if let Some(function) = first_function_name(trimmed) {
            return format!("SELECT {function}()");
        }
        if let Some(operator) = first_operator_key(trimmed) {
            return format!("SELECT operator {operator}");
        }
    }
    feature_key(trimmed)
}

fn first_function_name(sql: &str) -> Option<String> {
    let open = sql.find('(')?;
    let before = &sql[..open];
    let name = before
        .chars()
        .rev()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '.')
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    if name.is_empty() || name.eq_ignore_ascii_case("SELECT") {
        None
    } else {
        Some(name.to_ascii_uppercase())
    }
}

fn first_operator_key(sql: &str) -> Option<&'static str> {
    for (needle, label) in [
        ("->>", "json-text"),
        ("->", "json"),
        ("||", "concat"),
        ("::", "cast"),
        (" AT TIME ZONE ", "at-time-zone"),
        ("[", "index"),
        (" + ", "plus"),
        (" - ", "minus"),
        (" * ", "multiply"),
        (" / ", "divide"),
        (" = ", "equals"),
    ] {
        if sql.contains(needle) {
            return Some(label);
        }
    }
    None
}

fn normalize_whitespace(sql: &str) -> String {
    sql.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn unquote_sql(sql: &str) -> String {
    sql.replace(['"', '`'], "")
}

fn markdown_escape_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', "\\n")
}

fn summarize_correctness(input: &Path, outcomes: &[CorrectnessOutcome]) -> String {
    let mut status_counts = BTreeMap::<String, usize>::new();
    let mut parity_counts = BTreeMap::<String, usize>::new();
    let mut tag_counts = BTreeMap::<(String, String), usize>::new();

    for outcome in outcomes {
        let status = outcome.status.as_str().to_string();
        *status_counts.entry(status.clone()).or_default() += 1;
        *parity_counts
            .entry(outcome.parity_status.as_str().to_string())
            .or_default() += 1;
        for tag in &outcome.case.tags {
            *tag_counts.entry((status.clone(), tag.clone())).or_default() += 1;
        }
    }

    let mut output = String::new();
    output.push_str("# SQLite Correctness Report\n\n");
    output.push_str(&format!("Source: `{}`\n\n", input.display()));
    output.push_str("This report runs Python SQLGlot's SQLite-targeted output against stock `sqlite3`. A `sqlite-error` row is not a sqlgrok parity failure by itself; it is a SQLite compatibility or upstream SQLGlot candidate to investigate.\n\n");
    output.push_str(&format!("Total candidates: `{}`\n\n", outcomes.len()));

    output.push_str("## Status Counts\n\n");
    output.push_str("| Status | Count |\n| --- | ---: |\n");
    for (status, count) in &status_counts {
        output.push_str(&format!("| `{status}` | {count} |\n"));
    }

    output.push_str("\n## SQLGlot Parity Counts\n\n");
    output.push_str("| Parity | Count |\n| --- | ---: |\n");
    for (status, count) in &parity_counts {
        output.push_str(&format!("| `{status}` | {count} |\n"));
    }

    output.push_str("\n## Tag Buckets\n\n");
    output.push_str("| Status | Tag | Count |\n| --- | --- | ---: |\n");
    for ((status, tag), count) in top_counts(&tag_counts, 25) {
        output.push_str(&format!("| `{status}` | `{tag}` | {count} |\n"));
    }

    output.push_str("\n## Candidates\n\n");
    for outcome in outcomes {
        output.push_str(&format!(
            "### `{}` ({})\n\n",
            outcome.case.id,
            outcome.status.as_str()
        ));
        output.push_str(&format!("- source: {}\n", code_span(&outcome.case.sql)));
        if let Some(oracle_sql) = &outcome.oracle_sql {
            output.push_str(&format!("- sqlglot sqlite: {}\n", code_span(oracle_sql)));
        }
        output.push_str(&format!(
            "- sqlgrok parity: `{}`\n",
            outcome.parity_status.as_str()
        ));
        if let Some(sqlgrok_sql) = &outcome.sqlgrok_sql
            && Some(sqlgrok_sql) != outcome.oracle_sql.as_ref()
        {
            output.push_str(&format!("- sqlgrok sqlite: {}\n", code_span(sqlgrok_sql)));
        }
        if let Some(stderr) = &outcome.sqlite_stderr {
            output.push_str(&format!("- sqlite stderr: {}\n", code_span(stderr)));
        }
        if let Some(stdout) = &outcome.sqlite_stdout {
            output.push_str(&format!("- sqlite stdout: {}\n", code_span(stdout)));
        }
        if !outcome.case.note.trim().is_empty() {
            output.push_str(&format!("- note: {}\n", outcome.case.note.trim()));
        }
        output.push('\n');
    }

    output
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

impl CorrectnessStatus {
    fn as_str(&self) -> &'static str {
        match self {
            CorrectnessStatus::SqliteOk => "sqlite-ok",
            CorrectnessStatus::SqliteError => "sqlite-error",
            CorrectnessStatus::OracleError => "oracle-error",
        }
    }
}

impl CorrectnessParityStatus {
    fn as_str(&self) -> &'static str {
        match self {
            CorrectnessParityStatus::Match => "match",
            CorrectnessParityStatus::Mismatch => "mismatch",
            CorrectnessParityStatus::RustError => "rust-error",
            CorrectnessParityStatus::OracleError => "oracle-error",
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
        .arg(SQLGLOT_AST_INVENTORY_SCRIPT)
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

dialect_dir = root / "tests" / "dialects"
test_files = [root / "tests" / "test_transpile.py"]
if dialect_dir.exists():
    test_files.extend(dialect_dir.glob("test_*.py"))
test_files = [path for path in dict.fromkeys(test_files) if path.exists()]

UNSUPPORTED = object()
UNKNOWN = object()

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
                            value = eval_static(stmt.value, {})
                            if isinstance(value, str):
                                return value
    return None

def eval_static(node, env):
    if node is None:
        return None
    if isinstance(node, ast.Constant):
        return node.value
    if isinstance(node, ast.Name):
        if node.id in env:
            return env[node.id]
        if node.id == "UnsupportedError":
            return UNSUPPORTED
        return UNKNOWN
    if isinstance(node, ast.Tuple):
        values = [eval_static(item, env) for item in node.elts]
        return UNKNOWN if any(value is UNKNOWN for value in values) else tuple(values)
    if isinstance(node, ast.List):
        values = [eval_static(item, env) for item in node.elts]
        return UNKNOWN if any(value is UNKNOWN for value in values) else values
    if isinstance(node, ast.Set):
        values = [eval_static(item, env) for item in node.elts]
        return UNKNOWN if any(value is UNKNOWN for value in values) else set(values)
    if isinstance(node, ast.Dict):
        result = {}
        for key_node, value_node in zip(node.keys, node.values):
            key = eval_static(key_node, env)
            value = eval_static(value_node, env)
            if key is UNKNOWN or value is UNKNOWN:
                return UNKNOWN
            result[key] = value
        return result
    if isinstance(node, ast.JoinedStr):
        parts = []
        for value in node.values:
            if isinstance(value, ast.Constant):
                parts.append(str(value.value))
            elif isinstance(value, ast.FormattedValue):
                rendered = eval_static(value.value, env)
                if rendered is UNKNOWN:
                    return UNKNOWN
                parts.append(str(rendered))
            else:
                return UNKNOWN
        return "".join(parts)
    if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Add):
        left = eval_static(node.left, env)
        right = eval_static(node.right, env)
        if left is UNKNOWN or right is UNKNOWN:
            return UNKNOWN
        try:
            return left + right
        except Exception:
            return UNKNOWN
    if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.USub):
        value = eval_static(node.operand, env)
        if isinstance(value, (int, float)):
            return -value
        return UNKNOWN
    if isinstance(node, ast.Attribute):
        base = eval_static(node.value, env)
        if base == "exp":
            return f"exp.{node.attr}"
        return UNKNOWN
    return UNKNOWN

def set_target(target, value, env):
    if value is UNKNOWN:
        return
    if isinstance(target, ast.Name):
        env[target.id] = value
    elif isinstance(target, (ast.Tuple, ast.List)) and isinstance(value, (tuple, list)):
        for item, item_value in zip(target.elts, value):
            set_target(item, item_value, env)

def feature_tags(sql):
    normalized = sql.strip().lower()
    tags = ["transpile", read, write, "imported"]
    if normalized.startswith(("create table", "create index", "drop index", "alter table", "drop table")):
        tags.append("ddl")
    if " index " in f" {normalized} " or normalized.startswith(("create index", "drop index")):
        tags.append("index")
    if "constraint" in normalized or " foreign key " in f" {normalized} " or " check " in f" {normalized} ":
        tags.append("constraint")
    if " join " in f" {normalized} " or " from " in f" {normalized} " and "," in normalized:
        tags.append("join")
    if "group_concat" in normalized or "string_agg" in normalized or "array_agg" in normalized:
        tags.append("aggregate")
    if "json" in normalized or "->" in normalized:
        tags.append("json")
    if "interval" in normalized or "date" in normalized or "time" in normalized:
        tags.append("time")
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

def source_for_validate(path, parents, call, kwargs, env):
    sql = eval_static(call.args[0], env) if call.args else None
    if not isinstance(sql, str):
        return None, None

    call_read = eval_static(kwargs.get("read"), env) if "read" in kwargs else None
    call_write = eval_static(kwargs.get("write"), env) if "write" in kwargs else None
    if call_read is UNKNOWN or call_write is UNKNOWN:
        return None, None

    class_dialect = class_dialect_for(parents, call)
    effective_read = call_read or class_dialect or ""
    effective_write = call_write or ""
    if effective_read == read and effective_write == write:
        return sql, "validate"
    return None, None

def source_for_validate_all(path, parents, call, kwargs, env):
    base_sql = eval_static(call.args[0], env) if call.args else None
    if not isinstance(base_sql, str):
        return None, None

    read_map = eval_static(kwargs.get("read"), env) if "read" in kwargs else {}
    write_map = eval_static(kwargs.get("write"), env) if "write" in kwargs else {}
    if read_map is UNKNOWN or write_map is UNKNOWN:
        return None, None
    if read_map and not isinstance(read_map, dict):
        return None, None
    if write_map and not isinstance(write_map, dict):
        return None, None
    if write in write_map and write_map[write] is UNSUPPORTED:
        return None, None

    class_dialect = class_dialect_for(parents, call) or ""
    if isinstance(read_map, dict) and read in read_map:
        source_sql = read_map[read]
        source_note = "validate_all:read-map"
    elif class_dialect == read:
        source_sql = base_sql
        source_note = "validate_all:class-dialect"
    elif not class_dialect and not read_map and read == "":
        source_sql = base_sql
        source_note = "validate_all:default-read"
    else:
        return None, None

    if not isinstance(source_sql, str):
        return None, None
    if isinstance(write_map, dict) and write in write_map:
        source_note += ":write-map"
    else:
        source_note += ":oracle-only"
    return source_sql, source_note

def source_for_validate_identity(path, parents, call, _kwargs, env):
    class_dialect = class_dialect_for(parents, call)
    if class_dialect != read:
        return None, None
    sql = eval_static(call.args[0], env) if call.args else None
    if not isinstance(sql, str):
        return None, None
    return sql, "validate_identity"

def cases_from_call(path, parents, call, env):
    name = method_name(call)
    kwargs = keyword_map(call)
    if name == "validate":
        sql, note = source_for_validate(path, parents, call, kwargs, env)
    elif name == "validate_all":
        sql, note = source_for_validate_all(path, parents, call, kwargs, env)
    elif name == "validate_identity":
        sql, note = source_for_validate_identity(path, parents, call, kwargs, env)
    else:
        return []

    if not isinstance(sql, str):
        return []
    return [make_case(path, call.lineno, test_name_for(parents, call), sql, note)]

def collect_from_expr(path, parents, expr, env, out):
    for node in ast.walk(expr):
        if isinstance(node, ast.Call):
            out.extend(cases_from_call(path, parents, node, env))

def walk_statements(path, parents, statements, env, out):
    for stmt in statements:
        if isinstance(stmt, ast.Assign):
            value = eval_static(stmt.value, env)
            for target in stmt.targets:
                set_target(target, value, env)
        elif isinstance(stmt, ast.AnnAssign):
            value = eval_static(stmt.value, env)
            set_target(stmt.target, value, env)
        elif isinstance(stmt, ast.For):
            iterable = eval_static(stmt.iter, env)
            if isinstance(iterable, (list, tuple, set)):
                for item in iterable:
                    child_env = dict(env)
                    set_target(stmt.target, item, child_env)
                    walk_statements(path, parents, stmt.body, child_env, out)
        elif isinstance(stmt, ast.With):
            walk_statements(path, parents, stmt.body, dict(env), out)
        elif isinstance(stmt, ast.If):
            walk_statements(path, parents, stmt.body, dict(env), out)
            walk_statements(path, parents, stmt.orelse, dict(env), out)
        elif isinstance(stmt, ast.Try):
            walk_statements(path, parents, stmt.body, dict(env), out)
            for handler in stmt.handlers:
                walk_statements(path, parents, handler.body, dict(env), out)
            walk_statements(path, parents, stmt.orelse, dict(env), out)
            walk_statements(path, parents, stmt.finalbody, dict(env), out)
        elif isinstance(stmt, ast.Expr):
            collect_from_expr(path, parents, stmt.value, env, out)
        else:
            for child in ast.iter_child_nodes(stmt):
                if isinstance(child, ast.expr):
                    collect_from_expr(path, parents, child, env, out)

cases = []
for path in sorted(test_files):
    module = ast.parse(path.read_text(), filename=str(path))
    parents = enclosing_tests(module)
    for node in module.body:
        if not isinstance(node, ast.ClassDef):
            continue
        for stmt in node.body:
            if isinstance(stmt, ast.FunctionDef) and stmt.name.startswith("test_"):
                walk_statements(path, parents, stmt.body, {"exp": "exp"}, cases)

cases.sort(key=lambda case: (case["source"], case["id"]))
deduped = []
seen_sql = set()
seen_ids = {}
for case in cases:
    key = (case["sql"], case["read"], case["write"])
    if key in seen_sql:
        continue
    seen_sql.add(key)
    base_id = case["id"]
    index = seen_ids.get(base_id, 0) + 1
    seen_ids[base_id] = index
    if index > 1:
        case["id"] = f"{base_id}-{index}"
    deduped.append(case)
    if limit and len(deduped) >= limit:
        break

print(json.dumps(deduped, sort_keys=False))
"#;

const SQLGLOT_BENCHMARK_SCRIPT: &str = r#"
import json
import sys
import time

sys.path.insert(0, sys.argv[1])
import sqlglot

payload = json.load(sys.stdin)
cases = payload["cases"]
iterations = int(payload["iterations"])
warmup = int(payload["warmup"])

checksum = 0
for _ in range(warmup):
    for case in cases:
        checksum += len(sqlglot.transpile(case["sql"], read=case["read"], write=case["write"])[0])

checksum = 0
started = time.perf_counter_ns()
for _ in range(iterations):
    for case in cases:
        checksum += len(sqlglot.transpile(case["sql"], read=case["read"], write=case["write"])[0])
elapsed_ns = time.perf_counter_ns() - started

print(json.dumps({"elapsed_ns": elapsed_ns, "checksum": checksum}))
"#;

const SQLGROK_PYTHON_BINDING_BENCHMARK_SCRIPT: &str = r#"
import json
import sys
import time

import sqlgrok

payload = json.load(sys.stdin)
cases = payload["cases"]
iterations = int(payload["iterations"])
warmup = int(payload["warmup"])

checksum = 0
for _ in range(warmup):
    for case in cases:
        checksum += len(sqlgrok.transpile(case["sql"], read=case["read"], write=case["write"])[0])

checksum = 0
started = time.perf_counter_ns()
for _ in range(iterations):
    for case in cases:
        checksum += len(sqlgrok.transpile(case["sql"], read=case["read"], write=case["write"])[0])
elapsed_ns = time.perf_counter_ns() - started

print(json.dumps({"elapsed_ns": elapsed_ns, "checksum": checksum}))
"#;

const SQLGROK_PYTHON_BINDING_BATCH_BENCHMARK_SCRIPT: &str = r#"
import json
import sys
import time

import sqlgrok

payload = json.load(sys.stdin)
cases = payload["cases"]
iterations = int(payload["iterations"])
warmup = int(payload["warmup"])
requests = [
    {"sql": case["sql"], "read": case["read"], "write": case["write"]}
    for case in cases
]

checksum = 0
for _ in range(warmup):
    for row in sqlgrok.transpile_many(requests):
        if not row["ok"]:
            raise RuntimeError(row["error"])
        checksum += len(row["sql"][0])

checksum = 0
started = time.perf_counter_ns()
for _ in range(iterations):
    for row in sqlgrok.transpile_many(requests):
        if not row["ok"]:
            raise RuntimeError(row["error"])
        checksum += len(row["sql"][0])
elapsed_ns = time.perf_counter_ns() - started

print(json.dumps({"elapsed_ns": elapsed_ns, "checksum": checksum}))
"#;

const SQLGLOT_AST_INVENTORY_SCRIPT: &str = r###"
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
