# Changelog

Quick summaries of completed sqlgrok work. The roadmap says what should happen next;
this file records what landed.

## 2026-05-22

### Parser Coverage Ratchet

- Added a separate cinch correctness lane with an `xtask check-sqlite-correctness` command, documentation, and seed cases that run Python SQLGlot's SQLite-targeted output against stock SQLite.
- Reduced imported SQLGlot rust-errors for SQLite->SQLite from 8 to 0, MySQL->SQLite from 73 to 8, and Postgres->SQLite from 112 to 62.
- Added parser/generator carriers for MySQL user variables, `:=`, `<=>`, `&&`, hex literals, qualified upsert/update assignments, `VALUES(...)`, and common cast/type suffix forms.
- Added Postgres tokenizer/parser support for dollar-quoted strings, psycopg `%s` parameters, `!~`/`!~*`, `VALUES` table sources, ordered aggregate argument carriers, window `EXCLUDE` parsing, and `IS DISTINCT FROM`.
- Refreshed the generated MySQL/Postgres/SQLite SQLGlot import reports so the remaining backlog is classified as concrete mismatches or narrower parser gaps.
- Covered `VALUES` table sources across parser, qualification, scope analysis, and planner tests.
- Cleared the current strict Clippy backlog so `cargo clippy --all-targets -- -D warnings` passes.
- Closed the remaining MySQL-to-SQLite parser-error backlog in the imported SQLGlot report: `8` rust-errors to `0`.
- Added MySQL parser/transpile support for `TRIM(... FROM ...)`, `CHAR(... USING ...)`, multi-target `DELETE ... USING`, and a raw `JSON_TABLE(...)` table-source carrier.
- Hardened the MySQL parser-carrier cleanup with SQLGlot parity for default `TRIM(LEADING|TRAILING FROM ...)`, empty `TRIM()` rejection, balanced `JSON_TABLE(...)` parsing, and SQLite `JSON_TABLE` `VARCHAR` to `TEXT` output.
- Added SQLite numeric parity for MySQL float division and Postgres `DIV(...)` integer-division casts.
- Preserved explicit `ASC` in `ORDER BY` items so SQLite identity and `NULLS LAST` orderings match SQLGlot output.
- Matched SQLGlot date-diff parity for SQLite `DATEDIFF(a, b, 'unit')` and SQLite-targeted `CAST(... AS DATE)` rendering as `DATE(...)`.
- Matched SQLGlot Postgres regex match operators `~`, `~*`, `!~`, and `!~*` when targeting SQLite.
- Hardened the date/regex parity ratchets with parity fixtures and edge coverage for Postgres `~~` aliases, double bitwise-not, invalid `~~x`, and non-unit three-argument `DATEDIFF`.
- Added a first-class Postgres `SIMILAR TO` predicate carrier so SQLite-targeted transpilation no longer drops the predicate as a bogus alias.
- Matched Postgres-to-SQLite `DATE_TRUNC`/`DATE_PART` time-function output while preserving SQLite identity behavior.
- Fixed `EXTRACT(... FROM CAST(... AS DATE))` child expression transforms for Postgres-to-SQLite parity.
- Added Postgres JSON path parity for `#>`, `#>>`, and `JSON_EXTRACT_PATH(_TEXT)` SQLite-targeted output.
- Hardened JSON path parity around cast-chained `#>`/`#>>`, quoted path segments, numeric path segments, and deterministic mixed-segment `JSON_EXTRACT_PATH(_TEXT)` path combinations.
- Added local executor support for a practical `SIMILAR TO` subset with `%`, `_`, explicit escapes, and common regex operators.
- Extended `SIMILAR TO` coverage for `NOT`, alternation, quantifiers, character classes, escaped regex operators, literal dots, and unbalanced delimiter literals.
- Added the remaining upstream SQLGlot `SIMILAR TO` identity cases from Redshift as explicit parity fixtures and Rust regressions.
- Added a dedicated `SIMILAR TO` parity corpus and a larger executor truth table covering wildcards, regex operators, ranges, POSIX character classes, escapes, and negative cases.
- Matched SQLGlot SQLite-targeted function rewrites for Postgres `strpos`, `chr`, `ascii`, `greatest`/`least`, `bool_and`/`bool_or`, `split_part`, `position`, `substring ... FROM ... FOR`, and MySQL `CURDATE()`.
- Matched Postgres-to-SQLite operator parity for `^` power and `#` bitwise XOR.
- Matched SQLGlot spacing for `ROLLUP (...)`, `CUBE (...)`, and `GROUPING SETS (...)` generation.
- Matched Postgres-to-SQLite array literal output by rendering `ARRAY[...]` as SQLGlot-style `ARRAY(...)`, including nested array literals.
- Matched SQLGlot normalization for raw Postgres `CREATE TYPE ... AS ENUM (...)` statements targeting SQLite.
- Added a Postgres `E'...'` escaped-string AST carrier and matched the fixed SQLGlot fork's quoted SQLite-targeted rendering.
- Added an `xtask bench-sqlglot` benchmark harness and performance notes for comparing sqlgrok against Python SQLGlot on parity-clean MySQL/Postgres-to-SQLite workloads.
- Extended MySQL `SIGNED`/`UNSIGNED` cast parity to the `SIGNED INTEGER` and `UNSIGNED INTEGER` spellings used by SQLGlot.
- Fixed MySQL `IF(...)` child expression transforms so nested casts and division rewrites still run before SQLite generation.
- Matched SQLGlot's order-sensitive MySQL `AUTO_INCREMENT` SQLite rendering for inline and table-level primary keys.
- Added a SQLite-targeted function/operator parity batch covering `POSITION`, `LOCATE`, `CONCAT`, schema functions, MySQL log helpers, Postgres JSON aggregates, `IS UNKNOWN`, and MySQL `XOR`.
- Made parsing honor dialect-specific `#` tokenization so Postgres bitwise XOR works while MySQL hash comments stay intact.
- Refreshed generated MySQL/Postgres-to-SQLite SQLGlot parity reports; MySQL now has `0` imported rust-errors, while Postgres is down to `57`.
- Matched a SQLite-targeted time-function batch for `MAKETIME`/`MAKE_TIME`, MySQL UTC current-time functions, `TIME_STR_TO_TIME`, Unix timestamp conversion, and ambiguous MySQL `%M`/`%W` time-format tokens.
- Hardened the SQLGlot parity harness so generated cases containing NUL bytes are sent to the Python oracle over stdin instead of argv.
- Matched another SQLGlot-imported SQLite time batch covering MySQL `TIMESTAMPDIFF`, formatted `FROM_UNIXTIME`, Postgres `TO_DATE`, formatted `TO_TIMESTAMP`, `TO_CHAR`, and time-stepped `GENERATE_SERIES`.

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

