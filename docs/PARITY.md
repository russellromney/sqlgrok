# SQLGlot Parity Harness

sqlgrok uses Python SQLGlot as a behavioral oracle while keeping the implementation pure Rust.

## Case Format

Parity cases are JSON Lines files under `parity/cases/`:

```json
{"id":"mysql-group-concat-separator-to-sqlite","sql":"SELECT GROUP_CONCAT(v SEPARATOR '|') FROM gc","read":"mysql","write":"sqlite","tags":["transpile","mysql","sqlite","aggregate","function"],"source":"manual:orm-mysql-group-concat","mode":"transpile"}
```

Fields:

- `id`: stable case identifier.
- `sql`: source SQL.
- `read`: source dialect name.
- `write`: target dialect name.
- `tags`: optional lowercase kebab-case labels for filtering and reporting.
- `source`: optional source reference such as an upstream fixture path, issue id, or manual reproducer id.
- `mode`: optional harness mode. Currently only `transpile` is supported.
- `skip_reason`: optional reason to skip the case while preserving it in the corpus.
- `accepted_rust`: optional known-divergence output. If omitted, Rust must match Python exactly.
- `note`: optional explanation for known divergences.

## Running

Point the test at a Python SQLGlot checkout:

```bash
SQLGLOT_PYTHON_PATH=/path/to/sqlglot cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
```

If `SQLGLOT_PYTHON_PATH` is not set, the test also checks for a sibling checkout at `../sqlglot`.

Filter a run with environment variables:

```bash
SQLGROK_PARITY_ID=mysql-group-concat-separator-to-sqlite \
  SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_smoke_parity --features cli -- --nocapture

SQLGROK_PARITY_TAG=join \
  SQLGROK_PARITY_READ=mysql \
  SQLGROK_PARITY_WRITE=sqlite \
  SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
```

Supported filters:

- `SQLGROK_PARITY_ID`
- `SQLGROK_PARITY_TAG`
- `SQLGROK_PARITY_READ`
- `SQLGROK_PARITY_WRITE`

The harness rejects duplicate case ids and invalid tags. Tags must be lowercase kebab-case.

## Importing SQLGlot Fixtures

Use `xtask` to extract a small, deterministic batch from a local Python SQLGlot checkout:

```bash
cargo run --bin xtask -- import-sqlglot-fixtures \
  --sqlglot /path/to/sqlglot \
  --family transpile \
  --read mysql \
  --write sqlite \
  --limit 25 \
  --dry-run
```

Drop `--dry-run` to write the default output file:

```bash
cargo run --bin xtask -- import-sqlglot-fixtures \
  --sqlglot /path/to/sqlglot \
  --family transpile \
  --read mysql \
  --write sqlite \
  --limit 25
```

By default, imported cases are written to `parity/cases/transpile_<read>_<write>.jsonl`.
Use `--output` to choose a different file. The importer currently supports the `transpile`
family and reads straightforward `validate`, `validate_all`, and same-dialect
`validate_identity` cases from SQLGlot's Python tests.

Use `--only-matching` when you want a non-breaking seed file. That mode runs each imported
candidate through both Python SQLGlot and sqlgrok, then keeps only exact matches:

```bash
cargo run --bin xtask -- import-sqlglot-fixtures \
  --sqlglot /path/to/sqlglot \
  --family transpile \
  --read postgres \
  --write sqlite \
  --all \
  --only-matching
```

Use `--report-output` to make the rejected cases explicit instead of relying on
manual review. The report is JSONL with each candidate marked as `match`,
`mismatch`, `rust-error`, or `oracle-error`, including Python's expected output
and sqlgrok's actual output when available:

```bash
cargo run --bin xtask -- import-sqlglot-fixtures \
  --sqlglot /path/to/sqlglot \
  --family transpile \
  --read mysql \
  --write sqlite \
  --all \
  --only-matching \
  --report-output parity/reports/transpile_mysql_sqlite.jsonl
```

Summarize a report to choose the next work item:

```bash
cargo run --bin xtask -- summarize-report \
  --input parity/reports/transpile_mysql_sqlite.jsonl \
  --output parity/reports/transpile_mysql_sqlite.md
```

Imported cases include `source_file`, `source_line`, and `test_name` metadata so
larger batches can be traced back to the exact SQLGlot test. The importer also
adds feature tags for obvious DDL, index, and constraint cases.

## Ratchet

The intended workflow is:

1. Add or import a failing SQLGlot case.
2. Mark it with `accepted_rust` only when the divergence is intentional and documented.
3. Fix sqlgrok.
4. Remove `accepted_rust` once exact parity is reached.
5. Add a narrow Rust regression test for the fixed gap.
