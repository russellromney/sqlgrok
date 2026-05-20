use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = ImportArgs::parse(env::args().skip(1))?;
    args.validate()?;

    let mut cases = import_sqlglot_fixtures(&args)?;
    validate_cases(&cases)?;

    cases.sort_by(|left, right| left.id.cmp(&right.id));

    let jsonl = render_jsonl(&cases)?;
    eprintln!(
        "imported {} {} cases from {} (read={}, write={}, limit={}, dry_run={})",
        cases.len(),
        args.family,
        args.sqlglot.display(),
        args.read,
        args.write,
        args.limit,
        args.dry_run
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

#[derive(Debug)]
struct ImportArgs {
    sqlglot: PathBuf,
    family: String,
    read: String,
    write: String,
    limit: usize,
    dry_run: bool,
    output: PathBuf,
}

impl ImportArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut args = args.peekable();
        let Some(command) = args.next() else {
            return Err(Self::usage());
        };
        if command != "import-sqlglot-fixtures" {
            return Err(format!("unknown command {command:?}\n\n{}", Self::usage()));
        }

        let mut sqlglot = None;
        let mut family = None;
        let mut read = None;
        let mut write = None;
        let mut limit = 25;
        let mut dry_run = false;
        let mut output = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--sqlglot" => sqlglot = Some(next_value(&mut args, "--sqlglot")?.into()),
                "--family" => family = Some(next_value(&mut args, "--family")?),
                "--read" => read = Some(next_value(&mut args, "--read")?),
                "--write" => write = Some(next_value(&mut args, "--write")?),
                "--limit" => {
                    let raw = next_value(&mut args, "--limit")?;
                    limit = raw
                        .parse()
                        .map_err(|_| format!("--limit must be a positive integer, got {raw:?}"))?;
                }
                "--dry-run" => dry_run = true,
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
        if self.limit == 0 {
            return Err("--limit must be greater than zero".to_string());
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
        "usage: cargo run --bin xtask -- import-sqlglot-fixtures --sqlglot /path/to/sqlglot --family transpile --read mysql --write sqlite --limit 25 [--dry-run] [--output parity/cases/transpile_mysql_sqlite.jsonl]".to_string()
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
    mode: String,
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

def make_case(path, lineno, test_name, sql, source_note):
    rel = path.relative_to(root).as_posix()
    case_id = f"sqlglot-{slug(rel.replace('/', '-').replace('.py', ''))}-{lineno:04d}-{slug(test_name)}"
    return {
        "id": case_id,
        "sql": sql,
        "read": read,
        "write": write,
        "tags": ["transpile", read, write, "imported"],
        "source": f"sqlglot:{rel}:{lineno}:{test_name}:{source_note}",
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
    if len(deduped) >= limit:
        break

print(json.dumps(deduped, sort_keys=False))
"#;