### Postgres Locking Read Parity

- Added `FOR UPDATE` parsing/generation for Postgres-style locking reads and drop the clause when targeting SQLite to match Python SQLGlot.

### MySQL REPLACE Parity

- Added parser and generator support for MySQL/SQLite `REPLACE INTO` statements.

### MySQL IF Parity

- Added parser support for MySQL `IF(condition, true, false)` expressions and render them as SQLite `IIF(...)` when targeting SQLite.

### MySQL SIGNED Cast Parity

- Mapped MySQL `CAST(... AS SIGNED)` to SQLite `CAST(... AS INTEGER)` to match Python SQLGlot.

### SIGNED Cast Hostile Review Fix

- Scoped `SIGNED` cast normalization to MySQL-family reads so Postgres/SQLite unknown `SIGNED` casts stay preserved when targeting SQLite.

### MySQL DATE_FORMAT Parity

- Render MySQL `DATE_FORMAT(expr, format)` as SQLite `STRFTIME(format, expr)` when targeting SQLite.

### SQLite Transpile Report Baseline

- Added the generated SQLite-to-SQLite transpile report for the current SQLGlot importer coverage.
- Applied dialect transforms to standalone expression statements, closing a class of generated-report mismatches.
- Matched Python SQLGlot for MySQL/Postgres bit aggregates targeting SQLite: `BIT_AND`, `BIT_OR`, `BIT_XOR`, and MySQL `BIT_COUNT`.

