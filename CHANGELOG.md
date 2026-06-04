# Changelog

Quick summaries of completed sqlgrok work. The roadmap says what should happen next;
this file records what landed.

## 2026-06-02

### Public ABI And Binding Benchmarks

- Made `sqlgrok_*` the canonical public C ABI while keeping legacy
  `sqlglot_*` compatibility aliases.
- Added `sqlgrok_transpile_into` as an experimental caller-owned-buffer C ABI
  path for measuring output allocation overhead.
- Renamed generated FFI header/package references to `sqlgrok.h` and updated
  C/C++ examples, package metadata, and FFI workflows.
- Updated the prototype Node/Koffi, Ruby/Fiddle, and Go/cgo benchmark bindings
  to use the public `sqlgrok_*` ABI and emit multi-sample median/p95 reports.
- Published current single-call binding performance numbers for PyO3,
  Node/Koffi, Ruby/Fiddle, and Go/cgo against the same MySQL/Postgres/SQLite
  priority workloads.
- Added per-case benchmark reporting, a direct C ABI benchmark, and Criterion
  phase benches for tokenize/parse/transform/generate/full-transpile slowdown
  investigation.
- Added the slowest Postgres window/null-order row to the phase benchmark set
  and documented the `--per-case --mode core` caveat: it is good for finding
  slow Rust rows, but not a fair headline speedup because Python runs through a
  subprocess for each one-row case.
- Added benchmark report caveats to Markdown and JSON output so low-sample runs
  and core per-case reports are labeled as diagnostic evidence.
- Reduced tokenizer allocation on the hot parse path by pre-sizing the token
  output buffer and avoiding heap allocation for ordinary ASCII keyword
  classification.
- Added the dedicated `sqlgrok_alloc_profile` helper binary for Rust-core
  allocation reports and `bench-sqlglot --profile publishable` for stronger
  timing defaults before publishing benchmark claims.
- Extended `sqlgrok_alloc_profile` with `--phase` support so allocation reports
  can isolate tokenize, parse, transform, generate, and full transpile costs.
- Reduced dialect-transform allocation by moving expressions into
  `transform_expr` instead of cloning them first; refreshed allocation reports
  now show MySQL/Postgres/SQLite priority lanes at `6.98`/`6.50`/`6.42` KiB per
  transpile operation.
- Reduced parser allocation counts by avoiding uppercase `String` creation for
  hot context-keyword checks; full-transpile reports now show
  MySQL/Postgres/SQLite priority lanes at `92.38`/`88.00`/`70.50`
  allocations per operation.
- Cleared the local format/strict-Clippy blockers and preserved raw
  SQLite-targeted top-level `PIVOT` / bare Postgres `ALTER TABLE ... SET`
  statements so the focused transpile suite is green again.
- Removed whole-input copies from the tokenizer and parser by making both
  borrow the source SQL during parsing; refreshed full-transpile allocation
  reports now show MySQL/Postgres/SQLite priority lanes at
  `6.32`/`5.77`/`5.75` KiB per operation.
- Added byte end spans to tokens so parser raw carriers can reconstruct source
  text without relying on decoded token values. A hostile review kept public
  punctuation token values intact; spans are now correctness infrastructure for
  raw SQL and future borrowed-token work rather than a public-token API break.
- Added scoped allocation breakdowns to `sqlgrok_alloc_profile` and moved the
  hot transpile path to an owned dialect transform, avoiding a whole-AST clone
  after parse. Refreshed reports show MySQL/Postgres/SQLite priority lanes at
  `5.56`/`5.01`/`4.83` KiB per operation and expose parse as the dominant
  remaining allocation scope.
- Documented the borrowed/internal AST execution plan and added the first
  private parser `SqlText` abstraction for borrowed-or-owned SQL text, wiring it
  into span-based raw token reconstruction without changing public AST shapes.

### Public Documentation Cleanup

- Kept the public documentation surface intentionally small: README, roadmap,
  changelog, parity docs, and performance docs.
- Merged parser architecture, AST inventory direction, and binding plans into
  the roadmap instead of maintaining separate stale-prone docs.
- Reframed the old quick parity-check language as a curated regression corpus and made
  the SQLGlot pytest bridge the explicit parity goal.
- Renamed internal SQLite execution fixtures and reports to generic
  SQLite-compatibility terminology.
- Kept package metadata on MIT-only licensing and version `0.1.0`.

## 2026-05-22

### Parser Coverage Ratchet

