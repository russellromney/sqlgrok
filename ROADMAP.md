# sqlgrok Roadmap

sqlgrok's mission is to become a pure-Rust SQLGlot port that can run Python SQLGlot's behavior suite directly, with every known divergence tracked and ratcheted toward parity.

This roadmap is organized around executable milestones. Each milestone should leave behind tests, fixtures, and documentation that make the next milestone easier.

## Hostile Review Summary

The plan is viable, but implementation sessions need more than milestone names. A useful session should know the current objective, the files to inspect first, the command that proves the work, and the exact artifact to leave behind.

The current critical path is:

1. Keep the fast JSONL parity smoke suite green as a regression layer.
2. Build the Python `maturin` shim so SQLGlot's pytest helpers can call sqlgrok directly.
3. Add a SQLGlot pytest bridge that adapts `validate`, `validate_all`, and `validate_identity`.
4. Produce full-suite reports with `match`, `mismatch`, `rust-error`, `oracle-error`, and `unsupported-harness-shape` classifications.
5. Gate CI on a checked-in mismatch/error budget, then burn down MySQL/Postgres/SQLite transpiler mismatches from that report.
6. Expand the bridge beyond transpilation into parse, generate, and optimizer suites.

What this roadmap must prevent:

- "Port SQLGlot" as an unbounded task.
- Fixture dumps that produce giant, unreproducible diffs.
- Silent known divergences.
- String-normalization wins that hide semantic failures.
- CI/release artifacts that still ship under inherited upstream names.

## Operating Principles

- Python SQLGlot is the behavioral oracle until sqlgrok reaches mature parity.
- Cinch correctness checks are a separate lane: they can prove that SQLGlot's SQLite-targeted output is not stock-SQLite executable, but they do not change sqlgrok's default output away from SQLGlot parity without an explicit compatibility-mode decision.
- The Rust implementation stays native Rust; Python is allowed in tests, fixtures, and tooling.
- Every bug fix should add one narrow Rust regression test and, when possible, one parity case.
- Known divergences must be explicit in fixture metadata, not hidden in assertions.
- Progress should be measurable by counts: imported cases, exact matches, accepted divergences, unsupported cases, and regressions.
- Every implementation session should update either code, fixtures, or this roadmap. Do not leave discoveries only in chat.
- Completed user-facing changes should also update [CHANGELOG.md](CHANGELOG.md) with a short summary.

## Repository Map

Start here when opening a new implementation session:

- `tests/sqlglot_parity.rs`: Rust parity harness that calls Python SQLGlot.
- `parity/cases/*.jsonl`: parity corpus loaded by the smoke harness.
- `docs/PARITY.md`: fixture format and ratchet workflow.
- `docs/CINCH_CORRECTNESS.md`: SQLite execution checks for SQLGlot/cinch upstream candidates.
- `correctness/cases/*.jsonl`: cinch correctness candidates that run SQLGlot output against stock SQLite.
- `docs/ARCHITECTURE.md`: parser architecture notes and non-SQLGlot design influences.
- `CHANGELOG.md`: quick summaries of completed work.
- `tests/test_transpile.rs`: focused transpiler regressions.
- `src/parser/sql_parser.rs`: parser entry points and grammar behavior.
- `src/generator/sql_generator.rs`: SQL generation and dialect rendering.
- `src/dialects/`: dialect-specific functions, types, and time formats.
- `src/ast/types.rs`: AST shape and expression variants.
- `.github/workflows/`: CI and release packaging.

## Standard Session Loop

Use this loop for parity work:

1. Pick one ticket from "Next Implementation Sessions".
2. Add or import the smallest fixture that reproduces the gap.
3. Run the parity test and capture whether Rust differs, errors, or matches.
4. Fix the parser, AST, generator, or dialect mapping.
5. Add a focused Rust regression test near the owning behavior.
6. Run `cargo fmt`, `cargo test --features cli`, and the targeted parity command.
7. Update docs or fixture metadata if the behavior is still intentionally divergent.

Do not import a large upstream fixture family until filtering, tags, and summary output exist.

Use this loop for cinch correctness work:

