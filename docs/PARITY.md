# SQLGlot Parity

sqlgrok uses Python SQLGlot as its behavioral oracle while keeping the
implementation pure Rust.

String parity is the default product contract: when sqlgrok transpiles the same
input and dialect pair as Python SQLGlot, the output should match exactly.

SQLite execution compatibility is a separate concern. If Python SQLGlot emits
SQLite-targeted SQL that a specific SQLite build rejects, that is useful
evidence for an upstream SQLGlot issue or an opt-in compatibility mode. It is
not, by itself, a reason for default sqlgrok output to diverge from SQLGlot.

## Coverage Model

sqlgrok has five testing layers:

1. **Rust tests**: focused parser, generator, dialect, optimizer, CLI, and
   regression coverage. These prove local behavior, but they are not the full
   SQLGlot suite.
2. **Curated parity regression corpus**: `tests/sqlglot_parity.rs` loads
   `parity/cases/*.jsonl`, calls Python SQLGlot for each source SQL, and
   requires sqlgrok to match exactly unless a row explicitly documents a known
   divergence. This is a reviewable bug-locking corpus.
3. **SQLGlot suite bridge**: the primary parity layer. It runs or adapts Python
   SQLGlot's pytest helpers against a `maturin` Python shim backed by sqlgrok.
4. **Forced-pair backlog reports**: broad discovery reports that replay
   pytest-discovered SQL through priority read/write pairs using Python SQLGlot
   as the oracle.
5. **SQLite execution compatibility**: optional checks that run SQLite-targeted
   SQL against concrete engines such as stock SQLite, libSQL, or Turso. These
   checks find portability and upstream-candidate issues, but they are not the
   SQLGlot parity oracle.

The curated corpus currently lives in `parity/cases/curated.jsonl` plus any
additional JSONL files under `parity/cases/`.

## Python Shim

The bridge depends on the package under `python/`:

```bash
uv run --project python --with maturin maturin develop --manifest-path python/Cargo.toml
uv run --project python python -c "import sqlgrok; print(sqlgrok.transpile('SELECT 1', read='postgres', write='sqlite'))"
```

The shim exposes a SQLGlot-shaped surface:

```python
sqlgrok.transpile(sql, read=None, write=None) -> list[str]
```

It also exposes batch APIs for suite tooling and binding benchmarks. The Rust
crate and CLI remain the primary product surface while the binding API settles.

## SQLGlot Suite Bridge

The bridge runner executes selected SQLGlot pytest modules from a local checkout
and adapts SQLGlot helper semantics. It currently focuses on transpilation and
helper calls such as:

- `validate`
- `validate_all`
- `validate_identity`

Each recorded row includes:

- source test file, test function, and source line;
- helper name;
- source SQL;
- read and write dialects;
- expected SQL according to Python SQLGlot;
- actual sqlgrok output when available;
- status: `match`, `mismatch`, `rust-error`, `oracle-error`, or
  `unsupported-harness-shape`.

Run a budgeted helper-route bridge:

```bash
cargo run --features cli --bin xtask -- run-sqlglot-suite \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --family transpile \
  --read postgres \
  --write sqlite \
  --check-budget \
  --pytest-arg -q
```

Reports are written to:

```text
parity/reports/sqlglot_suite_<family>_<read>_<write>.jsonl
parity/reports/sqlglot_suite_<family>_<read>_<write>.md
```

The Markdown summary records coverage accounting:

- observed helper attempts;
- rows filtered by read/write route;
- largest filtered route buckets;
- status counts for the requested lane.

These counts reconcile the pytest bridge with older static importer reports.
The pytest bridge is the long-term source of truth for upstream helper semantics.

## Forced-Pair Backlog

Forced-pair mode keeps pytest discovery but evaluates every captured source SQL
under the requested read/write pair. Expected output comes from a fresh Python
SQLGlot oracle call for that forced pair.

```bash
cargo run --features cli --bin xtask -- run-sqlglot-suite \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --family transpile \
  --read mysql \
  --write sqlite \
  --force-pair \
  --pytest-arg -q
```

