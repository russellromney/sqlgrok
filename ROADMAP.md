# sqlgrok Roadmap

sqlgrok's mission is to become a pure-Rust SQLGlot port that can run Python
SQLGlot's behavior suite directly, with every known divergence tracked and
ratcheted toward parity.

This roadmap is the public execution plan. Completed work belongs in
[CHANGELOG.md](CHANGELOG.md); generated details belong in `parity/reports/` or
`benchmarks/reports/`.

## Current Critical Path

1. Keep the Rust library, CLI, and curated parity regression corpus green.
2. Make the SQLGlot pytest bridge the authoritative parity surface.
3. Burn down forced-pair MySQL -> SQLite, Postgres -> SQLite, and SQLite ->
   SQLite reports by bucket.
4. Add row-level report diffing so budgets catch newly regressed cases, not only
   worse status counts.
5. Expand beyond transpilation into parse/generate, optimizer, and expression
   AST-shape suites.
6. Stabilize the Rust API, C ABI, and first language bindings around the same
   conformance and benchmark cases.
7. Prepare a clean `0.1.0` crate release with small docs, MIT licensing, clear
   attribution, and reproducible parity/performance commands.

## Operating Principles

- Python SQLGlot is the behavioral oracle until sqlgrok reaches mature parity.
- No output divergence is desired by default. If sqlgrok differs from Python
  SQLGlot for the same input and dialects, treat it as a bug or explicitly
  tracked backlog.
- Keep SQLGlot string parity separate from SQLite execution compatibility.
  A specific SQLite build may reject SQLGlot's SQLite-targeted output; that is
  useful evidence, but it does not change default sqlgrok output unless Python
  SQLGlot changes or sqlgrok adds an explicit compatibility mode.
- The Rust implementation stays native Rust. Python is allowed in tests,
  fixtures, suite adaptation, and benchmarking.
- Every parity fix should add one narrow Rust regression test and, when
  possible, one curated parity case.
- Progress should be measurable by report counts and row-level diffs:
  `match`, `mismatch`, `rust-error`, `oracle-error`, and
  `unsupported-harness-shape`.
- Do not let Rust ergonomics create output divergence from Python SQLGlot.
- Completed user-facing changes should update [CHANGELOG.md](CHANGELOG.md).

## Repository Map

- `src/parser/sql_parser.rs`: parser entry points and grammar behavior.
- `src/generator/sql_generator.rs`: SQL generation and dialect rendering.
- `src/dialects/`: dialect-specific functions, types, and time formats.
- `src/ast/types.rs`: AST shape and expression variants.
- `tests/test_transpile.rs`: focused transpiler regressions.
- `tests/sqlglot_parity.rs`: curated JSONL parity regression harness.
- `parity/cases/*.jsonl`: focused parity regression cases.
- `python/python/sqlgrok/sqlglot_bridge.py`: SQLGlot pytest bridge adapter.
- `parity/reports/`: generated suite and forced-pair reports.
- `parity/budgets/`: checked-in parity budgets.
- `benchmarks/cases/`: parity-clean benchmark workloads.
- `docs/PARITY.md`: parity contract, report formats, budgets, and dialect
  version policy.
- `docs/PERFORMANCE.md`: benchmark methodology and current numbers.

## Standard Session Loop

Use this loop for parity work:

1. Pick a report bucket or focused bug.
2. Confirm Python SQLGlot's oracle output for the same SQL and dialects.
3. Add or import the smallest reproducing case.
4. Confirm whether sqlgrok mismatches, errors, or already matches.
5. Fix the parser, AST, generator, or dialect mapping.
6. Add a focused Rust regression test near the owning behavior.
7. Run the relevant Rust tests and SQLGlot parity command.
8. Refresh the relevant report or budget when the change affects suite counts.
9. Update [CHANGELOG.md](CHANGELOG.md).

Do not describe a curated corpus run as the full SQLGlot suite. The suite bridge
and forced-pair reports are the visibility tools for broad parity.

## Testing Plan

### Rust Gate

```bash
cargo fmt --check
cargo clippy --features cli --all-targets -- -D warnings
cargo test --features cli
```

This gate proves the Rust codebase and focused regressions are healthy. It is
necessary but not sufficient for SQLGlot parity.

### Curated Parity Regression Corpus

```bash
SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_curated_parity --features cli -- --nocapture
```

This layer is for focused bug locks and reviewable fixtures, not as the
completion criterion for the project.

### SQLGlot Suite Bridge

```bash
cargo run --features cli --bin xtask -- run-sqlglot-suite \
  --sqlglot /path/to/sqlglot \
  --family transpile \
  --read postgres \
  --write sqlite \
  --check-budget \
  --pytest-arg -q
```

This is the primary parity lane. It adapts SQLGlot's pytest helper semantics and
compares the Rust backend with Python SQLGlot.

### Forced-Pair Backlog

