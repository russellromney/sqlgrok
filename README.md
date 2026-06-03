# sqlgrok

sqlgrok is a pure-Rust SQL parser, optimizer, and transpiler project with one
north star: pass Python SQLGlot's behavior suite directly while exposing the
same core logic through fast, portable bindings.

The implementation stays Rust. Python is used for development tooling: SQLGlot
oracle comparison, pytest-suite adaptation, and the `maturin` shim that lets
SQLGlot's own tests call into sqlgrok.

## Goals

- Keep a pure-Rust SQL transpiler library and CLI.
- Match Python SQLGlot exactly for the same input SQL and read/write dialects.
- Run SQLGlot's upstream behavior suite through a pytest bridge, starting with
  transpilation.
- Track matches, mismatches, Rust errors, oracle errors, unsupported harness
  shapes, and missing features explicitly.
- Convert every fixed parity gap into a focused Rust regression test.
- Make SQLGlot-style transpilation available to other language ecosystems
  through thin bindings over the Rust core.

## Quick Start

```bash
cargo test --features cli
echo "SELECT GROUP_CONCAT(v SEPARATOR '|') FROM gc" | \
  cargo run --features cli --bin sqlgrok -- transpile --read mysql --write sqlite
```

## Testing

The primary project test goal is SQLGlot parity, not a tiny quick check. The
test stack has distinct lanes:

- Rust tests: parser, generator, dialect, optimizer, CLI, and focused regression
  coverage.
- Curated parity corpus: handpicked JSONL regression cases in `parity/cases/`
  that compare sqlgrok with Python SQLGlot.
- SQLGlot pytest bridge: the main parity path, adapting SQLGlot helper calls
  such as `validate`, `validate_all`, and `validate_identity` against the Rust
  backend.
- Forced-pair backlog reports: broad discovery runs that replay pytest-discovered
  SQL through priority lanes such as MySQL -> SQLite, Postgres -> SQLite, and
  SQLite -> SQLite.
- SQLite execution compatibility: a separate lane for checking whether SQLGlot's
  SQLite-targeted output runs on stock SQLite, libSQL, or Turso. These findings
  do not redefine the parity oracle.

Run the normal Rust gate:

```bash
cargo fmt --check
cargo clippy --features cli --all-targets -- -D warnings
cargo test --features cli
```

Run the SQLGlot suite bridge against a local SQLGlot checkout:

```bash
cargo run --features cli --bin xtask -- run-sqlglot-suite \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --family transpile \
  --read postgres \
  --write sqlite \
  --check-budget \
  --pytest-arg -q
```

Run the curated parity regression corpus:

```bash
SQLGLOT_PYTHON_PATH=/Users/russellromney/Documents/Github/sqlglot \
  cargo test sqlglot_python_curated_parity --features cli -- --nocapture
```

See [docs/PARITY.md](docs/PARITY.md) for the full testing model, report formats,
budgets, forced-pair mode, and dialect-version notes.

## Performance Snapshot

sqlgrok's Rust core is already materially faster than Python SQLGlot on
parity-clean SQLite-targeted workloads. The current single-call binding
benchmark calls one `transpile(sql, read, write)` per SQL string rather than
using a batch shortcut.

| Workload | Python SQLGlot | PyO3 | Node/Koffi | Ruby/Fiddle | Go/cgo |
| --- | ---: | ---: | ---: | ---: | ---: |
| MySQL -> SQLite | 359.9 us | 9.7 us (37.1x) | 9.0 us (40.0x) | 17.2 us (21.0x) | 13.1 us (27.4x) |
| Postgres -> SQLite | 317.9 us | 10.4 us (30.4x) | 15.0 us (21.2x) | 16.1 us (16.7x) | 13.6 us (23.4x) |
| SQLite -> SQLite | 384.7 us | 9.7 us (39.6x) | 35.7 us (10.8x) | 66.3 us (5.8x) | 33.9 us (11.3x) |

These are local, release-build, five-sample medians over checked-in 8-case
workloads. PyO3 is the most mature binding; Node/Ruby/Go are thin prototype FFI
benches. See [docs/PERFORMANCE.md](docs/PERFORMANCE.md) for repeatable commands
and caveats.

## Docs

- [ROADMAP.md](ROADMAP.md): execution plan, architecture principles, bindings
  plan, release plan, and session loop.
- [CHANGELOG.md](CHANGELOG.md): quick summaries of completed changes.
- [docs/PARITY.md](docs/PARITY.md): SQLGlot parity contract, bridge workflow,
  report formats, budgets, and dialect-version policy.
- [docs/PERFORMANCE.md](docs/PERFORMANCE.md): sqlgrok vs Python SQLGlot
  benchmark methodology and current numbers.

Generated Markdown reports live under `parity/reports/` and
`benchmarks/reports/`. They are useful artifacts, but they are not canonical
project documentation.

## Python Shim

Build or refresh the local Python shim with `uv`:

```bash
uv run --project python --with maturin maturin develop --manifest-path python/Cargo.toml
uv run --project python python -c "import sqlgrok; print(sqlgrok.transpile('SELECT 1', read='postgres', write='sqlite'))"
```

The shim exists first for SQLGlot-suite validation. The Rust crate and CLI
remain the primary product surface while the binding APIs settle.

## C ABI

The public C ABI uses `sqlgrok_*` symbols and generated `sqlgrok.h` headers:

- `sqlgrok_parse`
- `sqlgrok_transpile`
- `sqlgrok_transpile_into`
- `sqlgrok_generate`
- `sqlgrok_version`
- `sqlgrok_free`

The earlier `sqlglot_*` symbols remain as compatibility aliases while the
binding prototypes migrate.

## Lineage

This project is derived from Protegrity's Rust SQLGlot port, which is inspired
by and derived from Python [SQLGlot](https://github.com/tobymao/sqlglot). See
[the upstream Rust project](https://github.com/protegrity/sql-glot-rust) for
the original implementation lineage. Both upstream projects are MIT licensed.
