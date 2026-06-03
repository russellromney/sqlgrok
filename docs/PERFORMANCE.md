# Performance

sqlgrok should justify the Rust port with both parity and speed. The benchmark
lane compares Python SQLGlot with sqlgrok on parity-clean workloads, so the
numbers measure the same output contract rather than two different
transpilers.

Run benchmark commands with a release build for meaningful numbers:

```bash
cargo run --release --bin xtask -- bench-sqlglot \
  --mode core \
  --profile publishable \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --cases benchmarks/cases/postgres_sqlite.jsonl \
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

Use `--profile publishable` for stronger local timing reports. It expands the
defaults to `--iterations 5000 --warmup 500 --samples 10`; explicit
`--iterations`, `--warmup`, and `--samples` flags still win when provided.

Per-case reports need one extra caveat: `--per-case --mode core` is useful for
finding slow Rust rows, but it is not a fair headline speedup comparison. The
Rust candidate runs in-process while the Python SQLGlot oracle runs through a
subprocess for each one-row case, so Python startup/import cost is poorly
amortized. Use `python-binding` per-case reports for binding-to-binding
comparisons, and use `core` per-case reports as Rust-side profiling input.

## Language Binding Benchmarks

The prototype Node, Ruby, and Go bindings call the same release-built C ABI:

```bash
cargo build --release --lib
node bindings/node/bench.js --cases benchmarks/cases/postgres_sqlite.jsonl --samples 5
ruby bindings/ruby/bench.rb --cases benchmarks/cases/postgres_sqlite.jsonl --samples 5
cd bindings/go && go run . --cases ../../benchmarks/cases/postgres_sqlite.jsonl --samples 5
cc bindings/c/bench.c -Itarget/ffi/include -Ltarget/release -lsqlgrok -o target/sqlgrok_c_bench
DYLD_LIBRARY_PATH=target/release target/sqlgrok_c_bench --workload postgres --mode allocated
```

These bindings intentionally benchmark one `transpile(sql, read, write)` call at
a time. They do not use `transpile_many`, because most application integrations
will cross the boundary per request.

The public C ABI is:

- `sqlgrok_parse(sql, dialect)`.
- `sqlgrok_transpile(sql, read, write)`.
- `sqlgrok_generate(ast_json, dialect)`.
- `sqlgrok_version()`.
- `sqlgrok_free(ptr)`.

Returned strings must be freed with `sqlgrok_free`. The older `sqlglot_*`
symbols remain as compatibility aliases for early experiments.

## Current Snapshot

Fresh local release-build runs compare one Python SQLGlot `transpile(...)` call
with one sqlgrok binding call per SQL string. Each row used the checked-in
8-case workload, `--iterations 1000`, `--warmup 100`, and `--samples 5`.

| Workload | Python SQLGlot | PyO3 | Node/Koffi | Ruby/Fiddle | Go/cgo |
| --- | ---: | ---: | ---: | ---: | ---: |
| MySQL -> SQLite | 359.9 us | 9.7 us (37.1x) | 9.0 us (40.0x) | 17.2 us (21.0x) | 13.1 us (27.4x) |
| Postgres -> SQLite | 317.9 us | 10.4 us (30.4x) | 15.0 us (21.2x) | 16.1 us (16.7x) | 13.6 us (23.4x) |
| SQLite -> SQLite | 384.7 us | 9.7 us (39.6x) | 35.7 us (10.8x) | 66.3 us (5.8x) | 33.9 us (11.3x) |

The PyO3 numbers are the most mature binding data. The Node/Ruby/Go bindings
are deliberately thin prototypes over the C ABI, useful for overhead discovery
but not final package designs.

## Slowdown Investigation

The benchmark layer now has four tools for finding where time goes:

- `--per-case` on `bench-sqlglot` runs aggregate timing plus one timing pass per
  workload row.
- `sqlgrok_alloc_profile` counts allocated bytes and allocation calls for the
  Rust `sqlgrok::transpile(...)` path without perturbing normal timing runs.
- `bindings/c/bench.c` measures the direct C ABI without Node/Ruby/Go runtime
  overhead.
- `cargo bench --bench parser_bench` includes tokenize, parse, transform,
  generate, full transpile, and `transpile_many` phase benches for priority
  cases.

Run allocation profiling with:

```bash
cargo run --release --bin sqlgrok_alloc_profile -- \
  --cases benchmarks/cases/postgres_sqlite.jsonl \
  --phase transpile \
  --iterations 1000 \
  --warmup 100 \
  --per-case