- Added a batch transpilation API in Rust and PyO3 (`transpile_many`) plus benchmark support for parity-clean JSONL workloads, direct Rust versus PyO3 single-call/batch binding modes, Markdown/JSON reports, and seed MySQL-to-SQLite, Postgres-to-SQLite, and SQLite identity case files.
- Added prototype Node/Koffi, Ruby/Fiddle, and Go/cgo FFI benchmark bindings against the existing C ABI to measure cross-language single-call overhead.
- Made `xtask bench-sqlglot` fairer by adding multi-sample runs, alternating Python/candidate order, median/min/mean/p95/max reporting, per-sample Markdown/JSON output, and median-based speedups.
- Published the current PyO3 single-call performance snapshot in the README and performance docs: roughly `33x`-`38x` median speedups over Python SQLGlot on the checked-in MySQL/Postgres/SQLite-to-SQLite workloads.
- Added a separate SQLite compatibility lane with an `xtask check-sqlite-correctness` command and seed cases that run Python SQLGlot's SQLite-targeted output against stock SQLite.
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
- Cleared the remaining Postgres imported `time` rust-errors with raw carriers for `MAKE_INTERVAL`/`XMLELEMENT`, Postgres parameter rewriting, and `ON CONFLICT` target predicates/constraints.
- Started the Postgres join rust-error bucket with SQLite parity carriers for `XMLTABLE`, `ROWS FROM`, and multi-argument/ordinality `UNNEST` table sources.
- Reduced the imported Postgres-to-SQLite rust-error backlog from `57` to `28` by adding parity for wrapped `ARRAY(SELECT ...)`, `VARIADIC ARRAY` arguments, additional Postgres `TRIM`/`SUBSTRING` grammar forms, quoted collation preservation, and unary square/cube-root operators.
- Cleared the remaining imported Postgres-to-SQLite rust-error backlog from `28` to `0` with carriers for `WITHIN GROUP`, `LIKE`/`ILIKE ALL`, collated casts, Postgres range/distance operators, transaction `END` aliases, `OVERLAY`, recursive CTE `SEARCH`/`CYCLE`, schema-qualified cast types, `COPY` subqueries, negative JSON indexes, array containment, `MERGE ... DO NOTHING`, window `EXCLUDE`, and parenthesized `VALUES` joins.
- Matched SQLite compatibility Postgres-to-SQLite parity for `NATURAL JOIN`, quoted DDL column identifiers, generated identity primary keys, column-default `now()`, multi-CTE queries, and SQLGlot-shaped index DDL while documenting `CONCURRENTLY`/`USING` as SQLGlot-preserved engine/upstream candidates.
- Superseded the partial SQLGlot fixture-importer plan with a pytest-driven SQLGlot suite bridge plan and added the first `maturin`/`pyo3` Python shim exposing `sqlgrok.transpile(...)`.
- Added the first SQLGlot pytest bridge: helper patching for `validate`, `validate_all`, and `validate_identity`, classified JSONL reports, an `xtask run-sqlglot-suite` wrapper, and a Postgres-to-SQLite budgeted module run.
- Widened the SQLGlot pytest bridge to full transpile-family runs for Postgres-to-SQLite, MySQL-to-SQLite, and SQLite identity, with Markdown summaries, checked-in budgets, and `uv` as the default Python runner.
- Hardened the SQLGlot pytest bridge so unrelated upstream pytest failures do not fail a run after a parity report has been written; `--strict-pytest` keeps the old fail-on-pytest behavior available.
- Burned down the first full-suite bridge mismatch slice: MySQL-to-SQLite and Postgres-to-SQLite now have `0` current SQLGlot-suite bridge mismatches, and SQLite identity improved to `57` matches / `34` mismatches.
- Cleared the SQLite-to-SQLite SQLGlot-suite bridge output backlog: `102` discovered cases now match, with `0` mismatches and `0` Rust errors; the remaining `5` rows are harness-shape limitations.
- Taught the SQLGlot suite bridge to evaluate pretty-output and command-warning helper cases, clearing SQLite-to-SQLite unsupported harness shapes so all `107` discovered cases now match.
- Hardened the bridge/helper review fixes by keeping `identify=True` explicitly unsupported until sqlgrok has native identify-mode generation and limiting raw-SQL pretty normalization to the SQLGlot-covered `CREATE TABLE` fallback.
- Widened `TestTranspile.validate` bridge support for `pretty`, `identity`, and unsupported-level helper kwargs, and added bridge coverage accounting for observed helper attempts versus read/write-filtered routes.
- Added SQLGlot suite bridge forced-pair mode, which replays pytest-discovered transpile SQL through Python SQLGlot's oracle for a requested read/write pair; checked in forced reports for MySQL-to-SQLite, Postgres-to-SQLite, and SQLite identity covering all `15,164` observed helper attempts per lane.
- Added an `xtask bucket-suite-report` summarizer for SQLGlot suite reports and checked in bucket maps for the forced MySQL-to-SQLite, Postgres-to-SQLite, and SQLite identity lanes.
- Burned down the first forced-suite function mismatch cluster by matching SQLite-targeted `LOCATE`, `STR_POSITION`, `NVL2`, and `DECODE` rewrites across MySQL, Postgres, and SQLite source modes, including positioned string search, two-argument `NVL2`, and null-safe `DECODE` expression comparisons.
- Preserved quoted reserved-word implicit select aliases such as `SELECT x "union"` so SQLite-targeted output matches SQLGlot's `AS "union"` rendering.
- Matched SQLGlot's SQLite-targeted `CREATE VIEW` handling for `SQL SECURITY` / `SECURITY` properties by parsing and dropping those unsupported view options.
- Matched SQLGlot's typed-literal SQLite casts for `INT 1` and predicate-side `TEXT 'x'` forms, improving each forced MySQL/Postgres/SQLite-to-SQLite lane by two matches without increasing rust-errors.
- Accepted explicit reserved-word aliases after `AS` such as `SELECT x AS union`, reducing forced-suite rust-errors by four in each tracked SQLite-targeted lane.
- Added SQLGlot-shaped `SAFE_CAST(... AS ...)` parsing for SQLite-targeted output, including BigQuery-style `FORMAT` clauses and MySQL `TIMESTAMP` casts to `TIMESTAMPTZ`; the forced MySQL-to-SQLite lane now drops to `3,892` mismatches and `1,667` rust-errors.
- Parsed SQLGlot's `ARRAY(...)` expression-list form separately from `ARRAY(SELECT ...)`, reducing rust-errors by `74` in each forced MySQL/Postgres/SQLite-to-SQLite lane and lifting exact matches by `69` per lane.
- Added `=>` named-argument expression support and SQLite-targeted `PARSE_JSON(...)` unwrapping, increasing forced-suite exact matches by `75+` per tracked lane while reducing rust-errors by roughly `50` per lane.
- Accepted `RANGE(...)` table functions and top-level `REPLACE(...)` expressions, reducing another `58` forced-suite rust-errors per tracked lane and surfacing the remaining differences as explicit mismatches.
- Accepted SQLGlot forced-suite parser forms for trailing select-list commas, shorthand `VALUES 1, 2` rows, `GROUP BY ALL`, Postgres `@>` array containment, and `IGNORE`/`RESPECT NULLS` carriers; refreshed forced reports now show MySQL/Postgres/SQLite rust-errors down to `1421`/`1443`/`1399`.
- Added a first DuckDB-style `FROM`-first select parser slice for `FROM tbl`, `FROM tbl SELECT x`, comma/join sources, and nested `(FROM ...)` subqueries, reducing forced-suite rust-errors to `1404`/`1426`/`1382` for MySQL/Postgres/SQLite.
- Added a first `|>` pipeline parser slice for `FROM`-first `WHERE`, `ORDER BY`, `LIMIT/OFFSET`, `DISTINCT`, `SELECT`, `AS`, and join stages, reducing forced-suite rust-errors to `1352`/`1374`/`1330` for MySQL/Postgres/SQLite.
- Matched SQLGlot SQLite-targeted rendering for `TIMESTAMP [precision] WITH/WITHOUT TIME ZONE` typed literals and `AT TIME ZONE`, lifting forced-suite exact matches to `8031`/`8700`/`8333` for MySQL/Postgres/SQLite.
- Parsed `CHARACTER VARYING` and `CHARACTER VARYING(n)` casts as SQLite-targeted `TEXT`/`TEXT(n)`, reducing forced-suite rust-errors by `32` in each tracked MySQL/Postgres/SQLite lane.
- Matched MySQL-to-SQLite parser parity for table index hints plus MySQL `UPDATE`/`DELETE` `ORDER BY`/`LIMIT` tails, reducing forced-suite rust-errors to `1305`/`1334`/`1290` for MySQL/Postgres/SQLite without any forced-row regressions.
- Ported a local SQLGlot SQLite correctness slice for ORM-heavy Postgres functions: `ASCII`/`LEFT`/`RIGHT`/`BTRIM`/`STARTS_WITH`, JSON builders, JSON array length/type inspection, and Postgres `#>`/`#>>` JSON path extraction now emit SQLite builtins instead of unavailable Postgres helper names.
- Matched SQLGlot's preserved Postgres `TRUNCATE` options and multi-target rendering, reducing forced-suite rust-errors to `1298`/`1327`/`1283` for MySQL/Postgres/SQLite with no row-level regressions.
- Added a Postgres `COMMENT ON ...` raw carrier with dollar-quoted body normalization, reducing forced-suite rust-errors to `1295`/`1323`/`1280` for MySQL/Postgres/SQLite with no row-level regressions.
- Matched SQLite-targeted `TABLESAMPLE` dropping for table sources, reducing forced-suite rust-errors to `1284`/`1312`/`1269` for MySQL/Postgres/SQLite with no row-level regressions.
- Matched SQLGlot's `BEGIN` normalization plus Postgres transaction-option dropping and MySQL partition/locking table-tail carriers, reducing forced-suite rust-errors to `1264`/`1292`/`1249` for MySQL/Postgres/SQLite with no row-level regressions.
- Added a broader forced-suite parser-carrier batch for MySQL `MATCH ... AGAINST`, `STRAIGHT_JOIN`, dotted `@@GLOBAL` variables, aliased `VALUES` upserts, and Postgres aggregate `FILTER(...) OVER (...)` suffixes; refreshed forced reports now show MySQL/Postgres/SQLite rust-errors down to `1251`/`1280`/`1243` with no row-level regressions.
- Updated the roadmap to mark the Postgres `E'...'` SQLite quoting issue as fixed upstream by SQLGlot [#7677](https://github.com/tobymao/sqlglot/pull/7677), replacing the old open-PR follow-up note.
- Resynced the local Python SQLGlot oracle to upstream `main` at `d6a53b43` and refreshed forced-suite reports against `15,170` helper attempts per lane; MySQL/Postgres/SQLite rust-errors are now `1177`/`1206`/`1168`.
- Added exact SQLite-targeted parser parity for qualified function names such as `SAFE.*`, `NET.*`, and `assert.true(...)`, plus standalone cast aliases, `interval::int`, and SQLGlot-style `IF ... THEN ... END` expressions.
- Reconciled oracle drift from SQLGlot [#7677](https://github.com/tobymao/sqlglot/pull/7677) by matching SQLite-targeted Postgres `E'...'` byte-string fallback output, and updated MySQL `AUTO_INCREMENT` primary-key ordering for the current SQLGlot oracle.
- Matched SQLGlot's `BETWEEN SYMMETRIC`/`ASYMMETRIC` rewrites and prefix `NOT BETWEEN` rendering; refreshed forced-suite reports now show MySQL/Postgres/SQLite rust-errors at `1155`/`1184`/`1146`.
- Burned down another forced-suite parser batch by matching SQLite-targeted `CAST(... FORMAT ...)`, recursive `MAP`/`STRUCT` type casts, quantified `LIKE`/`ILIKE` lists, predicate-side boolean casts, `STRUCT(... AS ...)` arguments, and `FLOOR`/`CEIL(... TO ...)` carriers; refreshed forced-suite reports now show MySQL/Postgres/SQLite exact matches at `8408`/`9076`/`8683` and rust-errors at `994`/`1022`/`999`.
- Reduced the next forced-suite parser buckets with raw carriers for function-local `ORDER BY`/`HAVING`/`LIMIT` tails, SQLite null-treatment dropping in those raw function arguments, `ANY_VALUE` to `MAX`, `LATERAL VIEW` table tails, Snowflake-style `AT`/`CHANGES` table tails, and `DIRECTED JOIN` dropping while preserving explicit `OUTER`; refreshed forced-suite reports now show MySQL/Postgres/SQLite exact matches at `8548`/`9194`/`8803` and rust-errors at `926`/`950`/`927`.
- Matched SQLGlot forced-suite `BY` parser forms for ClickHouse-style `LIMIT ... BY ...` and create-table `PARTITIONED BY`/`DISTRIBUTED BY`/`LOCATION`/`TBLPROPERTIES` tails; refreshed forced reports now show MySQL/Postgres/SQLite rust-errors down to `797`/`827`/`796`.
- Reduced the forced-suite `Expected identifier` parser bucket with repeated/shorthand CTEs, string-literal table names, parenthesized `TABLESAMPLE`, create-table `EMPTY`/`WITH (...)`/`TTL` tails, CTAS column rendering, alias-column keywords, and `ALL` function-argument normalization; refreshed forced reports now show MySQL/Postgres/SQLite rust-errors down to `745`/`774`/`756`.
- Added SQLGlot-shaped parsing for `OFFSET ... ROWS FETCH ...` and SQL Server-style `CROSS`/`OUTER APPLY` lateral joins when targeting SQLite; refreshed forced reports now show MySQL/Postgres/SQLite rust-errors down to `719`/`748`/`730`.
- Burned down another forced-suite parser batch covering cast-format `AT TIME ZONE` tails, BigQuery JSON typed literals, struct literals, `SELECT AS STRUCT`, bare `PARTITION BY`/`CLUSTER BY` create-table options, parenthesis-less `JOIN ... USING`, pipeline `AGGREGATE`, `xor(...)`, and bare `JOIN` preservation; refreshed forced reports now show MySQL/Postgres/SQLite rust-errors down to `612`/`652`/`633`.
- Extended SQLite-targeted function rewrites to cross all forced source dialects: `LEVENSHTEIN(a, b)` to `EDITDIST3(a, b)`, `MEDIAN(x)` to `PERCENTILE_CONT(x, 0.5)`, `MOD(a, b)` to `a % b`, `COUNT_IF(cond)` to `SUM(IIF(cond, 1, 0))`, `STRPOS(x, y)` to `INSTR(x, y)` (was Postgres-only), `TIME_STR_TO_TIME(x [, tz])` to `x` (was MySQL-only), and `TIME_TO_TIME_STR(x)` to `CAST(x AS TEXT)`; refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `8828`/`9488`/`9128` (up from `8767`/`9422`/`9039`) and mismatches at `3841`/`3424`/`3714` (down from `3911`/`3500`/`3813`).
- Matched additional SQLite-targeted function rewrites and rendering: `YEAR(x)`/`MONTH(x)`/`DAY(x)` preserve their names (wrapping the inner expr in `DATE(...)` only for MySQL-family sources) instead of always rewriting to `EXTRACT(... FROM ...)`, `CHARINDEX(x, y)` to `INSTR(y, x)`, `MAX_BY`/`MIN_BY` to `ARG_MAX`/`ARG_MIN`, `POSITION(x IN y)` extended to all sources, `MEDIAN(x) OVER (...)` now flows through the rewrite while preserving the window spec, `LTRIM`/`RTRIM` keep their second-arg trim chars instead of dropping them, and the unconditional `ASCII` to `UNICODE` rewrite was removed because Python SQLGlot preserves `ASCII` for SQLite. Function names in generic `Expr::Function` calls are now uppercased for SQLite target to match SQLGlot's output. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9152`/`9835`/`9476` and mismatches at `3517`/`3077`/`3366`.
- Added another SQLite-targeted function-rewrite batch matching SQLGlot output: `TO_NUMBER(x [, fmt])` to `CAST(x AS REAL)`, `SAFE_DIVIDE(x, y)` to `IIF(y <> 0, CAST(x AS REAL) / y, NULL)`, `BOOLAND_AGG` to `MIN`, `BOOLOR_AGG` to `MAX`, `BOOLAND(x, y)` to `((x) AND (y))`, `BOOLOR(x, y)` to `((x) OR (y))`, `DATEFROMPARTS` to `DATE_FROM_PARTS`, and `NVL` to `COALESCE` across all non-Oracle targets (was previously `IFNULL`/`ISNULL` for some). The `STR_TO_TIME` SQLite generator now emits `STR_TO_TIME` instead of `TO_TIMESTAMP` for non-MySQL/non-BigQuery targets. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9240`/`9923`/`9547` and mismatches at `3429`/`2989`/`3295`.
- Matched another SQLite-targeted rendering batch: `INTERVAL <n> <unit>` quotes the numeric value to match SQLGlot (`INTERVAL '1' DAY`); `SHA256(x)`/`SHA512(x)` are only canonicalized to `SHA2(x, 256|512)` for Postgres-family sources and preserved for MySQL/SQLite sources, and the parser no longer mis-maps `SHA256`/`SHA512` to a default 256-bit Sha2 typed function; SQLite-targeted `CAST` now folds `TINYBLOB`/`MEDIUMBLOB`/`LONGBLOB` to `BLOB`, `INT8`/`INT16`/`INT32`/`INT64`/`INT128`/`INT256` to `INTEGER`, and `BIGNUMERIC` to `BIGDECIMAL`. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9315`/`10016`/`9662` and mismatches at `3354`/`2896`/`3180`.
- Another SQLite-targeted function-rewrite batch: `REGEXP_EXTRACT` now stays `REGEXP_EXTRACT` (was being renamed to `REGEXP_SUBSTR`), the parser preserves overflow arguments on `REGEXP_EXTRACT`/`REGEXP_REPLACE` by falling back to generic function calls when there are more args than the typed shape allows, `COUNT_IF(cond) FILTER(WHERE ...)` and `COUNT_IF(cond) OVER (...)` now flow through the `SUM(IIF(...))` rewrite while preserving the suffix, `GENERATE_UUID()` to `UUID()`, `LAST_DAY_OF_MONTH(x)` to `LAST_DAY(x)`, `CURRENT_VERSION()` to `SQLITE_VERSION()`, `ARRAY_LENGTH(arr)` keeps `ARRAY_LENGTH` instead of converting to `ARRAY_SIZE`, and `TS_OR_DS_TO_DATE(x)` renders as `DATE(x)` for SQLite target. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9466`/`10173`/`9818` and mismatches at `3203`/`2739`/`3024`.
- Yet another SQLite-targeted function-rewrite batch: `TO_CHAR(x)` to `CAST(x AS TEXT)`, `TRUNCATE(x, n)` to `TRUNC(x)`, `APPROX_COUNT_DISTINCT` generator emits `APPROX_DISTINCT` for SQLite, and `MOD(left, right)` now wraps either operand in parentheses when it is itself a binary op so we match SQLGlot's `(8 - 1 + 7) % 7` precedence. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9522`/`10216`/`9874` and mismatches at `3147`/`2696`/`2968`.
- Source-aware `TO_TIMESTAMP` handling and `SHOW` dropping: parser no longer collapses `TO_TIMESTAMP` into the generic `StrToTime` typed function so SQLite/MySQL sources preserve `TO_TIMESTAMP(...)` instead of leaking through as `STR_TO_TIME(...)`; Postgres-family `TO_TIMESTAMP(x, fmt)` continues to lower into `STR_TO_TIME` with format conversion via an explicit dialect transform. SQLite-targeted raw `SHOW` statements coming from MySQL-family sources collapse to an empty statement to match SQLGlot's `Command` fallback, while Postgres- and SQLite-source `SHOW` raw passthroughs are preserved. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9573`/`10218`/`9924` and mismatches at `3096`/`2694`/`2918`.
- Burned down another CREATE TABLE / function batch: parser drops `VOLATILE` / `TRANSIENT` / `EXTERNAL` / `GLOBAL` / `LOCAL` / `SET` / `MULTISET` / `ICEBERG` / `DYNAMIC` table modifiers before `TABLE`, accepts column-less `CREATE TABLE x` heads (e.g. `CREATE TABLE x USING ICEBERG`), parses `STORED AS ...` and `USING ...` as create-table options, and falls back to raw when an unrecognized tail follows the head so we don't drop dialect-specific options. `VARCHAR2` / `NVARCHAR2` parse as `Varchar(len)` so SQLite renders them as `TEXT(len)`. SQLite-targeted `gen_create_table_options` preserves the `USING <engine>` option while dropping every other dialect-specific option to match SQLGlot. `GETDATE()` parses as a plain function instead of `CurrentTimestamp` and is preserved across MySQL/Postgres/SQLite sources targeting SQLite (only T-SQL sources still rewrite it to `CURRENT_TIMESTAMP`). Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9691`/`10333`/`10041` and mismatches at `3012`/`2621`/`2843`, with rust-errors down to `577`/`609`/`590`.
- More SQLite function/rendering parity: `TRY_CAST(... AS ...)` now parses as `Expr::TryCast` and lowers to `CAST(... AS ...)` for SQLite (matches SQLGlot's TRY_CAST dropping). `DAYOFMONTH` / `DAYOFYEAR` / `DAYOFWEEK` / `WEEKOFYEAR` rewrite to `DAY_OF_MONTH` / `DAY_OF_YEAR` / `DAY_OF_WEEK` / `WEEK_OF_YEAR` (wrapping the inner expression in `DATE(...)` for MySQL-family sources), and `WEEK(x)` wraps in `DATE(x)` for MySQL source. `POW` typed function renders as `POWER` for SQLite. `SUBSTR` and `SUBSTRING` both render as `SUBSTRING` for SQLite. `JSON_VALUE` and `TO_JSON` / `TO_JSON_STRING` are preserved as plain function calls instead of being folded into `JSONExtract` / `JSONFormat` typed functions. SQLite-targeted `ALTER COLUMN c TYPE x` expands to `ALTER COLUMN c SET DATA TYPE x`. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9787`/`10408`/`10141` and mismatches at `2917`/`2547`/`2744`.
- `CREATE TABLE` and `LIMIT`/`OFFSET` parity batch: `CREATE TABLE ... LIKE source` rewrites to `CREATE TABLE ... AS SELECT * FROM source LIMIT 0` to match SQLGlot's `LikeProperty` lowering. `CREATE TABLE` now tracks `OR REPLACE`. `AUTO_INCREMENT` / `IDENTITY` / inline `PRIMARY KEY` ordering is reconciled with SQLGlot's SQLite output: AUTO_INCREMENT alone is dropped, AUTO_INCREMENT before PRIMARY KEY is dropped (INTEGER PRIMARY KEY is autoincrement implicitly in SQLite), AUTO_INCREMENT after PRIMARY KEY keeps `PRIMARY KEY AUTOINCREMENT`, IDENTITY columns always produce `PRIMARY KEY AUTOINCREMENT`, and a separate `PRIMARY KEY (col)` table constraint consolidates into `AUTOINCREMENT PRIMARY KEY` on the column. `TEXT(n)` parses as `Unknown("TEXT(n)")` so SQLite output preserves the length suffix. SQLite-targeted SELECT emits a sentinel `LIMIT -1` when only `OFFSET` is present (SQLite requires LIMIT alongside OFFSET) and drops `LIMIT ALL` entirely. SQLite-targeted `INSERT IGNORE` renders as `INSERT OR IGNORE`. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9828`/`10447`/`10183` and mismatches at `2876`/`2508`/`2702`.
- TIMESTAMP type and JSON path parity: SQLite-targeted CAST folds `TIMESTAMP_NTZ` / `TIMESTAMP_LTZ` / `TIMESTAMP_TZ` to the underscoreless `TIMESTAMPNTZ` / `TIMESTAMPLTZ` / `TIMESTAMPTZ` to match SQLGlot. `JSON_EXTRACT_PATH_TEXT(x, ...)` now lowers to `x ->> $.path` for every source dialect (was Postgres-only); `JSON_EXTRACT_PATH(x, ...)` continues to lower only for Postgres-family sources. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9854`/`10467`/`10209` and mismatches at `2850`/`2488`/`2676`.
- Parser no longer folds `DATEADD(unit, n, expr)` into `TypedFunction::DateAdd` so SQLite-targeted output preserves the original `DATEADD(unit, n, expr)` form for MySQL/Postgres/SQLite sources (matches SQLGlot, which keeps `DATEADD` unchanged across these sources). `DATE_ADD` continues to flow through `TypedFunction::DateAdd`. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9902`/`10515`/`10257` and mismatches at `2802`/`2440`/`2628`.
- Parser no longer folds `DATE_FORMAT` into `TypedFunction::TimeToStr`; instead, MySQL/Hive-family source `DATE_FORMAT` flows through `TimeToStr` via an explicit dialect transform so cross-target rewrites (STRFTIME / TO_CHAR / FORMAT_TIMESTAMP) still work, while Postgres/SQLite source `DATE_FORMAT` is preserved as a plain function call to match SQLGlot's identity behavior for those sources. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9901`/`10558`/`10300` and mismatches at `2803`/`2397`/`2585`.
- Preserve `ROUND(x, decimals, mode)` (3-argument form, mode argument used by Snowflake / BigQuery) by falling back to a plain function call instead of folding into the 2-arg `Round` typed function. Preserve the original case of unrecognized data type names (`some_udt`, `MyType`, `BINARY_DOUBLE`, etc.) in `parse_data_type` so SQLGlot round-trips the UDT spelling. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9926`/`10613`/`10382` and mismatches at `2778`/`2342`/`2503`.
- `TS_OR_DS_TO_DATE_STR(x)` lowers to `SUBSTRING(CAST(x AS TEXT), 1, 10)` for SQLite. Generator uppercases unquoted SQL pseudo-column identifiers (`current_time`, `current_date`, `current_timestamp`, `current_user`, `current_role`, `current_schema`, `localtime`, `localtimestamp`, `session_user`, `system_user`, `user`) for SQLite output, matching SQLGlot's canonicalization. Quoted identifiers (`"current_time"`) keep their original case. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9939`/`10624`/`10395` and mismatches at `2765`/`2331`/`2490`.
- Statement-level `REPLACE(...)` for non-Postgres sources targeting SQLite is rewritten through a `Raw` statement with a space before the open paren (`REPLACE (...)`) to match SQLGlot's `Command`-fallback rendering. Postgres-source `REPLACE(...)` is left alone since SQLGlot's Postgres parser handles it natively. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `9970`/`10624`/`10426` and mismatches at `2734`/`2331`/`2459`.
- More SQLite function rewrites and identity canonicalization: `UUID_STRING(...)` (any args) lowers to `UUID()`, `ENDSWITH(x, y)` to `ENDS_WITH(x, y)`, and `VAR_POP(x)` to `VARIANCE_POP(x)`. Parser no longer folds `CURRENT_TIMESTAMP` into `TypedFunction::CurrentTimestamp` (only `NOW` does); `CURRENT_TIMESTAMP()` / `CURRENT_TIMESTAMP(n)` lowers to the bare `CURRENT_TIMESTAMP` column form for SQLite, matching SQLGlot's drop-the-parens behavior. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10034`/`10661`/`10490` and mismatches at `2670`/`2294`/`2395`.
- SQLite-targeted Interval generator now splits string interval literals like `INTERVAL '1 DAY'` into `INTERVAL '1' DAY` (value as string, unit as keyword) to match SQLGlot, but only when the unit token is a recognized DateTimeField keyword so Postgres compound intervals like `INTERVAL '1 01:00'` stay intact. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10054`/`10662`/`10511` and mismatches at `2650`/`2293`/`2374`.
- `SPACE(n)` lowers to `REPEAT(' ', n)` for SQLite. `TIME_SLICE(x, n, 'unit' [, 'END'])` and `TS_OR_DS_ADD(x, n, 'unit')` unquote the unit string into a bare keyword (`HOUR`, `DAY`, etc.) when the unit token is a recognized DateTimeField name. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10090`/`10698`/`10547` and mismatches at `2614`/`2257`/`2338`.
- SQLite-targeted `TypedFunction::DateAdd` generator now lowers `DATE_ADD(x, n [, unit])` to `DATE(x, '<n>[ <unit>]')` (with the second arg as a packed string literal) to match SQLGlot's SQLite output. Interval-form inputs (`DATE_ADD(x, INTERVAL 5 DAY)`) lower to `DATE(x, 'INTERVAL '5' DAY')` matching SQLGlot's nested-quoting form. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10090`/`10750`/`10600` and mismatches at `2614`/`2205`/`2285`.
- More SQLite function rewrites: `DATE_STR_TO_DATE(x)` lowers to the bare `x` (matches SQLGlot's identity unwrap), `DATE_TRUNC(col, ...)` uppercases and string-quotes the first arg (so `DATE_TRUNC(date, WEEK(MONDAY))` becomes `DATE_TRUNC('DATE', WEEK(MONDAY))`), and `DATE_FROM_UNIX_DATE(n)` lowers to `DATE(DATE('1970-01-01'), '<n> DAY')`. Parser no longer folds `STR_TO_DATE` into the generic `StrToTime` typed function so SQLite/Postgres-source `STR_TO_DATE(x, fmt)` preserves its original name instead of leaking through as `STR_TO_TIME`. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10115`/`10785`/`10653` and mismatches at `2589`/`2170`/`2232`.
- SQLite-targeted JSON path normalization for `JSON_EXTRACT` / `JSON_EXTRACT_SCALAR` / `JsonAccess` arrow output: bracket-quoted segments (`$["a b"]`) collapse to dotted-quoted form (`$."a b"`), and trailing `[*]` / `.*` wildcard suffixes are dropped to match SQLGlot. Also added a MySQL-source-only `STR_TO_DATE(x, fmt)` lowering that converts the format string and renames to `STR_TO_TIME` when the format contains time markers (Postgres / SQLite sources continue to preserve `STR_TO_DATE`). Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10151`/`10812`/`10680` and mismatches at `2553`/`2143`/`2205`.
- `JSON_EXTRACT_PATH_TEXT(x, k1, k2, ...)` for non-Postgres sources now lowers to `x ->> '$.k1'` using only the first path arg, matching SQLGlot's behavior for those sources (non-Postgres parsers don't natively make sense of multi-arg JSON_EXTRACT_PATH_TEXT). Postgres-family sources still join all path args. First-arg strings that already start with `$` are passed through verbatim. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10168`/`10812`/`10697` and mismatches at `2536`/`2143`/`2188`.
- More SQLite function rewrites: `CHARINDEX(needle, haystack, position)` and `INSTR(haystack, needle, position[, occurrence])` lower to the SUBSTRING-and-offset IIF form (SQLite has no 3-arg INSTR). `STRING_AGG(x [, sep]) WITHIN GROUP (ORDER BY ...)` drops the WITHIN GROUP clause for SQLite output since the lowered `GROUP_CONCAT` doesn't accept it (other WITHIN GROUP aggregates like `PERCENTILE_CONT` keep the clause). `UPPER(TO_HEX(x))` / `UPPER(HEX(x))` lowers to `HEX(x)` (HEX output is already uppercase). Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10204`/`10848`/`10733` and mismatches at `2500`/`2107`/`2152`.
- Parser now accepts trailing `UNSIGNED` / `SIGNED` on `INT` / `INTEGER` / `BIGINT` data type forms (MySQL idiom), producing `Unknown("INT UNSIGNED" | "BIGINT SIGNED" | ...)`. SQLite-targeted CAST folds these to `UINT` / `INTEGER` / `UBIGINT` / `BIGINT` to match SQLGlot. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10208`/`10852`/`10737` and mismatches at `2496`/`2103`/`2148`.
- `CONVERT_TIMEZONE(zone, x)` lowers to `x AT TIME ZONE zone` for SQLite; the 3-arg form `CONVERT_TIMEZONE(src, tgt, x)` lowers to `CAST(x AS TIMESTAMPNTZ) AT TIME ZONE src AT TIME ZONE tgt`. Parser now accepts `ALTER TABLE ... ALTER [COLUMN] name [SET DATA TYPE | TYPE] dtype` (Postgres / DuckDB / Snowflake style) and routes through `AlterTableAction::AlterColumnType`, which renders as `ALTER COLUMN name SET DATA TYPE dtype` for SQLite (matches SQLGlot's full form). Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10228`/`10872`/`10757` and mismatches at `2476`/`2083`/`2128`.
- `DATE_ADD` lowering improvements: SQLite-targeted `DATE_ADD(a, n, c)` where `c` isn't a recognized `DateTimeField` keeps the column / expression in the payload (`DATE(a, '<n> <c>')`) instead of dropping it. `INTERVAL <signed-number> <unit>` and `INTERVAL '<n>' <unit>` interval arguments render with the same nested-quote shape SQLGlot produces (`DATE(x, 'INTERVAL '-1' DAY')`). `Expr::UnaryOp Minus(Number)` now renders inside the DATE payload as a signed literal instead of Rust debug output. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10233`/`10924`/`10809` and mismatches at `2471`/`2031`/`2076`.
- `DATE_TO_DATE_STR` / `TIME_TO_TIME_STR` / `DATE_TO_TIME_STR` all lower to `CAST(x AS TEXT)` for SQLite. Extended `TO_NUMBER` to accept the 3-arg `(expr, fmt, nlsparam)` Oracle form and still lower to `CAST(expr AS REAL)`. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10244`/`10935`/`10820` and mismatches at `2460`/`2020`/`2065`.
- `DECLARE` (BigQuery / T-SQL / Snowflake variable declarations) routes through the raw-statement parser so the body is preserved instead of leaking `DECLARE;` plus trailing tokens. Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10244`/`10975`/`10820` and mismatches at `2460`/`1991`/`2065` (postgres also dropped a parser rust-error along the way).
- SQLite-source `JOIN ... t2` (no `ON` / `USING`) synthesizes `ON TRUE` to match SQLite's grammar requirement and SQLGlot's identity behavior. The synthesis only fires for SQLite sources so MySQL/Postgres identity round-trips that leave the ON clause off still match. SQLite-targeted `CURRENT_USER` (bare column form) renders with parens as `CURRENT_USER()` (other pseudo-columns like `CURRENT_TIMESTAMP` stay bare). Refreshed forced reports now show MySQL/Postgres/SQLite exact matches at `10256`/`10987`/`10878` and mismatches at `2448`/`1979`/`2007`.

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
- Added parser architecture notes, including the Databend parser article as inspiration for parser ergonomics while keeping Python SQLGlot as the behavior contract.

### Parity Harness

- Added JSONL parity metadata: `tags`, `source`, `mode`, `skip_reason`, `accepted_rust`, and `note`.
- Added parity filters via `SQLGROK_PARITY_ID`, `SQLGROK_PARITY_TAG`, `SQLGROK_PARITY_READ`, and `SQLGROK_PARITY_WRITE`.
- Added duplicate id and tag validation plus summary output.
- Changed the harness to load all `parity/cases/*.jsonl` files.

### CI And Tooling

- Added standard CI for format, clippy, tests, and pinned Python SQLGlot curated parity.
- Added `xtask import-sqlglot-fixtures` for deterministic SQLGlot fixture extraction with `--dry-run`, `--limit`, `--read`, and `--write`.

### First Parity Ratchets

- Locked in MySQL `GROUP_CONCAT(... SEPARATOR ...)` to SQLite parity.
- Added `JoinType::Comma` so comma joins preserve SQLGlot string parity while remaining semantic cartesian joins in execution.
- Removed the accepted-divergence marker from the comma join curated case.
- Reached curated parity with `4/4` exact matches and `0` accepted divergences.

### Project Memory

- Moved the roadmap to top-level [ROADMAP.md](ROADMAP.md) so it sits beside README and CHANGELOG.

### AST Inventory

- Added `xtask inventory-ast` to compare Python SQLGlot's `sqlglot/expressions/` package against sqlgrok's Rust AST enums.
- Added an AST inventory report with coverage counts, priority gaps, module summaries, and a full generated inventory.
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
- Reduced forced-suite parser rust-errors for parameterized `STRING`/`JSON`/`FLOAT` casts, table-function aliases with plain or typed column lists, ClickHouse-style `INSERT ... FORMAT Values`, raw insert variants, raw extended `TRUNCATE`, raw top-level `PIVOT`, complex `UNPIVOT`, and non-standard set-operation modifiers.
- Matched Python SQLGlot's SQLite-targeted `UNPIVOT` behavior by dropping parsed `UNPIVOT` table-source wrappers.

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
