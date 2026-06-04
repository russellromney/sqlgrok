use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use serde::Deserialize;
use sqlgrok::{
    Dialect, InternalFastPathStatus, TranspileRequest, dialects, generate, parse,
    tokens::Tokenizer, transpile, transpile_internal_fast_experiment,
    transpile_internal_fast_guarded_status, transpile_many,
};
use std::fs;
use std::hint::black_box;

#[derive(Debug, Deserialize)]
struct BenchJsonCase {
    sql: String,
    read: String,
    write: String,
}

fn bench_parse_simple(c: &mut Criterion) {
    c.bench_function("parse simple SELECT", |b| {
        b.iter(|| {
            parse(
                black_box("SELECT a, b, c FROM users WHERE id > 10"),
                Dialect::Ansi,
            )
            .unwrap()
        })
    });
}

fn bench_parse_complex(c: &mut Criterion) {
    let sql = "SELECT u.id, u.name, o.total \
               FROM users AS u \
               INNER JOIN orders AS o ON u.id = o.user_id \
               WHERE u.active = TRUE AND o.total > 100 \
               ORDER BY o.total DESC \
               LIMIT 50";
    c.bench_function("parse complex SELECT with JOIN", |b| {
        b.iter(|| parse(black_box(sql), Dialect::Ansi).unwrap())
    });
}

fn bench_parse_cte(c: &mut Criterion) {
    let sql = "WITH cte AS (SELECT id, name FROM users WHERE active = TRUE) \
               SELECT cte.id, cte.name FROM cte WHERE cte.id > 10";
    c.bench_function("parse CTE query", |b| {
        b.iter(|| parse(black_box(sql), Dialect::Ansi).unwrap())
    });
}

fn bench_roundtrip(c: &mut Criterion) {
    let sql = "SELECT a, b FROM t WHERE a > 1 AND b < 10 ORDER BY a";
    c.bench_function("roundtrip parse+generate", |b| {
        b.iter(|| {
            let ast = parse(black_box(sql), Dialect::Ansi).unwrap();
            generate(&ast, Dialect::Ansi)
        })
    });
}

fn bench_transpile(c: &mut Criterion) {
    let sql = "SELECT CAST(x AS INT), SUBSTR(name, 1, 3) FROM users WHERE active = TRUE";
    c.bench_function("transpile Ansi -> Postgres", |b| {
        b.iter(|| transpile(black_box(sql), Dialect::Ansi, Dialect::Postgres).unwrap())
    });
}

fn bench_internal_fast_identity(c: &mut Criterion) {
    let sql = "SELECT a, b FROM t WHERE a > 10 ORDER BY b DESC LIMIT 10";
    let mut group = c.benchmark_group("internal_fast_identity");
    group.bench_function("public_transpile_sqlite_identity", |b| {
        b.iter(|| transpile(black_box(sql), Dialect::Sqlite, Dialect::Sqlite).unwrap())
    });
    group.bench_function("internal_fast_sqlite_identity", |b| {
        b.iter(|| {
            transpile_internal_fast_experiment(black_box(sql), Dialect::Sqlite, Dialect::Sqlite)
                .unwrap()
                .unwrap()
        })
    });
    group.finish();
}

fn sqlite_identity_cases() -> Vec<(String, Dialect, Dialect)> {
    let path = format!(
        "{}/benchmarks/cases/sqlite_sqlite.jsonl",
        env!("CARGO_MANIFEST_DIR")
    );
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let case: BenchJsonCase = serde_json::from_str(line).unwrap();
            let read = Dialect::from_str(&case.read).unwrap();
            let write = Dialect::from_str(&case.write).unwrap();
            (case.sql, read, write)
        })
        .collect()
}

