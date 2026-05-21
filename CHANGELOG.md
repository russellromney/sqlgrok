# Changelog

Quick summaries of completed sqlgrok work. The roadmap says what should happen next;
this file records what landed.

## 2026-05-20

### Project Foundation

- Created the public `russellromney/sqlgrok` repository.
- Renamed the project-facing crate, CLI, metadata, docs, and package surfaces toward `sqlgrok`.
- Preserved MIT licensing and upstream attribution to Protegrity's Rust SQLGlot port and Python SQLGlot.
- Added README links to the upstream Rust SQLGlot repo and the project docs.
- Cleaned remaining `sql-glot-rust` / `sqlglot-rust` project references.

### Planning And Architecture

- Added [ROADMAP.md](ROADMAP.md) with executable parity milestones and implementation sessions.
- Hardened the roadmap with a hostile review pass so each session names files, tasks, and acceptance checks.
- Added [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), including the Databend parser article as inspiration for parser ergonomics while keeping Python SQLGlot as the behavior contract.

### Parity Harness

- Added JSONL parity metadata: `tags`, `source`, `mode`, `skip_reason`, `accepted_rust`, and `note`.
- Added parity filters via `SQLGROK_PARITY_ID`, `SQLGROK_PARITY_TAG`, `SQLGROK_PARITY_READ`, and `SQLGROK_PARITY_WRITE`.
- Added duplicate id and tag validation plus summary output.
- Changed the harness to load all `parity/cases/*.jsonl` files.

### CI And Tooling

- Added standard CI for format, clippy, tests, and pinned Python SQLGlot parity smoke.
- Added `xtask import-sqlglot-fixtures` for deterministic SQLGlot fixture extraction with `--dry-run`, `--limit`, `--read`, and `--write`.

### First Parity Ratchets

- Locked in MySQL `GROUP_CONCAT(... SEPARATOR ...)` to SQLite parity.
- Added `JoinType::Comma` so comma joins preserve SQLGlot string parity while remaining semantic cartesian joins in execution.
- Removed the accepted-divergence marker from the comma join smoke case.
- Reached smoke parity with `4/4` exact matches and `0` accepted divergences.

### Project Memory

- Moved the roadmap to top-level [ROADMAP.md](ROADMAP.md) so it sits beside README and CHANGELOG.

### AST Inventory

- Added `xtask inventory-ast` to compare Python SQLGlot's `sqlglot/expressions/` package against sqlgrok's Rust AST enums.
- Added [docs/AST_INVENTORY.md](docs/AST_INVENTORY.md) with coverage counts, priority gaps, module summaries, and a full generated inventory.
- Marked AST inventory complete in the roadmap and selected DDL/type normalization as the next ratchet.

### DDL And Type Normalization

- Added MySQL-to-SQLite DDL parity cases for `CREATE TABLE` table options, column options, type affinity, and `AUTO_INCREMENT` ordering.
- Taught the parser to consume common MySQL `CREATE TABLE (...)` options such as `ENGINE`, `AUTO_INCREMENT`, `DEFAULT CHARACTER SET`, `COLLATE`, and `COMMENT`.
- Added SQLite type normalization for integer, boolean, real, text, blob, decimal, and numeric column types.
- Fixed the CLI `transpile` path so it applies dialect transforms before generating output.

### DDL AST Properties

- Added a first-class `CreateTableOption` AST enum for MySQL-family table options.
- Preserved `ENGINE`, table-level `AUTO_INCREMENT`, character set, collation, comment, and row format options through MySQL round-trips.
- Kept SQLite generation dropping MySQL table options while preserving valid `AUTOINCREMENT` on integer primary keys.
- Added a table-level primary-key ratchet for MySQL `AUTO_INCREMENT` columns targeting SQLite.

### Roadmap Reconciliation

- Marked the initial parity harness session complete in [ROADMAP.md](ROADMAP.md).
- Split the next core work into executable sessions for DDL indexes/constraints, SQLGlot test bridging, parser architecture cleanup, and clippy/docs debt.
- Left future-looking work in the roadmap and kept completed DDL AST behavior summarized here.

### DDL Index And Constraint Ratchets

- Added `CREATE INDEX` and `DROP INDEX` AST, parser, and generator support, including unique indexes, `IF EXISTS`/`IF NOT EXISTS`, PostgreSQL `CONCURRENTLY`, optional `USING`, and MySQL `DROP INDEX ... ON table`.
- Added MySQL-to-SQLite parity cases for standalone indexes plus table/check/foreign-key constraint DDL.
- Extended the SQLGlot fixture importer with source file, source line, test name, and automatic DDL/index/constraint tags.
- Updated the AST inventory to mark standalone index statement coverage as partial.

### DDL Index Hostile Review Fixes

- Widened index parameters from bare names to `OrderByItem`, adding coverage for expression indexes and descending index keys.
- Added index statement support to table discovery, AST diffing, dialect/plugin transforms, and comment tests.
- Updated fixture importer inventory labels for `Create` and `Drop` index coverage.

### Partial Indexes

- Added partial-index support: `CREATE INDEX ... WHERE <predicate>` now parses, stores the predicate on the AST, applies dialect/plugin transforms to the predicate, and renders for SQLite/Postgres (previously a hard parse error). Added MySQL/SQLite-to-SQLite parity cases and a focused regression test.

### SQLite Function Parity

- Matched Python SQLGlot for Postgres `NOW()` to SQLite by rendering bare `CURRENT_TIMESTAMP`, while preserving MySQL `NOW()` to SQLite as `NOW()`.
- Matched Python SQLGlot for MySQL `IFNULL(...)` to SQLite by rewriting it to `COALESCE(...)`.

### MySQL LIMIT Parity

- Added parser support for MySQL comma limits (`LIMIT offset, count`) and normalize them to SQLGlot-style `LIMIT count OFFSET offset` when targeting SQLite.