```bash
cargo run --features cli --bin xtask -- run-sqlglot-suite \
  --sqlglot /path/to/sqlglot \
  --family transpile \
  --read mysql \
  --write sqlite \
  --force-pair \
  --pytest-arg -q

cargo run --features cli --bin xtask -- bucket-suite-report \
  --input parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl
```

Forced-pair mode is broad discovery. It replays pytest-discovered SQL through a
requested read/write pair using Python SQLGlot as the oracle, then produces a
bucketed burn-down backlog.

## Parity Milestones

### 1. Transpile Parity

Status: in progress.

Priority lanes:

- MySQL -> SQLite.
- Postgres -> SQLite.
- SQLite -> SQLite.

Current work:

- Reduce forced-pair `rust-error` buckets first, because parser coverage reveals
  the real mismatch backlog.
- Burn down high-volume mismatch clusters by feature family.
- Keep helper-route budgets clean for tracked lanes.
- Add row-level budget diffing so one fixed row cannot hide a new broken row.

### 2. Parse/Generate Identity

Status: planned.

Goal: run SQLGlot parse/generate identity tests through sqlgrok after the
transpile bridge is stable.

Deliverables:

- Bridge SQLGlot helper families outside transpilation.
- Preserve source metadata and classification fields.
- Add Rust AST/parser regressions for every fixed bucket.

### 3. Optimizer And Semantic Passes

Status: planned.

Goal: port SQLGlot optimizer behavior that matters for scope, qualification,
type annotation, projection expansion, predicate pushdown, and lineage.

Deliverables:

- Keep optimizer tests independent from transpiler tests unless the behavior
  explicitly depends on both.
- Add schema fixtures for qualification and type annotation.
- Separate string-exact parity from semantic equivalence where SQLGlot does.

### 4. AST Expansion

Status: planned.

Goal: let forced-suite failures and SQLGlot expression inventory drive AST work.

Deliverables:

- Inventory SQLGlot expression families into generated reports, not a
  hand-maintained public doc.
- Add AST nodes in small batches tied to parser/generator/serde/test coverage.
- Avoid lossy raw carriers when a construct is common enough to deserve a real
  representation.

## Parser Architecture Direction

Python SQLGlot remains the AST and behavior contract. Other parser projects may
inspire internals, but they do not define output behavior.

Useful ideas from the Databend parser article
[RisingWave Query Parser](https://www.databend.com/blog/category-engineering/2025-09-10-query-parser/):

- Keep parsing syntax-focused and defer semantic analysis to later passes.
- Track source spans on tokens and AST nodes so diagnostics can point to exact
  input ranges.
- Track the furthest parse error to report the most useful failure instead of
  the last incidental one.
- Use precedence-driven expression parsing so operator behavior is explicit and
  testable.
- Consider zero-copy token and AST representation only where profiling proves it
  matters.

Boundaries:

- Do not adopt another project's AST as the sqlgrok AST contract.
- Do not optimize for zero-copy representation before parity gaps are
  measurable.
- Do not mix semantic validation into parsing unless Python SQLGlot does so for
  the same case.

## Dialect Version Policy

SQLGlot dialects are dialect families, not exact server-version promises.
sqlgrok should still document the versions it tests against when execution
compatibility matters.

Initial public profiles:

- PostgreSQL: modern ORM-relevant Postgres, roughly Postgres 16+ syntax.
- MySQL: modern MySQL 8.x syntax and ORM/driver patterns.
- SQLite: stock SQLite with explicit notes for features that depend on recent
  versions, such as JSON operators, generated columns, window functions, STRICT
  tables, and optional math functions.
- libSQL/Turso: execution-compatibility profiles layered on SQLite string
  parity, not separate default transpilation oracles.

SQLGlot parity comes first. SQLite execution compatibility should be tracked as
a separate lane with its own profile and opt-in behavior decisions.

## Bindings Plan

The Rust core should become useful from other ecosystems through thin bindings,
not rewrites.

Stable first surface:

- `transpile(sql, read, write) -> Result<String>`.
- `transpile_many(requests) -> Vec<Result<String>>`.

Binding ladder:

1. Rust crate and CLI.
2. Python PyO3 package for SQLGlot-suite validation and Python users.
3. C ABI with `sqlgrok_*` symbols, keeping legacy `sqlglot_*` shims while
   compatibility matters.
4. Node/Bun through N-API, WebAssembly, or a C ABI bridge.
5. Ruby through Magnus or FFI.
6. Go through cgo or a generated C ABI wrapper.
7. .NET, Java/Kotlin, and Elixir once the ABI and conformance suite settle.

Every binding should run the same conformance corpus and have benchmark coverage
before it is described as production-ready.

## Release Plan

For `0.1.0`:

- Keep MIT licensing and upstream attribution clear.
- Publish only the small public doc surface:
  - `README.md`
  - `ROADMAP.md`
  - `CHANGELOG.md`
  - `docs/PARITY.md`
  - `docs/PERFORMANCE.md`
- Keep generated reports out of canonical docs.
- Ensure package metadata points to maintained docs.
- Run the Rust gate, curated parity corpus, and tracked SQLGlot suite bridge
  budgets before release.