fn bench_internal_fast_sqlite_workload(c: &mut Criterion) {
    let cases = sqlite_identity_cases();
    let supported = cases
        .iter()
        .filter(|(sql, read, write)| {
            matches!(
                transpile_internal_fast_guarded_status(sql, *read, *write)
                    .unwrap()
                    .0,
                InternalFastPathStatus::Used
            )
        })
        .cloned()
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("internal_fast_sqlite_workload");
    group.bench_function("public_transpile_all", |b| {
        b.iter(|| {
            let mut checksum = 0usize;
            for (sql, read, write) in black_box(&cases) {
                checksum = checksum.wrapping_add(transpile(sql, *read, *write).unwrap().len());
            }
            checksum
        })
    });
    group.bench_function("public_transpile_supported_only", |b| {
        b.iter(|| {
            let mut checksum = 0usize;
            for (sql, read, write) in black_box(&supported) {
                checksum = checksum.wrapping_add(transpile(sql, *read, *write).unwrap().len());
            }
            checksum
        })
    });
    group.bench_function("internal_fast_supported_only", |b| {
        b.iter(|| {
            let mut checksum = 0usize;
            for (sql, read, write) in black_box(&supported) {
                checksum = checksum.wrapping_add(
                    transpile_internal_fast_experiment(sql, *read, *write)
                        .unwrap()
                        .unwrap()
                        .len(),
                );
            }
            checksum
        })
    });
    group.bench_function("guarded_status_all", |b| {
        b.iter(|| {
            let mut checksum = 0usize;
            for (sql, read, write) in black_box(&cases) {
                let (status, output) =
                    transpile_internal_fast_guarded_status(sql, *read, *write).unwrap();
                checksum = checksum
                    .wrapping_add(status as usize)
                    .wrapping_add(output.as_ref().map_or(0, String::len));
            }
            checksum
        })
    });
    group.finish();
}

fn phase_cases() -> Vec<(&'static str, &'static str, Dialect, Dialect)> {
    vec![
        (
            "mysql_group_concat",
            "SELECT GROUP_CONCAT(v ORDER BY v SEPARATOR '|') FROM gc",
            Dialect::Mysql,
            Dialect::Sqlite,
        ),
        (
            "postgres_distinct_on",
            "SELECT DISTINCT ON (account_id) account_id, created_at FROM events ORDER BY account_id, created_at DESC",
            Dialect::Postgres,
            Dialect::Sqlite,
        ),
        (
            "postgres_identity_column",
            "CREATE TABLE users (id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY, created_at TIMESTAMP DEFAULT now())",
            Dialect::Postgres,
            Dialect::Sqlite,
        ),
        (
            "postgres_window_nulls",
            "SELECT user_id, ROW_NUMBER() OVER (PARTITION BY account_id ORDER BY created_at) FROM events",
            Dialect::Postgres,
            Dialect::Sqlite,
        ),
        (
            "sqlite_cte",
            "WITH a AS (SELECT 1 AS x), b AS (SELECT 2 AS y) SELECT a.x + b.y FROM a, b",
            Dialect::Sqlite,
            Dialect::Sqlite,
        ),
    ]
}

fn bench_priority_phases(c: &mut Criterion) {
    let mut group = c.benchmark_group("priority_phase_breakdown");
    for (id, sql, read, write) in phase_cases() {
        group.bench_with_input(BenchmarkId::new("tokenize", id), sql, |b, sql| {
            b.iter(|| {
                let mut tokenizer = Tokenizer::new(black_box(sql));
                tokenizer.tokenize().unwrap()
            })
        });

        group.bench_with_input(BenchmarkId::new("parse", id), sql, |b, sql| {
            b.iter(|| parse(black_box(sql), read).unwrap())
        });

        let ast = parse(sql, read).unwrap();
        group.bench_with_input(BenchmarkId::new("transform", id), &ast, |b, ast| {
            b.iter(|| dialects::transform(black_box(ast), read, write))
        });

        let transformed = dialects::transform(&ast, read, write);
        group.bench_with_input(BenchmarkId::new("generate", id), &transformed, |b, ast| {
            b.iter(|| generate(black_box(ast), write))
        });

        group.bench_with_input(BenchmarkId::new("transpile", id), sql, |b, sql| {
            b.iter(|| transpile(black_box(sql), read, write).unwrap())
        });
    }
    group.finish();
}

fn bench_transpile_many(c: &mut Criterion) {
    let requests = phase_cases()
        .into_iter()
        .map(|(_, sql, read, write)| TranspileRequest {
            sql: sql.to_string(),
            read,
            write,
            pretty: false,
        })
        .collect::<Vec<_>>();

    c.bench_function("transpile_many priority cases", |b| {
        b.iter(|| transpile_many(black_box(&requests)))
    });
}

criterion_group!(
    benches,
    bench_parse_simple,
    bench_parse_complex,
    bench_parse_cte,
    bench_roundtrip,
    bench_transpile,
    bench_internal_fast_identity,
    bench_internal_fast_sqlite_workload,
    bench_priority_phases,
    bench_transpile_many
);
criterion_main!(benches);
