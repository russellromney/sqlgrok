# Performance

sqlgrok should justify the Rust port with both parity and speed. The benchmark
lane compares Python SQLGlot with sqlgrok on parity-clean workloads, so the
numbers measure the same output contract rather than two different
transpilers.

Run benchmark commands with a release build for meaningful numbers:

```bash
cargo run --release --bin xtask -- bench-sqlglot \
  --mode core \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --cases benchmarks/cases/postgres_sqlite.jsonl \
  --iterations 2000 \
  --warmup 100 \
  --samples 5
```

Use `--dry-run` to print the report instead of writing it.

## Modes

- `--mode core`: compares Python SQLGlot with in-process Rust
  `sqlgrok::transpile(...)` calls from the `xtask` binary.
- `--mode python-binding`: compares Python SQLGlot with one PyO3
  `sqlgrok.transpile(...)` call per SQL string. This is the honest
  request/response binding number most users should expect.
- `--mode python-binding-batch`: compares Python SQLGlot with the local PyO3
  `sqlgrok.transpile_many(...)` binding. This is the throughput ceiling for
  bulk callers and SQLGlot-suite tooling.

Python subprocesses default to `uv run --project python ... python`, so the
SQLGlot baseline and PyO3 binding modes use the same interpreter path. Install
or refresh the editable extension before benchmarking `python-binding` or
`python-binding-batch`:

```bash
uv run --project python --with maturin maturin develop --manifest-path python/Cargo.toml
uv run --project python python -c "import sqlgrok; print(sqlgrok.transpile_many([{'sql': 'SELECT 1', 'read': 'postgres', 'write': 'sqlite'}]))"
```

You can override the interpreter with `--python /path/to/python` or `PYTHON=...`.

## Workloads

Checked-in benchmark workloads live in:

- `benchmarks/cases/mysql_sqlite.jsonl`
- `benchmarks/cases/postgres_sqlite.jsonl`
- `benchmarks/cases/sqlite_sqlite.jsonl`

Each row is JSONL:

```json
{"id":"postgres-offset-only","sql":"SELECT x FROM y OFFSET 10","read":"postgres","write":"sqlite","tags":["limit","orm"]}
```

The command validates every workload case against Python SQLGlot before timing.
If a case no longer matches exactly, the benchmark fails instead of timing
different behavior. That keeps benchmark evidence tied to the same contract as
the parity harness.

## Reports

By default, `bench-sqlglot` writes:

- Markdown: `benchmarks/reports/sqlglot_comparison_<mode>.md`
- JSON: `benchmarks/reports/sqlglot_comparison_<mode>.json`

Override them with `--output` and `--json-output`.

The JSON report includes mode, case file, case count, iterations, warmup,
sample count, per-sample timings, min/mean/median/p95/max per-operation timing,
checksums, and median speedup:

```bash
cargo run --release --bin xtask -- bench-sqlglot \
  --mode python-binding \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --cases benchmarks/cases/mysql_sqlite.jsonl \
  --samples 10 \
  --output benchmarks/reports/mysql_sqlite_pyo3.md \
  --json-output benchmarks/reports/mysql_sqlite_pyo3.json
```

## Reading Results

Treat the default five-sample report as directional evidence, not a product
claim. For decisions, run with `--samples 10` or more on a quiet machine and
compare median and p95 timings across:

- `core` mode, which isolates Rust implementation speed.
- `python-binding` mode, which includes normal per-call PyO3 boundary cost.
- `python-binding-batch` mode, which shows the best-case bulk path that
  amortizes boundary cost via `transpile_many`.
- The three priority dialect pairs: MySQL-to-SQLite, Postgres-to-SQLite, and
  SQLite identity.

`bench-sqlglot` alternates Python-first and candidate-first sample order so the
same runner is not always helped or hurt by process/cache position. The headline
speedup is based on median ns/op, not the fastest sample.

## Current Snapshot

Fresh local release-build `python-binding` runs compare one Python SQLGlot
`transpile(...)` call with one PyO3-backed `sqlgrok.transpile(...)` call per SQL
string. Each row below used the checked-in 8-case workload, `--iterations 1000`,
`--warmup 100`, and `--samples 5`.

| Workload | Python SQLGlot median | sqlgrok PyO3 median | Median speedup |
| --- | ---: | ---: | ---: |
| MySQL -> SQLite | 784.7 us/op | 21.0 us/op | 37.3x |
| Postgres -> SQLite | 806.6 us/op | 21.2 us/op | 38.0x |
| SQLite -> SQLite | 585.4 us/op | 17.9 us/op | 32.8x |

If sqlgrok is not materially faster, profile the Rust path instead of assuming
the port is doomed. Useful targets include tokenizer allocation, parser
backtracking, AST cloning during dialect transforms, and generator string
growth.
