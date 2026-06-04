use std::{collections::BTreeMap, env, fs, path::PathBuf};

use serde::Deserialize;
use sqlgrok::{
    Dialect, InternalFastPathStatus, transpile_internal_fast_experiment_status,
    transpile_internal_fast_guarded_status,
};

#[derive(Debug, Deserialize)]
struct Case {
    id: String,
    sql: String,
    read: String,
    write: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut cases = None;
    let mut guarded = false;
    for arg in env::args().skip(1) {
        if arg == "--guarded" {
            guarded = true;
        } else if cases.is_none() {
            cases = Some(PathBuf::from(arg));
        } else {
            return Err(format!("unexpected argument: {arg}"));
        }
    }
    let cases = cases.unwrap_or_else(|| PathBuf::from("benchmarks/cases/sqlite_sqlite.jsonl"));

    let input = fs::read_to_string(&cases)
        .map_err(|err| format!("failed to read {}: {err}", cases.display()))?;
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut total = 0usize;

    for (line_number, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let case: Case = serde_json::from_str(line).map_err(|err| {
            format!(
                "{}:{}: invalid JSONL row: {err}",
                cases.display(),
                line_number + 1
            )
        })?;
        let read = Dialect::from_str(&case.read)
            .ok_or_else(|| format!("{}: unknown read dialect {:?}", case.id, case.read))?;
        let write = Dialect::from_str(&case.write)
            .ok_or_else(|| format!("{}: unknown write dialect {:?}", case.id, case.write))?;

        let (status, _output) = if guarded {
            transpile_internal_fast_guarded_status(&case.sql, read, write)
        } else {
            transpile_internal_fast_experiment_status(&case.sql, read, write)
        }
        .map_err(|err| format!("{}: internal fast classification failed: {err}", case.id))?;

        total += 1;
        let label = status_label(status);
        *counts.entry(label).or_default() += 1;
        println!("{}\t{}", case.id, label);
    }

    println!();
    println!("total\t{total}");
    for (status, count) in counts {
        println!("{status}\t{count}");
    }

    Ok(())
}

fn status_label(status: InternalFastPathStatus) -> &'static str {
    match status {
        InternalFastPathStatus::Used => "used",
        InternalFastPathStatus::ParseDeclined => "parse-declined",
        InternalFastPathStatus::TransformDeclined => "transform-declined",
        InternalFastPathStatus::GenerateDeclined => "generate-declined",
        InternalFastPathStatus::OutputMismatch => "output-mismatch",
    }
}
