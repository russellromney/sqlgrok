# SQLGlot Parity Harness

sqlgrok uses Python SQLGlot as a behavioral oracle while keeping the implementation pure Rust.

String parity is the default product contract: when sqlgrok transpiles the same input
and dialect pair as Python SQLGlot, the output should match exactly.

There is a separate cinch correctness lane for testing whether SQLGlot's SQLite-targeted
output is actually accepted by stock SQLite. See [CINCH_CORRECTNESS.md](CINCH_CORRECTNESS.md).
Those findings are upstream/cinch candidates and should not change sqlgrok's default
output unless Python SQLGlot changes or sqlgrok grows an explicit opt-in compatibility
mode.

## Coverage Model

There are two parity layers:

1. **Fast JSONL smoke parity**: `tests/sqlglot_parity.rs` loads `parity/cases/*.jsonl`,
   calls Python SQLGlot for each source SQL, then requires sqlgrok to match exactly.
   This is a regression corpus, not the full SQLGlot suite.
2. **SQLGlot suite bridge**: the planned authoritative layer runs/adapts Python
   SQLGlot's own pytest helpers against a `maturin` Python shim backed by sqlgrok.
   This bridge must preserve SQLGlot's helper semantics instead of relying on static
   extraction alone.

The old fixture importer remains useful for ratcheting and smoke generation, but it is
not sufficient as the project completion criterion.

## Python Shim

The first bridge dependency is a small Python package under `python/`:

```bash
uv run --project python python -c "import sqlgrok; print(sqlgrok.transpile('SELECT 1', read='postgres', write='sqlite'))"
```

The shim exposes a SQLGlot-shaped `sqlgrok.transpile(sql, read=None, write=None) -> list[str]`
surface. It is test infrastructure first; the stable Rust API and CLI remain the product
surface while the bridge matures.

## SQLGlot Suite Bridge

The bridge runner should execute selected SQLGlot pytest modules from a local checkout,
starting with transpilation. It should adapt or monkeypatch SQLGlot's `validate`,
`validate_all`, and `validate_identity` helpers so each upstream case records:

- source test file, test function, and source line;
- source SQL, read dialect, write dialect, and expected SQL according to SQLGlot's own
  helper logic;
- actual sqlgrok output through the Python shim;
- status: `match`, `mismatch`, `rust-error`, `oracle-error`, or
  `unsupported-harness-shape`.

The bridge output belongs in `parity/reports/sqlglot_suite_<family>_<read>_<write>.jsonl`
with a Markdown summary beside it.

The Markdown summary also records coverage accounting:

- `Observed helper attempts`: every SQLGlot helper call the bridge saw while running the
  selected upstream pytest modules.
- `Filtered by read/write`: helper calls whose read/write route did not match the
  requested bridge lane.
- `Filtered Routes`: the largest helper/read/write buckets that were seen but filtered.

These counts are the reconciliation layer between the pytest bridge and the legacy static
importer. The pytest bridge records SQLGlot's explicit helper routes; the importer can
force a requested read/write pair over many extracted SQL snippets to create a broader
ratchet backlog. The bridge is the long-term source of truth, while importer reports
remain useful for finding work until more SQLGlot helper shapes and bridge lanes are
adapted.

The bridge also has an explicit forced-pair mode for backlog discovery. In this mode,
pytest still discovers SQL through SQLGlot's helpers, but every captured source SQL is
re-evaluated as the requested `--read`/`--write` pair. The expected output comes from a
fresh Python SQLGlot oracle call for that forced pair, not from the helper's original
target string. These reports are written as
`parity/reports/sqlglot_suite_forced_<family>_<read>_<write>.jsonl` so they do not
replace the stricter helper-route budget reports.

CI should gate the bridge by budget:

- fail if `rust-error`, `oracle-error`, or `unsupported-harness-shape` increases;
- fail if `mismatch` increases above the checked-in budget;
- allow mismatch reductions only when the budget is updated intentionally.

The landed bridge runs the upstream transpile family, `tests/test_transpile.py` plus all
`tests/dialects/test_*.py` modules, with read/write filters selecting the rows that enter
the report:

```bash
cargo run --features cli --bin xtask -- run-sqlglot-suite \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --family transpile \
  --read postgres \
  --write sqlite \
  --check-budget \
  --pytest-arg -q
```

This is the first full-family transpile bridge for the tracked lanes. It still starts with
transpile helper semantics; parse/generate and optimizer helpers are future bridge lanes.

Run the forced-pair backlog view with `--force-pair`:

```bash
cargo run --features cli --bin xtask -- run-sqlglot-suite \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --family transpile \
  --read mysql \
  --write sqlite \
  --force-pair \
  --pytest-arg -q
```

Summarize a forced report into attackable buckets:

```bash
cargo run --features cli --bin xtask -- bucket-suite-report \
  --input parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl
```

The bucket report groups rows by status, helper, source test, SQL shape, normalized parser
error, and mismatch signature. Use it to choose cluster-sized implementation slices
instead of reading the full JSONL by hand.

As of the latest checked-in reports, the forced-pair bridge sees all `15,164` transpile
helper attempts for each tracked lane:

- MySQL->SQLite: `7,869` match, `3,870` mismatch, `1,545` rust-error, `1,743` oracle-error,
  `137` unsupported harness shape.
- Postgres->SQLite: `8,451` match, `3,503` mismatch, `1,616` rust-error, `1,457`
  oracle-error, `137` unsupported harness shape.
- SQLite->SQLite: `8,121` match, `3,789` mismatch, `1,568` rust-error, `1,549`
  oracle-error, `137` unsupported harness shape.

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

The importer is a legacy ratchet tool. Use `xtask` to extract a small, deterministic
batch from a local Python SQLGlot checkout:

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
family and scans SQLGlot's core transpile test plus dialect test files. It extracts
literal `validate`, `validate_all`, and dialect-class `validate_identity` calls, including
simple local variables, f-strings, and loops, then uses Python SQLGlot as the oracle for
the requested read/write dialect pair.

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

Use `--report-output` to make the full candidate backlog explicit instead of relying on
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