```

The allocation report is not a wall-clock benchmark. Use it to identify which
cases allocate the most and then confirm timing wins with `bench-sqlglot` or
Criterion.

Use `--phase tokenize|parse|transform|generate|transpile` to isolate allocation
pressure. `transform` pre-parses before resetting counters, and `generate`
pre-parses plus transforms before resetting counters, so those phase reports do
not include earlier setup allocation.

Current allocation-profile snapshot, using the checked-in 8-case workloads with
`--iterations 1000 --warmup 100 --per-case`:

| Workload | Allocated | Allocations | Heaviest checked-in case |
| --- | ---: | ---: | --- |
| MySQL -> SQLite | 7.46 KiB/op | 118.62 allocs/op | `mysql-if-cast-div` at 14.81 KiB/op |
| Postgres -> SQLite | 6.88 KiB/op | 110.62 allocs/op | `postgres-distinct-on` at 10.94 KiB/op |
| SQLite -> SQLite | 6.63 KiB/op | 90.25 allocs/op | `sqlite-cte` at 15.34 KiB/op |

Example phase read on the current heaviest MySQL row, `mysql-if-cast-div`:

| Phase | Allocated | Allocations |
| --- | ---: | ---: |
| tokenize | 3.61 KiB/op | 30 allocs/op |
| parse | 6.56 KiB/op | 110 allocs/op |
| transform | 7.90 KiB/op | 72 allocs/op |
| generate | 0.35 KiB/op | 10 allocs/op |
| transpile | 14.81 KiB/op | 192 allocs/op |

That points the next optimization at MySQL expression transforms, not generator
string growth.

Current Postgres-to-SQLite per-case PyO3 medians:

| Case | Python SQLGlot | PyO3 | Speedup |
| --- | ---: | ---: | ---: |
| `postgres-window-nulls` | 224.2 us | 11.9 us | 18.9x |
| `postgres-distinct-on` | 352.7 us | 8.9 us | 39.8x |
| `postgres-extract-date-trunc` | 275.6 us | 8.3 us | 33.0x |
| `postgres-rollup` | 283.4 us | 8.3 us | 34.1x |
| `postgres-json-path-text` | 212.3 us | 8.0 us | 26.4x |
| `postgres-string-agg` | 241.6 us | 7.1 us | 34.0x |
| `postgres-identity-column` | 301.3 us | 6.8 us | 44.4x |
| `postgres-offset-only` | 198.7 us | 3.4 us | 57.6x |

Direct C ABI Postgres-to-SQLite medians:

| Mode | Median | p95 | Note |
| --- | ---: | ---: | --- |
| `allocated` | 7.3 us | 9.2 us | `sqlgrok_transpile` returning owned string plus `sqlgrok_free`. |
| `into` | 8.0 us | 10.1 us | `sqlgrok_transpile_into` writing into caller buffer. |
| `version` | 0.012 us | 0.021 us | Raw C call floor using `sqlgrok_version`. |

The caller-owned-buffer API is not faster yet because sqlgrok still builds the
Rust output `String` internally before copying. Returned C string allocation is
not the dominant cost.

Short Criterion phase run highlights:

| Case / phase | Median-ish result |
| --- | ---: |
| MySQL `GROUP_CONCAT` parse | ~10.2 us |
| MySQL `GROUP_CONCAT` transform | ~3.4 us |
| MySQL `GROUP_CONCAT` generate | ~0.8 us |
| Postgres `DISTINCT ON` parse | ~6.9 us |
| Postgres window/null-order parse | ~5.3 us |
| Postgres `DISTINCT ON` transform | ~2.8 us |
| Postgres identity-column parse | ~11.0 us |
| SQLite multi-CTE transpile | ~30.7 us |
| `transpile_many` priority cases | ~65.2 us for 4 cases |

The next real optimization targets are parser/token allocation and the
multi-CTE/full-transpile path. Generation is generally sub-microsecond in these
priority cases.

Short Criterion runs are diagnostic, not publication-grade measurements. Use
longer `--warm-up-time` / `--measurement-time` runs before claiming a regression
or improvement from phase benches.

If sqlgrok is not materially faster, profile the Rust path instead of assuming
the port is doomed. Useful targets include tokenizer allocation, parser
backtracking, AST cloning during dialect transforms, and generator string
growth.