1. Confirm sqlgrok's default output still matches Python SQLGlot.
2. Add the source SQL to `correctness/cases/` with minimal SQLite setup SQL.
3. Run `xtask check-sqlite-correctness` to prove whether SQLGlot's SQLite output executes.
4. If SQLite rejects the SQLGlot output, classify it as an upstream SQLGlot candidate, an explicit sqlgrok compatibility-mode candidate, or a downstream execution concern.
5. Do not alter default transpilation away from Python SQLGlot unless the chosen fix is to track an upstream SQLGlot change.

## Milestone 0: Project Foundation

Status: mostly complete.

Goal: make sqlgrok clearly its own project while preserving upstream attribution and release hygiene.

Deliverables:

- Rename crate, CLI, docs, package metadata, and generated artifacts to sqlgrok.
- Keep MIT licensing and clear NOTICE attribution to upstream Rust SQLGlot and Python SQLGlot.
- Keep CI green for normal Rust tests, CLI tests, doctests, and parity smoke tests.
- Maintain a small README that explains the mission and points to deeper docs.

Acceptance checks:

```bash
cargo test --features cli
rg --hidden "sqlglot-rust|sqlglot_rust|libsqlglot_rust|-lsqlglot_rust|target/release/sqlglot|bin/sqlglot" . --glob '!ROADMAP.md' --glob '!.git/**' --glob '!target/**'
```

The `rg` check should return no project-facing references. Upstream attribution links are allowed.

Remaining work:

- Decide whether FFI symbol names stay `sqlglot_*` for compatibility or gain `sqlgrok_*` aliases.
- Add a normal CI workflow for test, fmt, clippy, and parity smoke.

## Milestone 1: Parity Harness

Status: in progress.

Goal: make it easy to compare sqlgrok against a local Python SQLGlot checkout.

Deliverables:

- Keep the JSONL parity fixture format documented in [PARITY.md](docs/PARITY.md).
- Support exact-match cases and documented accepted Rust divergences.
- Emit useful failure output: case id, dialects, source SQL, Python output, Rust output, and note.
- Add case filtering by `SQLGROK_PARITY_ID`, `SQLGROK_PARITY_TAG`, `SQLGROK_PARITY_READ`, and `SQLGROK_PARITY_WRITE`.
- Add summary output with totals for exact matches, accepted divergences, skipped cases, and failures.
- Add fixture fields: `tags`, `source`, `mode`, and `skip_reason`.
- Validate fixture ids are unique and tags use a small documented vocabulary.

Acceptance checks:

```bash
SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
SQLGROK_PARITY_ID=mysql-group-concat-separator-to-sqlite \
  SQLGLOT_PYTHON_PATH=/path/to/sqlglot \
  cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
```

## Milestone 2: SQLGlot Suite Bridge

Status: supersedes the fixture-importer critical path.

Goal: run or adapt Python SQLGlot's actual pytest helper semantics against sqlgrok, rather than treating a partial static extractor as the suite.

Deliverables:

- Add a Python package/shim backed by Rust through `pyo3`/`maturin`, exposing at least `sqlgrok.transpile(sql, read=None, write=None) -> list[str]`.
- Add a pytest-driven runner that executes selected SQLGlot test modules from `/Users/russellromney/Documents/Github/sqlglot` or `SQLGLOT_PYTHON_PATH`.
- Adapt SQLGlot helper methods such as `validate`, `validate_all`, and `validate_identity` so each case compares SQLGlot's own expected SQL against sqlgrok's actual output.
- Preserve source metadata: upstream file, source line, test name, helper name, read/write dialects, input SQL, expected SQL, and actual SQL.
- Write JSONL reports to `parity/reports/sqlglot_suite_<family>_<read>_<write>.jsonl`.
- Classify every attempted upstream case as `match`, `mismatch`, `rust-error`, `oracle-error`, or `unsupported-harness-shape`.
- Add a forced-pair bridge mode that keeps pytest helper discovery but asks Python SQLGlot for a fresh oracle output for every discovered SQL under a requested read/write pair.
- Add a checked-in budget file so CI fails on new Rust errors, oracle errors, unsupported harness shapes, or mismatch-count regressions while allowing intentional mismatch burn-down.
- Keep the old fixture importer as a legacy ratchet/smoke-corpus generator, not as the definition of full-suite coverage.

Acceptance checks:

```bash
uv run --project python python -c "import sqlgrok; print(sqlgrok.transpile('SELECT 1', read='postgres', write='sqlite'))"

cargo run --bin xtask -- run-sqlglot-suite \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --family transpile \
  --read postgres \
  --write sqlite \
  --report-output parity/reports/sqlglot_suite_transpile_postgres_sqlite.jsonl

cargo run --bin xtask -- run-sqlglot-suite \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --family transpile \
  --read mysql \
  --write sqlite \
  --check-budget
```

The exact command may change as the bridge matures, but the workflow must run SQLGlot's
helper semantics rather than relying only on static SQL extraction.

### Legacy Fixture Importer

Status: useful but no longer the endgame.

The existing `xtask import-sqlglot-fixtures` command extracts a deterministic subset of
SQLGlot transpiler cases and feeds the fast JSONL parity smoke suite. Keep it for
reviewable regression fixtures, quick reports, and focused ratchets. Do not describe it
as "the full SQLGlot suite."

## Milestone 3: Transpiler Parity Ratchet

Status: in progress.

Goal: steadily close high-value cross-dialect gaps in parser and generator behavior.

Initial focus areas:

- MySQL to SQLite and MySQL to Postgres function mappings.
- Postgres to SQLite syntax that matters for pg-over-sqlite.
- Comma joins, explicit cross joins, join aliases, and multi-source FROM clauses.
- Aggregate functions such as `GROUP_CONCAT`, `STRING_AGG`, `ARRAY_AGG`, `COUNT(DISTINCT ...)`, and ordered aggregates.
- Limit/offset, casts, date/time functions, null-safe functions, and string functions.

First bugs to protect:

- `orm-pg-comma-cross-join`: `FROM cj a, cj b` must not lose the second table.
- `orm-mysql-group-concat`: `GROUP_CONCAT(...)` must not degrade into an invalid `GROUP_` function.
- Explicit `CROSS JOIN` must remain unaffected while comma joins are handled intentionally.

Deliverables:

- Group parity fixtures by dialect pair and feature area.
- Convert fixed parity gaps into Rust regression tests under `tests/`.
- Add issue labels or fixture tags for `parser`, `generator`, `dialect`, `function`, `join`, and `aggregate`.
- Track a parity dashboard in generated Markdown or CI output.
- Add a semantic execution check for SQLite-backed examples when string output alone is insufficient.
- Check in on SQLGlot issue [#7676](https://github.com/tobymao/sqlglot/issues/7676) and open a PR from `russellromney/codex/sqlite-postgres-escape-strings` once maintainers confirm the preferred escape-string modeling.

Acceptance checks:

```bash
cargo test --features cli
SQLGLOT_PYTHON_PATH=/path/to/sqlglot cargo test sqlglot_python_smoke_parity --features cli
```

String parity is the default. If sqlgrok intentionally emits semantically equivalent SQL with different spelling, the fixture must include `accepted_rust` and a note explaining why exact parity is deferred.

## Milestone 4: Parser Coverage

Status: planned.

Goal: parse the SQL shapes Python SQLGlot understands, even before every expression can be optimized or executed.

Architecture note: use the Databend parser article linked from [ARCHITECTURE.md](docs/ARCHITECTURE.md) as inspiration for spans, furthest-error tracking, zero-copy design, and syntax/semantic separation. Do not use it as the AST contract; Python SQLGlot remains the parity target.

Deliverables:

- Import parser-oriented fixtures from Python SQLGlot.
- Classify unsupported grammar by feature area and dialect.
- Improve error messages with token position and surrounding context.
- Add furthest-error tracking before broad grammar expansion.
- Preserve room for token/AST spans even if the first implementation stores them sparsely.
- Add round-trip tests for parsed ASTs where generation support already exists.
- Separate parser failures from generator failures in fixture metadata.

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
- Add `docs/AST_INVENTORY.md` before large AST expansion begins.

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
- Keep optimizer tests independent from transpiler tests unless the feature explicitly depends on both.

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
- Document compatibility policy for crate name, binary name, library artifact name, header name, and FFI symbol prefix.

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
- Ensure release tarballs, Debian packages, RPMs, and Homebrew formula use `sqlgrok` artifact names.

Acceptance checks:

- A contributor can update fixtures, run tests, and open a PR without hand-editing generated fixture files.
- Release artifacts use sqlgrok names consistently.

## Next Implementation Sessions

Cut these as issues or run them directly in order:

### Session 1: Harden The Parity Harness

Status: complete.

Files:

- `tests/sqlglot_parity.rs`
- `docs/PARITY.md`
- `parity/cases/smoke.jsonl`

Tasks:

- Add `tags`, `source`, `mode`, and `skip_reason` fields to `ParityCase`.
- Add environment filtering by id, tag, read dialect, and write dialect.
- Print a summary with exact matches, accepted divergences, skipped cases, and failures.
- Reject duplicate ids.

Done when:

```bash
SQLGROK_PARITY_ID=mysql-group-concat-separator-to-sqlite cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
cargo test --features cli
```

### Session 2: Add Standard CI

Status: complete.

Files:

- `.github/workflows/ci.yml`
- `.github/workflows/ffi-build.yml`
- `.github/workflows/release.yml`

Tasks:

- Add CI for `cargo fmt --check`, `cargo clippy --features cli --all-targets`, and `cargo test --features cli`.
- Add a parity smoke job that uses a pinned Python SQLGlot checkout or skips clearly when unavailable.
- Verify hidden workflow files do not contain inherited artifact names.

Done when:

```bash
cargo fmt --check
cargo clippy --features cli --all-targets
cargo test --features cli
rg --hidden "sqlglot-rust|sqlglot_rust|libsqlglot_rust|-lsqlglot_rust|target/release/sqlglot|bin/sqlglot" . --glob '!ROADMAP.md' --glob '!.git/**' --glob '!target/**'
```

### Session 3: Build The Fixture Importer Skeleton

Status: complete.

Files:

- `Cargo.toml`
- `xtask/` or `src/bin/xtask.rs`
- `docs/PARITY.md`
- `parity/cases/`

Tasks:

- Add an importer command with `--sqlglot`, `--family`, `--read`, `--write`, `--limit`, and `--dry-run`.
- Start with deterministic JSONL writing and fixture validation before parsing every upstream test shape.
- Import no more than 25 MySQL-to-SQLite transpiler cases in the first batch.
- Load all `parity/cases/*.jsonl` files from the parity harness so importer output is immediately runnable.

Done when:

```bash
cargo run --bin xtask -- import-sqlglot-fixtures --sqlglot /path/to/sqlglot --family transpile --read mysql --write sqlite --limit 25 --dry-run
cargo test sqlglot_python_smoke_parity --features cli
```

### Session 4: Ratchet Comma Join And GROUP_CONCAT

Status: complete.

Files:

- `parity/cases/*.jsonl`
- `tests/test_transpile.rs`
- `src/parser/sql_parser.rs`
- `src/generator/sql_generator.rs`
- `src/dialects/`

Tasks:

- Add or refine fixtures for comma joins, explicit cross joins, and MySQL `GROUP_CONCAT`.
- Ensure comma joins preserve all table sources and do not regress explicit `CROSS JOIN`.
- Ensure MySQL `GROUP_CONCAT` renders valid SQLite/Postgres output where supported.
- Add focused Rust regression tests for each fixed bug.
- Remove the accepted-divergence escape hatch from the smoke parity corpus.

Done when:

```bash
cargo test test_mysql_group_concat_to_sqlite --features cli
cargo test cross_join --features cli
SQLGLOT_PYTHON_PATH=/path/to/sqlglot cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
cargo test --features cli
```

### Session 5: AST Inventory

Status: complete.

Files:

- `docs/AST_INVENTORY.md`
- `src/ast/types.rs`
- `/path/to/sqlglot/sqlglot/expressions/`
- `src/bin/xtask.rs`

Tasks:

- Inventory Python SQLGlot expression classes against Rust AST variants.
- Mark each expression `supported`, `partial`, `unsupported`, or `out-of-scope`.
- Identify the top 10 missing AST constructs blocking transpiler fixtures.
- Add an `xtask inventory-ast` command so the inventory can be regenerated after upstream SQLGlot updates.

Done when:

- `docs/AST_INVENTORY.md` exists and is specific enough to drive AST expansion tickets.

### Session 6: Ratchet DDL And Type Normalization

Status: complete.

Files:

- `parity/cases/*.jsonl`
- `tests/test_transpile.rs`
- `src/ast/types.rs`
- `src/parser/sql_parser.rs`
- `src/generator/sql_generator.rs`
- `src/dialects/`
- `docs/AST_INVENTORY.md`

Tasks:

- Import or hand-add the smallest MySQL-to-SQLite DDL case that currently blocks the first imported fixture batch.
- Normalize MySQL `INT` to SQLite `INTEGER` where Python SQLGlot does.
- Preserve or intentionally discard MySQL table options such as `ENGINE`, `AUTO_INCREMENT`, `CHARACTER SET`, `COLLATE`, and `COMMENT`.
- Add focused Rust regression tests and update the AST inventory notes if the fix exposes a missing DDL node.

Done when:

```bash
SQLGLOT_PYTHON_PATH=/path/to/sqlglot SQLGROK_PARITY_TAG=ddl cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
cargo test ddl --features cli
cargo test --features cli
```

### Session 7: Split DDL Into First-Class AST Properties

Status: in progress.

Files:

- `src/ast/types.rs`
- `src/parser/sql_parser.rs`
- `src/generator/sql_generator.rs`
- `src/dialects/`
- `tests/test_transpile.rs`
- `parity/cases/*.jsonl`

Tasks:

- Replace the current parse-and-discard handling for MySQL `CREATE TABLE` options with explicit AST properties where SQLGlot keeps semantic information.
- Decide which properties are target-only rendering details, which are source metadata, and which should survive cross-dialect transforms.
- Add parser/generator coverage for common properties beyond the first ratchet: `ENGINE`, `CHARACTER SET`, `COLLATE`, `COMMENT`, `ROW_FORMAT`, and table-level `AUTO_INCREMENT`.
- Keep SQLite output behavior aligned with Python SQLGlot while allowing MySQL identity round-trips to preserve useful options.
- Add fixture metadata for options that Python SQLGlot warns about and intentionally drops for SQLite.

Landed:

- `CreateTableOption` exists in the AST.
- MySQL-family generation preserves `ENGINE`, table-level `AUTO_INCREMENT`, character set, collation, comment, row format, and unknown passthrough options.
- SQLite generation drops MySQL-only table options while preserving valid `AUTOINCREMENT` behavior.
- MySQL table-level single-column primary keys are moved inline for SQLite identity-column output.

Remaining:

- Decide how much warning/drop metadata should live in fixtures versus docs.
- Broaden DDL fixtures from hand-added ratchets into imported SQLGlot cases.

Done when:

```bash
SQLGROK_PARITY_TAG=ddl cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
cargo test create_table --features cli
cargo test --features cli
```

### Session 8: Expand DDL Parity To Indexes And Constraints

Status: in progress.

Files:

- `parity/cases/*.jsonl`
- `tests/test_transpile.rs`
- `src/ast/types.rs`
- `src/parser/sql_parser.rs`
- `src/generator/sql_generator.rs`
- `src/dialects/`

Tasks:

- Add focused Python SQLGlot parity cases for `CREATE INDEX`, `DROP INDEX`, `ALTER TABLE`, table constraints, foreign keys, check constraints, and default expressions.
- Split failures into parser gaps, AST representation gaps, generator gaps, and dialect-transform gaps.
- Implement the smallest high-value DDL family first, preferring exact SQLGlot string parity unless a fixture documents an intentional divergence.
- Add one Rust regression test for each closed parity gap.

Landed:

- Standalone `CREATE INDEX` and `DROP INDEX` statements have AST, parser, and generator support.
- Index parameters reuse `OrderByItem`, covering basic expression indexes and sort direction.
- Partial indexes parse and render `WHERE` predicates, with dialect/plugin transforms applied to the predicate expression.
- MySQL-to-SQLite parity fixtures cover standalone indexes, check constraints, foreign keys, and `ALTER TABLE ... ADD CONSTRAINT`.
- Focused Rust regression tests cover index round-trips and DDL constraint transpilation.

Remaining:

- Expand index support to included columns and dialect-specific index options.
- Add imported SQLGlot DDL batches once the importer can filter by feature tag before writing.
- Broaden `ALTER TABLE` action coverage beyond add/drop/rename/type/constraint basics.

Done when:

```bash
SQLGROK_PARITY_TAG=ddl cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
cargo test ddl --features cli
cargo test --features cli
```

### Session 9: Build The SQLGlot Test Bridge

Status: in progress.

Files:

- `python/pyproject.toml`
- `python/Cargo.toml`
- `python/src/lib.rs`
- `python/python/sqlgrok/__init__.py`
- `python/sqlgrok_sqlglot_bridge/`
- `src/bin/xtask.rs`
- `tests/sqlglot_parity.rs`
- `docs/PARITY.md`
- `parity/reports/`
- `parity/budgets/`

Tasks:

- Build a `maturin`/`pyo3` Python shim that exposes `sqlgrok.transpile(sql, read=None, write=None) -> list[str]`.
- Add a pytest bridge that runs selected SQLGlot tests from a local checkout and adapts SQLGlot's helper semantics.
- Start with transpilation and SQLGlot helpers `validate`, `validate_all`, and `validate_identity`.
- Emit JSONL and Markdown reports with `match`, `mismatch`, `rust-error`, `oracle-error`, and `unsupported-harness-shape`.
- Add a forced-pair mode that evaluates pytest-discovered SQL against a requested read/write pair using Python SQLGlot as the oracle.
- Add a budget file and CI check mode that fails on regressions without requiring all mismatches to be fixed immediately.
- Keep the legacy importer available, but treat it as smoke/regression tooling rather than full-suite coverage.

Landed:

- Imported fixtures now include `source_file`, `source_line`, and `test_name`.
- The importer auto-tags obvious DDL, index, and constraint cases.
- A first `python/` pyo3 shim exposes `sqlgrok.transpile(...)` for bridge work.
- The first pytest bridge patches `validate`, `validate_all`, and `validate_identity`, writes classified JSONL, and runs through `xtask run-sqlglot-suite` with a budget check.
- The bridge now defaults to full transpile-family module discovery and has checked-in budgets/reports for Postgres-to-SQLite, MySQL-to-SQLite, and SQLite identity.
- Forced-pair bridge reports now cover all `15,164` transpile helper attempts for MySQL-to-SQLite, Postgres-to-SQLite, and SQLite identity.

Remaining:

- Burn down the forced-pair report gaps by bucket, starting with the highest-volume Rust errors and mismatch clusters.
- Add report diffing so budget updates can highlight newly fixed and newly regressed rows, not only counts.
- Wire the bridge into CI after the local command is stable.

Done when:

```bash
cargo run --bin xtask -- run-sqlglot-suite --sqlglot /path/to/sqlglot --family transpile --read postgres --write sqlite --check-budget
cargo test --features cli
```

### Session 10: Parser Architecture Cleanup

Status: planned.

Files:

- `docs/ARCHITECTURE.md`
- `src/parser/sql_parser.rs`
- `src/tokens/`
- `tests/`

Tasks:

- Use the Databend parser article as design inspiration for clearer parse units, better error reporting, and syntax/semantic separation.
- Identify parser hot spots where one-off dialect branches are hiding reusable grammar structure.
- Introduce small helper APIs only where they reduce repeated parser branching or make parity failures easier to localize.
- Keep Python SQLGlot as the behavior contract; Databend is architecture inspiration, not an AST oracle.

Done when:

```bash
cargo test parser --features cli
cargo test sqlglot_python_smoke_parity --features cli
cargo test --features cli
```

### Session 11: Clippy And Documentation Debt Burn-Down

Status: planned.

Files:

- `src/`
- `tests/`
- `README.md`
- `ROADMAP.md`
- `CHANGELOG.md`

Tasks:

- Reduce the existing clippy warning backlog so new warnings become meaningful.
- Prioritize warnings that affect public API clarity, unsafe FFI docs, or confusing parser/generator code.
- Keep behavior-preserving cleanup commits separate from parity feature commits when practical.
- Update docs only for user-visible behavior, architecture decisions, or completed roadmap movement.

Done when:

```bash
cargo clippy --features cli --all-targets
cargo test --features cli
```

## Definition Of Done For Parity Fixes

A parity fix is complete when:

- The failing SQL is represented in `parity/cases/`.
- The Rust behavior matches Python SQLGlot or the divergence is explicitly accepted with a note.
- A focused Rust regression test covers the fixed behavior.
- `cargo test --features cli` passes.
- Any documentation or examples affected by the change are updated.