Forced reports are written separately:

```text
parity/reports/sqlglot_suite_forced_<family>_<read>_<write>.jsonl
parity/reports/sqlglot_suite_forced_<family>_<read>_<write>.md
```

Summarize a forced report into implementation buckets:

```bash
cargo run --features cli --bin xtask -- bucket-suite-report \
  --input parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl
```

Use forced-pair reports to choose cluster-sized implementation slices. They are
broader than helper-route reports, but they are still oracle-backed.

## Budgets

CI should gate suite reports by checked-in budgets:

- fail if `rust-error`, `oracle-error`, or `unsupported-harness-shape`
  increases;
- fail if `mismatch` increases above the checked-in budget;
- allow reductions only when the budget is updated intentionally.

The next budget improvement is row-level diffing. Count-level budgets can miss a
bad trade where one row is fixed and a different row regresses. Row-level
budgets should use stable case identifiers derived from source file, line,
helper name, SQL, and requested dialect pair.

## Dialect Versions

SQLGlot dialects are dialect families. sqlgrok should document concrete
execution profiles separately from SQLGlot string parity.

Initial profiles:

- PostgreSQL: modern ORM-relevant Postgres, roughly Postgres 16+ syntax.
- MySQL: modern MySQL 8.x syntax and common ORM/driver SQL.
- SQLite: stock SQLite, with notes for JSON operators, generated columns, window
  functions, STRICT tables, and optional math functions.
- libSQL/Turso: execution-compatibility profiles layered on top of SQLite
  string parity, not separate default transpilation oracles.

For example, SQLGlot may output SQLite-dialect JSON operators that require a
recent SQLite build. That is not a string-parity failure. It belongs in an
execution-compatibility lane.

## Case Format

Curated parity cases are JSON Lines files under `parity/cases/`:

```json
{"id":"mysql-group-concat-separator-to-sqlite","sql":"SELECT GROUP_CONCAT(v SEPARATOR '|') FROM gc","read":"mysql","write":"sqlite","tags":["transpile","mysql","sqlite","aggregate","function"],"source":"manual:orm-mysql-group-concat","mode":"transpile"}
```

Fields:

- `id`: stable case identifier.
- `sql`: source SQL.
- `read`: source dialect name.
- `write`: target dialect name.
- `tags`: optional lowercase kebab-case labels for filtering and reporting.
- `source`: optional source reference such as an upstream fixture path, issue id,
  or manual reproducer id.
- `mode`: optional harness mode. Currently only `transpile` is supported.
- `skip_reason`: optional reason to skip the case while preserving it in the
  corpus.
- `accepted_rust`: optional known-divergence output. If omitted, Rust must match
  Python exactly.
- `note`: optional explanation for known divergences.

## Curated Corpus Commands

Run all curated cases:

```bash
SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_curated_parity --features cli -- --nocapture
```

Filter a run:

```bash
SQLGROK_PARITY_ID=mysql-group-concat-separator-to-sqlite \
  SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_curated_parity --features cli -- --nocapture

SQLGROK_PARITY_TAG=join \
  SQLGROK_PARITY_READ=mysql \
  SQLGROK_PARITY_WRITE=sqlite \
  SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_curated_parity --features cli -- --nocapture
```

Supported filters:

- `SQLGROK_PARITY_ID`
- `SQLGROK_PARITY_TAG`
- `SQLGROK_PARITY_READ`
- `SQLGROK_PARITY_WRITE`

The harness rejects duplicate case ids and invalid tags. Tags must be lowercase
kebab-case.

## Legacy Importer

`xtask import-sqlglot-fixtures` is a legacy ratchet tool. It can extract a
deterministic subset of SQLGlot transpile cases into JSONL, but it is not the
full suite and should not be described that way.

```bash
cargo run --bin xtask -- import-sqlglot-fixtures \
  --sqlglot /path/to/sqlglot \
  --family transpile \
  --read mysql \
  --write sqlite \
  --limit 25 \
  --dry-run
```

Use it for small curated fixture additions. Use the SQLGlot suite bridge and
forced-pair reports for broad parity visibility.