### Parser Carrier Ratchet

- Added an opaque raw statement carrier for unsupported command and DDL shapes so generated reports distinguish parser gaps from transpiler mismatches.
- Preserved MySQL/Postgres/SQLite command families such as `SET`, `ANALYZE`, `GRANT`, `REVOKE`, `CREATE VIRTUAL TABLE`, and unsupported `CREATE`/`ALTER` forms instead of failing at parse time.
- Added SQLite `INSERT OR ...` identity coverage and fixed raw SQL extraction for non-ASCII source text.

### Postgres STRING_AGG Parity

- Map Postgres `string_agg(expr, separator)` to SQLite `GROUP_CONCAT(expr, separator)`.

### SQLite JSON Path Parity

- Normalize Postgres JSON arrow paths for SQLite output and render SQLite JSON extract functions with `->`/`->>` operators.

### Postgres UUID Function Parity

- Map Postgres `gen_random_uuid()` to SQLite `UUID()` to match Python SQLGlot.

### Postgres Interval Literal Parity

- Split Postgres packed interval literals such as `INTERVAL '1 day'` into SQLite-style `INTERVAL '1' DAY`.

### Postgres ORDER BY Null Ordering Parity

- Add SQLGlot-style default `NULLS LAST` / `NULLS FIRST` ordering for Postgres-to-SQLite `ORDER BY` clauses, including window specs.

### Postgres ON CONFLICT Spacing Parity

- Render dialect-targeted `ON CONFLICT` column targets without a space before `(` while preserving ANSI identity roundtrips.

### MySQL UNSIGNED Cast Parity

- Map MySQL `CAST(... AS UNSIGNED)` to SQLite `UBIGINT` to match Python SQLGlot.

### MySQL ON DUPLICATE KEY Parity

- Parse and render MySQL `ON DUPLICATE KEY UPDATE` clauses for SQLite-targeted transpilation.

### Postgres DISTINCT ON Parity

- Rewrite simple Postgres `DISTINCT ON` selects to SQLGlot-style SQLite `ROW_NUMBER()` subqueries.
- Cover `DISTINCT ON` rewrites with explicit ordering, expression outputs, and wildcard projections.

### MySQL INSERT IGNORE Parity

- Parse and render MySQL `INSERT IGNORE` for SQLite-targeted transpilation.

### SQLGlot Fixture Importer Ratchet

- Add `--only-matching` importer mode to seed upstream SQLGlot parity files without introducing known failures.

### SQLite GLOB Parity

- Rewrite SQLite-targeted `GLOB(pattern, value)` calls to SQLGlot's infix `value GLOB pattern` output.
- Map `LONGVARCHAR` to SQLite `TEXT` for SQLGlot DDL parity.
- Cover SQLGlot's MySQL text/blob SQLite affinity mappings for `TINYTEXT`, `MEDIUMTEXT`, `LONGTEXT`, `MEDIUMBLOB`, and `LONGBLOB`.

### SQLGlot Import Reports

- Add importer report output so non-matching SQLGlot fixture candidates become an explicit ratchet backlog.
- Support uncapped SQLGlot fixture imports and Markdown report summaries for working through the full backlog.
- Widen SQLGlot fixture extraction across dialect tests, dialect identity cases, simple variables, f-strings, and loop-expanded cases so MySQL/Postgres-to-SQLite reports cover hundreds of candidates.

### Protocol Shim Transpiler Holes

- Match Python SQLGlot for MySQL-to-SQLite integer `DIV`, `DATEDIFF`, and raw `REPLACE INTO` command formatting.
- Match Python SQLGlot for Postgres-to-SQLite typed date/time/timestamp literals and `LIMIT ALL`.
- Match Python SQLGlot for standalone MySQL `GROUP_CONCAT(...)` expressions, including ignored `ORDER BY`, separators, distinct arguments, and multi-expression concatenation.
