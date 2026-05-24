# sqlgrok

sqlgrok is a pure-Rust SQL parser, optimizer, and transpiler project with one north star: pass Python SQLGlot's behavior suite directly.

It is currently bootstrapped from Protegrity's Rust SQLGlot port. The implementation stays Rust; Python is used only for test orchestration, SQLGlot oracle comparison, and the temporary `maturin` shim needed to run SQLGlot's own test semantics against the Rust backend.

## Goals

- Keep a pure-Rust SQL transpiler library and CLI.
- Run Python SQLGlot's behavior suite directly through a pytest bridge, starting with transpilation.
- Track exact matches, mismatches, Rust errors, oracle errors, unsupported harness shapes, and missing features explicitly.
- Convert every fixed parity gap into a focused Rust regression test.

## Quick Start

```bash
cargo test --features cli
echo "SELECT GROUP_CONCAT(v SEPARATOR '|') FROM gc" | cargo run --features cli --bin sqlgrok -- transpile --read mysql --write sqlite
```

## Python SQLGlot Parity

The project docs are split by purpose:

- [ROADMAP.md](ROADMAP.md): future work, execution sessions, and acceptance checks.
- [CHANGELOG.md](CHANGELOG.md): quick summaries of completed changes.
- [docs/PARITY.md](docs/PARITY.md): parity fixture format and SQLGlot oracle workflow.
- [docs/CINCH_CORRECTNESS.md](docs/CINCH_CORRECTNESS.md): SQLite execution checks for cinch/upstream candidates beyond SQLGlot string parity.
- [docs/PERFORMANCE.md](docs/PERFORMANCE.md): repeatable sqlgrok vs Python SQLGlot benchmark workflow.
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): parser architecture notes and outside influences.
- [docs/AST_INVENTORY.md](docs/AST_INVENTORY.md): SQLGlot expression coverage map.

The fast parity smoke test looks for Python SQLGlot in one of two places:

- `SQLGLOT_PYTHON_PATH=/path/to/sqlglot`
- a sibling checkout at `../sqlglot`

Run it with:

```bash
SQLGLOT_PYTHON_PATH=/Users/russellromney/Documents/Github/sqlglot \
  cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
```

The initial corpus lives in `parity/cases/smoke.jsonl`. Cases without an `accepted_rust` field must match Python SQLGlot exactly. Cases with `accepted_rust` document a known divergence while still checking that Rust behavior is stable.

The smoke corpus is not the full SQLGlot suite. It is a fast regression layer. The full-suite path is the SQLGlot pytest bridge described in [ROADMAP.md](ROADMAP.md) and [docs/PARITY.md](docs/PARITY.md): a `maturin` Python shim exposes `sqlgrok.transpile(...)`, then the bridge adapts SQLGlot's `validate`, `validate_all`, and `validate_identity` helpers into JSONL/Markdown reports with budgeted CI gates.

Build the Python shim locally with:

```bash
cd python
maturin develop
python -c "import sqlgrok; print(sqlgrok.transpile('SELECT 1', read='postgres', write='sqlite'))"
```

## Lineage

This project is derived from Protegrity's Rust SQLGlot port, which is inspired by and derived from Python [SQLGlot](https://github.com/tobymao/sqlglot). See [the upstream Rust project](https://github.com/protegrity/sql-glot-rust) for the original implementation lineage. Both upstream projects are MIT licensed.
