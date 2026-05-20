# sqlgrok Roadmap

sqlgrok's mission is to become a pure-Rust SQLGlot port that can run Python SQLGlot's behavior suite directly, with every known divergence tracked and ratcheted toward parity.

This roadmap is organized around executable milestones. Each milestone should leave behind tests, fixtures, and documentation that make the next milestone easier.

## Operating Principles

- Python SQLGlot is the behavioral oracle until sqlgrok reaches mature parity.
- The Rust implementation stays native Rust; Python is allowed in tests, fixtures, and tooling.
- Every bug fix should add one narrow Rust regression test and, when possible, one parity case.
- Known divergences must be explicit in fixture metadata, not hidden in assertions.
- Progress should be measurable by counts: imported cases, exact matches, accepted divergences, unsupported cases, and regressions.

## Milestone 0: Project Foundation

Status: in progress.

Goal: make sqlgrok clearly its own project while preserving upstream attribution and release hygiene.

Deliverables:

- Rename crate, CLI, docs, package metadata, and generated artifacts to sqlgrok.
- Keep MIT licensing and clear NOTICE attribution to upstream Rust SQLGlot and Python SQLGlot.
- Keep CI green for normal Rust tests, CLI tests, doctests, and parity smoke tests.
- Maintain a small README that explains the mission and points to deeper docs.

Acceptance checks:

```bash
cargo test --features cli
rg "sqlglot-rust|sqlglot_rust|libsqlglot_rust|-lsqlglot_rust" .
```

The `rg` check should return no project-facing references. Upstream attribution links are allowed.

## Milestone 1: Parity Harness

Status: in progress.

Goal: make it easy to compare sqlgrok against a local Python SQLGlot checkout.

Deliverables:

- Keep the JSONL parity fixture format documented in [PARITY.md](PARITY.md).
- Support exact-match cases and documented accepted Rust divergences.
- Emit useful failure output: case id, dialects, source SQL, Python output, Rust output, and note.
- Add case filtering so a developer can run one fixture or one group quickly.
- Add summary output with totals for exact matches, accepted divergences, skipped cases, and failures.

Acceptance checks:

```bash
SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
```

## Milestone 2: SQLGlot Fixture Importer

Status: planned.

Goal: consume Python SQLGlot test data directly enough that updates from upstream are routine.

Deliverables:

- Add a fixture importer that reads selected Python SQLGlot test files and writes sqlgrok JSONL cases.
- Preserve source metadata: upstream file, test name, dialects, and expected SQL.
- Start with transpiler/generator fixtures before parser and optimizer fixtures.
- Provide an allowlist for supported fixture families and a skiplist for cases requiring missing APIs.
- Make generated fixtures deterministic so updates produce reviewable diffs.

Acceptance checks:

```bash
cargo run --bin xtask -- import-sqlglot-fixtures --sqlglot /path/to/sqlglot
cargo test sqlglot_python_smoke_parity --features cli
```

The exact command may change once the tooling exists, but the workflow should stay that simple.

## Milestone 3: Transpiler Parity Ratchet

Status: planned.

Goal: steadily close high-value cross-dialect gaps in parser and generator behavior.

Initial focus areas:

- MySQL to SQLite and MySQL to Postgres function mappings.
- Postgres to SQLite syntax that matters for pg-over-sqlite.
- Comma joins, explicit cross joins, join aliases, and multi-source FROM clauses.
- Aggregate functions such as `GROUP_CONCAT`, `STRING_AGG`, `ARRAY_AGG`, `COUNT(DISTINCT ...)`, and ordered aggregates.
- Limit/offset, casts, date/time functions, null-safe functions, and string functions.

Deliverables:

- Group parity fixtures by dialect pair and feature area.
- Convert fixed parity gaps into Rust regression tests under `tests/`.
- Add issue labels or fixture tags for `parser`, `generator`, `dialect`, `function`, `join`, and `aggregate`.
- Track a parity dashboard in generated Markdown or CI output.

Acceptance checks:

```bash
cargo test --features cli
SQLGLOT_PYTHON_PATH=/path/to/sqlglot cargo test sqlglot_python_smoke_parity --features cli
```

## Milestone 4: Parser Coverage

Status: planned.

Goal: parse the SQL shapes Python SQLGlot understands, even before every expression can be optimized or executed.

Deliverables:

- Import parser-oriented fixtures from Python SQLGlot.
- Classify unsupported grammar by feature area and dialect.
- Improve error messages with token position and surrounding context.
- Add round-trip tests for parsed ASTs where generation support already exists.

Acceptance checks:

- Parser fixtures either pass, are tagged unsupported with a reason, or have a linked issue.
- New grammar support includes direct parser tests and at least one generation or transpilation test when applicable.

## Milestone 5: AST Model Parity

Status: planned.

Goal: make the Rust AST expressive enough to represent SQLGlot's important expression families without lossy fallbacks.

Deliverables:

- Inventory Python SQLGlot expression classes against Rust AST variants.
- Add Rust AST nodes for high-frequency missing constructs.
- Keep serde output stable or versioned when AST shape changes.
- Add helper APIs for traversal, transformation, and expression construction as nodes are added.

Acceptance checks:

- The AST inventory document shows each expression as supported, partially supported, unsupported, or intentionally out of scope.
- New AST nodes have parser, generator, serde, and visitor coverage.

## Milestone 6: Optimizer And Semantic Passes

Status: planned.

Goal: bring over the SQLGlot passes that matter for correctness, compatibility, and useful programmatic SQL work.

Initial focus areas:

- Qualification and scope analysis.
- Type annotation.
- Predicate pushdown.
- Subquery unnesting.
- Projection expansion.
- Function normalization.
- Lineage and diff behavior.

Deliverables:

- Import selected optimizer fixtures from Python SQLGlot.
- Separate semantic equivalence tests from string-exact tests where SQLGlot permits alternate formatting.
- Add schema fixtures for qualification and type annotation.

Acceptance checks:

- Optimizer parity cases are grouped by pass.
- Regressions include the input SQL, schema if needed, expected output, and source SQLGlot fixture reference.

## Milestone 7: Compatibility APIs

Status: planned.

Goal: make sqlgrok pleasant to use as a Rust library, CLI, and FFI library while staying compatible with parity needs.

Deliverables:

- Stabilize the Rust public API around parse, generate, transpile, dialects, errors, AST traversal, and fixtures.
- Keep the CLI scriptable for parity and debugging workflows.
- Decide whether C FFI names remain `sqlglot_*` for compatibility or move to `sqlgrok_*` with aliases.
- Add examples for common cross-protocol use cases: MySQL-over-SQLite, Postgres-over-SQLite, and raw SQLite comparison.

Acceptance checks:

- Public examples compile in doctests or integration tests.
- CLI output is stable enough for harness and CI usage.

## Milestone 8: CI, Releases, And Upstream Sync

Status: planned.

Goal: make ongoing sync from Python SQLGlot and the upstream Rust port low-friction.

Deliverables:

- Add CI jobs for Rust tests, CLI tests, doctests, format, clippy, and parity smoke tests.
- Add an optional CI job that runs against a pinned Python SQLGlot revision.
- Add a script for updating imported fixtures from a new Python SQLGlot checkout.
- Generate release notes from merged parity improvements and fixed divergences.

Acceptance checks:

- A contributor can update fixtures, run tests, and open a PR without hand-editing generated fixture files.
- Release artifacts use sqlgrok names consistently.

## Near-Term Backlog

These are the next useful tickets to cut:

- Add parity case filtering by `id`, dialect pair, and tag.
- Add tags to parity fixture rows.
- Add a generated parity summary to test output.
- Import a small batch of Python SQLGlot transpiler cases for MySQL, Postgres, and SQLite.
- Add regression fixtures for comma joins and MySQL `GROUP_CONCAT`.
- Add CI for `cargo test --features cli`.
- Add an `xtask` or small Rust helper for fixture import and validation.
- Decide FFI naming compatibility policy.
- Add `cargo fmt --check` and `cargo clippy --features cli --all-targets` to the standard validation loop.

## Definition Of Done For Parity Fixes

A parity fix is complete when:

- The failing SQL is represented in `parity/cases/`.
- The Rust behavior matches Python SQLGlot or the divergence is explicitly accepted with a note.
- A focused Rust regression test covers the fixed behavior.
- `cargo test --features cli` passes.
- Any documentation or examples affected by the change are updated.
