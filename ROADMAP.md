# sqlgrok Roadmap

sqlgrok's mission is to become a pure-Rust SQLGlot port that can run Python
SQLGlot's behavior suite directly, with every known divergence tracked and
ratcheted toward parity.

This roadmap is the public execution plan. Completed work belongs in
[CHANGELOG.md](CHANGELOG.md); generated details belong in `parity/reports/` or
`benchmarks/reports/`.

## Current Critical Path

The project is a pure port of SQLGlot's architecture:
`transpile(sql, read, write) = generate(parse(sql, read), write)`. Work is
port-driven, not gap-driven: read SQLGlot's per-dialect parsers and
generators and port that knowledge directly; the forced suite is the
scoreboard that proves each slice, not the to-do list that discovers it.

1. Keep the Rust library, CLI, and curated parity regression corpus green.
2. Backfill the remaining parser-owned canonicalization and generator-owned
   render/lowering gaps directly in the parse/generate architecture.
3. Phase 4 backfill: transcribe SQLGlot's per-target generator dicts
   (TYPE_MAPPING, TRANSFORMS renames) into `rules.rs` tables and typed
   generator arms; the tables are already dict-shaped for this.
4. Keep the deleted legacy transform path from returning: new dialect behavior
   must enter through parsers, AST fields, or generators.
5. Ratchet every slice on the seven forced lanes with row-level diffs (zero
   regressions), not just status counts.
6. Expand beyond transpilation into parse/generate, optimizer, and expression
   AST-shape suites.
7. Stabilize the Rust API, C ABI, and first language bindings around the same
   conformance and benchmark cases.
8. Prepare a clean `0.1.0` crate release with small docs, MIT licensing, clear
   attribution, and reproducible parity/performance commands.

## Architecture Port (parse -> generate)

Status: active; the legacy transform path has been retired. Full plan:
[docs/PORTING_PLAN.md](docs/PORTING_PLAN.md).

The core architectural effort is converging sqlgrok onto SQLGlot's actual model:

```
transpile(sql, read, write) = generate(parse(sql, read), write)
```

There is no `transform(source, target)` middle layer in the target design. All
dialect knowledge lives in two homes:

- Parser (read side): resolves source-dialect ambiguity into a canonical,
  dialect-neutral AST. Anything keyed on `source` belongs here.
- Generator (write side): TYPE_MAPPING, function renames, node->SQL lowerings,
  keyword filters, quoting. Everything keyed on `target` only.

Why: the legacy `transform_*` layer encodes the dialect matrix as O(N^2)
`if source == X && target == Y` rules; SQLGlot is O(N) (one parser + one
generator per dialect). This is why `* -> sqlite` is ~91% suite match while
`* -> postgres` is much lower; the generators are sqlite-shaped.

Phases (ratcheted on the forced suite, no real regressions, commit per slice):

1. Relocate target-only rules into the generator as `src/dialects/rules.rs`
   data tables (`Option<&'static str>`, zero-alloc, shared with the perf
   fast path). Started: function renames and type mappings.
2. Move the source-branching rules from `transform_expr`/`transform_statement`
   into the parser as canonical-AST normalizations (each a silent-wrong risk).
   Landed example: MySQL `||` -> logical OR at tokenize time.
3. Delete the transform layer and the `(source, target)` signature. Landed.
4. Port SQLGlot's per-target dicts to backfill thin non-sqlite generators.

### Phase 3 retirement runway: draining the legacy path

Status: landed. `transform_owned(from, to)` and the builtin
`transform(source, target)` signatures have been deleted; public transpile
paths now run `generate(parse(sql, read), write)` directly.

The remaining work is no longer a broad expression-function migration. The
function/date source gates have moved into typed parser/generator architecture.
What remains is parser/generator completeness: raw carriers that should become
structured AST, statement-level command modeling, and generator coverage for
targets that are still thinner than SQLite.

Retirement sequence:

1. **Raw aggregate ORDER BY carriers.**
   - Problem: Postgres aggregate calls such as `ARRAY_AGG(x ORDER BY y)` can
     still arrive as raw string args. The transform layer injects SQLite
     `NULLS FIRST/LAST` semantics into that raw string.
   - Fix: add typed aggregate argument ordering (`ORDER BY`, `LIMIT`, and
     null-direction metadata) to function/aggregate AST nodes; parse it on the
     read side; render it on the write side.
   - Progress: `RawOrderNullsPolicy` now carries Postgres raw aggregate
     null-ordering semantics from the parser to the SQLite generator, deleting
     `propagate_nulls_direction` and the final source-gated branch in
     `transform_expr`. `Expr::Function` also has structured function-local
     `ORDER BY` and `LIMIT` fields, and the parser/generator round-trips those
     tails for the common aggregate/function family without raw argument text.
   - Remaining: model the harder nonstandard aggregate tails that still need
     raw fallback: `HAVING`, `IGNORE`/`RESPECT NULLS`, and comma-style
     `LIMIT`.

2. **Raw table-source carriers.**
   - Problem: `TableSource::Raw` still carries behavior, not only passthrough:
     `UNNEST(...)` array literal rewrites, BigQuery `WITH OFFSET`, Postgres
     VALUES alias cleanup, raw typed literal rewrites, and raw function
     uppercasing.
   - Fix: model the missing table-source structure directly: richer
     `TableFunction`/`Unnest` fields for ordinality, offset aliases, alias
     column lists, table-function tails, and `ROWS FROM`; typed carriers for
     JSON/XML table sources where needed.
   - Progress: `RawTableSourceNormalization` now carries the current raw
     table-source rewrite policy from the parser to the SQLite generator,
     including Postgres VALUES alias cleanup, BigQuery `WITH OFFSET`, UNNEST
     array literal policy, backtick quoting, function uppercasing, and typed
     literal normalization. The table-source transform no longer backfills
     `source_dialect`, and SQLite raw table-source rendering no longer branches
     on source dialect. Single-expression BigQuery/Postgres `UNNEST(...)`
     now uses typed `TableSource::Unnest` fields for alias column lists,
     `WITH OFFSET`, offset aliases, generated BigQuery offset aliases, and
     Postgres `WITH ORDINALITY`; generator-owned rendering covers SQLite,
     Postgres, DuckDB, and BigQuery shapes while unsupported bracket-array
     forced-read cases stay on the raw passthrough path. `ROWS FROM (...)` now
     has a typed `TableSource::RowsFrom` carrier with function aliases,
     table-level aliases, typed alias-column lists, and `WITH ORDINALITY`.
   - Exit: replace the remaining raw text with structured table-source fields
     where practical. Remaining table-source raw carriers are multi-argument
     `UNNEST`, JSON/XML table sources, table-function tails, and dialect
     fallback cases where the tokenizer still treats source syntax as quoted
     identifiers rather than parseable expressions.

3. **Raw statement normalization.**
   - Problem: `RawStatement` still carries rewrite behavior for Postgres enum
     and recursive CTE raw text, `COPY`, MySQL `SHOW`, raw `PIVOT`/`UNPIVOT`,
     and insert-into-function cleanup.
   - Fix: add typed or semi-typed statement variants (`CommandKind` where full
     AST modeling is not worth it yet) so parsers classify the source behavior
     and generators render or drop by target.
   - Progress: `RawStatementNormalization` now carries the existing raw
     statement rewrite policy from the parser to the SQLite generator. The
     generator no longer branches on `source_dialect` for raw statements, and
     `transform_statement` no longer backfills raw statement source dialect.
     A first `Statement::Command` carrier now owns command-shaped cases for
     `COPY`, MySQL `SHOW`, standalone `PIVOT` / `UNPIVOT`, statement-level
     `REPLACE(...)`, SQLite database commands, and Postgres
     `CREATE TYPE ... AS ENUM`, leaving `RawStatementNormalization` slimmer.
   - Exit: replace the remaining raw statement text with typed or semi-typed
     statement variants where practical, leaving raw statements as inert
     unsupported passthrough only. Remaining raw-statement behavior is mostly
     Postgres recursive CTE cleanup and insert-into-function cleanup.

4. **Target-only generator lowerings.**
   - Problem: target dialects beyond SQLite still need broader generator
     coverage, and new target behavior must not revive a middle transform pass.
   - Fix: keep target-only rendering decisions in the generator, or in
     explicitly named parser canonicalizations where SQLGlot parses a different
     IR shape.
   - Progress: generator-owned rendering now handles `ILIKE` fallback,
     SQLite `GROUP_CONCAT`/`STRING_AGG` `WITHIN GROUP` omission, SQLite lock
     omission, quoted identifier spelling, SQLite identity join cleanup,
     SQLite `DISTINCT ON` lowering, SEMI/ANTI join lowering for targets that
     cannot render them, and limit/top/fetch normalization across T-SQL,
     SQLite, and ordinary `LIMIT` targets. Parser-owned flags preserve identity
     behavior where a target dialect's identity lane keeps source `LIMIT`
     spelling. The transform no longer mutates those AST shapes.
   - Exit: target-only behavior is covered by generator tests and forced lanes,
     with no builtin transform API available.

5. **AST gap closure.**
   - Problem: the remaining hard cases are places where the AST cannot yet
     express SQLGlot's IR: raw table-source structure, aggregate arg ordering,
     source-specific join/apply quirks, and parser fallback cases.
   - Fix: add the missing AST fields first, then relocate behavior. Do not
     move raw-text rewrites into another module and call that architecture.
   - Exit: full matrix has no pair-keyed behavior; correctness factorizes into
     parser lanes plus generator lanes.

Final deletion status:

- The no-op compatibility shim was proven on the seven forced lanes and then
  removed.
- `src/lib.rs`, CLI, Python binding path, benches, and allocation profiling now
  use parse/generate directly for built-in dialects.
- Remaining work is parser/generator completeness and AST expressiveness, not
  transform-arm drainage.

### Measurement model: pair lanes retire with Phase 3

The dialect x dialect lane matrix is the right scoreboard only while
behavior can vary per pair. The old transform layer made pairs the unit of
failure (`if source==X && target==Y` rules), so we track read/write pairs.
Once nothing is pair-keyed, correctness factorizes: a read bug appears in
every lane with that read dialect, a write bug in every lane with that
write target, and N^2 lanes just re-measure the same N parser and N
generator bugs with quadratic suite runtime.

Plan, after Phase 3 deletion:

- **Short term: keep the current seven pair lanes as the ratchet.**
  They provide continuity across the retirement and protect against hidden
  report swaps while parser/generator backfill continues.
- **Next scoreboard model:**
  1. *Identity per dialect* (`d -> d`), one number per dialect — SQLGlot's
     own primary check (`validate_identity`), exercising one parser plus one
     generator with no cross-dialect noise. O(N).
  2. *A spanning set instead of the matrix*: all reads -> one fixed write
     (isolates parsers; the write side is held constant) plus one fixed
     read -> all writes (isolates generators). 2N lanes with the same fault
     coverage the full matrix has once nothing is pair-keyed.
  3. Later (Parse/Generate Identity milestone): AST-shape parity comparing
     `parse(sql, d)` directly against SQLGlot's parsed AST — read-side
     measurement with no generator involved.
- **Switch-over check:** any failure a retired pair lane can show that the
  spanning set plus identities cannot reproduce means something is still
  secretly pair-keyed. Run the full matrix once at switch-over as proof, and
  once after any suspicious divergence.
- Budgets and row-level diffs move from per-pair reports to per-dialect
  identity reports and the spanning lanes.

### Non-sqlite write-target baselines (2026-06-09, forced suite)

First measured baselines for the O(N^2) hole, and the counts after the
Phase 1.5 function/type relocation landed the same day. The latest counts add
the 2026-06-10 COALESCE/date-time and Postgres JSON path parser relocation
work plus the 2026-06-11 formatted-time, JSON aggregate, date/time arithmetic,
timezone conversion, and compact expression-oddity parser/generator relocation
work, plus the 2026-06-14 SQLite CREATE TABLE generator relocation
(reports in `parity/reports/`):

| lane | match (baseline) | match (latest) | mismatch (latest) |
| --- | ---: | ---: | ---: |
| postgres -> postgres | 7100 | 8335 (+1235) | 4639 |
| mysql -> postgres | 6405 | 7661 (+1256) | 5059 |
| sqlite -> postgres | 6508 | 7700 (+1192) | 5195 |
| postgres -> duckdb | 6626 | 7015 (+389) | 5959 |
| postgres -> sqlite | 11871 | 11931 (+60) | 1043 |
| mysql -> sqlite | 11676 | 11695 (+19) | 1028 |
| sqlite -> sqlite | 11856 | 11863 (+7) | 1032 |

Zero row-level regressions across all seven lanes. The single biggest
write=postgres bucket was the cast style: SQLGlot renders `CAST(x AS T)` for
every target and never `::`. `* -> sqlite` is ~91% match; the postgres lanes
moved from ~50-55% to ~57-63%; relocating the remaining rules per the phases
above continues the burn-down.

### Transform-rule audit (2026-06-09)

Where the remaining `(source, target)` rules in `src/dialects/mod.rs` belong
(verified against the Python SQLGlot oracle, full read x write sweeps):

- **Read-side parser normalization** (source-keyed; ~61 source-branch sites):
  - `NOW()` -> CurrentTimestamp only for postgres-family, presto-family,
    databricks, exasol sources; `GETDATE()` -> CurrentTimestamp only for
    tsql-family, redshift, snowflake, databricks. Other sources keep them as
    anonymous functions. (Our parser canonicalized `NOW` for every source.)
  - MySQL cast types `SIGNED`/`SIGNED INTEGER` -> BIGINT and
    `UNSIGNED`/`UNSIGNED INTEGER` -> UBIGINT are tokenizer keywords in SQLGlot
    (read-side), not transform rules.
  - MySQL `TIMESTAMP` -> TIMESTAMPTZ at parse (SQLGlot tokenizer keyword);
    write side maps TIMESTAMPTZ back to `TIMESTAMP` for a mysql target.
  - `SUBSTR`/`SUBSTRING`, `LEN`/`LENGTH`, `RANDOM`/`RAND` canonicalize at
    parse for all sources (SQLGlot `_sql_names` aliases on one expression).
  - Landed 2026-06-10: postgres `JSON_EXTRACT_PATH` /
    `JSON_EXTRACT_PATH_TEXT` now parse into a typed JSON path node that
    preserves source path segments for identity and renders per target. The
    old postgres-family `JSON_EXTRACT_PATH` transform arm is gone, and
    postgres->duckdb improved to 6773 matches.
  - mysql `FROM_UNIXTIME` / postgres `TO_TIMESTAMP` time-format arms, and the
    other `is_*_family(source)` arms in `transform_expr` (concentrated
    2038-3200).
- **Write-side generator lowering** (target-keyed only):
  - CurrentTimestamp rendering: `GETDATE()` (tsql-family, redshift), `NOW()`
    (doris), bare `CURRENT_TIMESTAMP` (postgres-family sans redshift,
    presto-family, sqlite, duckdb, oracle, drill, druid, teradata),
    `CURRENT_TIMESTAMP()` (mysql-family sans doris, bigquery, hive-family,
    snowflake, clickhouse, dremio, exasol, tableau).
  - Rand rendering: `RANDOM` (postgres-family, sqlite, duckdb, snowflake,
    teradata), `DBMS_RANDOM.VALUE` (oracle), `randCanonical` (clickhouse),
    `RAND` otherwise.
  - Substring rendering: `SUBSTR` (oracle, presto-family),
    `SUBSTRING(x FROM a FOR b)` (postgres-family), `SUBSTRING(x, a, b)`
    otherwise.
  - Length rendering: `LEN` (tsql-family), `CHAR_LENGTH` (mysql-family,
    clickhouse) for character length, `LENGTH` otherwise.
  - Type maps: `rules::map_type` plus the target-only arms of
    `map_data_type` (sqlite affinity, bigquery INT->BIGINT, BYTEA<->BLOB).
- **Genuinely unresolved / needs an AST distinction** (string-named
  `Expr::Function` cannot carry SQLGlot's node flags):
  - `Length(binary=...)`: mysql/clickhouse/snowflake/bigquery parse `LENGTH`
    as byte length (renders `LENGTH` even where char length renders
    `CHAR_LENGTH`; duckdb target lowers it to a `CASE TYPEOF(...)` form).
    Modeled for now with two canonical names (`CHAR_LENGTH` vs `LENGTH`).
  - Landed 2026-06-10: `Expr::Coalesce` now carries SQLGlot's `is_nvl`,
    `is_null`, and source spelling metadata, so the `NVL`/`ISNULL` transform
    rename hook is gone.
  - ASOF JOIN method and source-aware CROSS/OUTER APPLY need a new
    `JoinClause` field shared with `internal_ast` (tracked under Transpile
    Parity).

## Track Boundaries

Three efforts are intentionally moving together, but they are not the same
track:

- **Parity Architecture:** the public parse -> canonical AST -> generate path.
  This owns correctness work: moving source behavior into parsers, moving target
  behavior into generators, and deleting the source->target transform layer.
- **SQLGlot Inventory Codegen:** development tooling that mines Python SQLGlot's
  declarative tables. Generated inventories should feed public parser/generator
  work first. The checked-in inventory sample is intentionally limited to
  postgres/mysql/sqlite until a larger artifact has a CI freshness check or a
  production consumer.
- **Internal Fast Path:** a private borrowed/zero-copy path for performance. It
  may consume shared static rule data after public output parity is proven, but
  it must keep byte-for-byte guards against the public generator and must not
  become the place where new SQLGlot behavior is defined.

Default rule: make the public SQLGlot-parity pipeline correct first, then let
codegen and the internal fast path reuse that knowledge.

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

Use this loop for port work. The unit of work is a behavior family (a
function, a type, a clause), not a report bucket.

1. Pick a behavior family — from the transform-rule audit, from a transform
   arm slated for deletion, or from a SQLGlot per-dialect table not yet
   ported.
2. Read how SQLGlot actually implements it: the local checkout has
   per-dialect parsers in `sqlglot/parsers/<name>.py` and generators in
   `sqlglot/generators/<name>.py`. Look for node flags
   (`Length(binary=...)`, `Coalesce(is_nvl=...)`) — they decide the AST
   shape.
3. Sweep the oracle: read x write matrix including every identity pair
   (`transpile(sql, read=r)` with no write defaults write=read). Identity
   transpiles skip the transform layer but always run the generator, so the
   generator must be correct for them.
4. Port it: source-keyed behavior goes into the parser as canonicalization;
   target-keyed behavior goes into the generator (a `rules.rs` data table if
   it can be data, a typed arm if structural). If SQLGlot carries a node
   flag, add the field to the `TypedFunction`/AST variant rather than
   inventing a string canonical.
5. Delete the transform arms the port replaces.
6. Lock focused Rust regression tests near the owning behavior, including
   identity cases.
7. Run the Rust gate, the curated parity corpus, and the seven forced lanes;
   require zero row-level regressions (diff report JSONL against git HEAD,
   not just status counts).
8. Update [CHANGELOG.md](CHANGELOG.md). Commit per family.

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

Current work (forced lanes, write=sqlite; latest counts pg 11927 / my 11691 /
sq 11860):

- Drive the Architecture Port: relocate target rules into the generator and
  source rules into the parser so the same fix lifts non-sqlite write lanes.
- Burn down high-volume mismatch clusters by feature family, ratcheting every
  change on the forced suite with zero real regressions per lane.
- Reduce forced-pair `rust-error` buckets, because parser coverage reveals the
  real mismatch backlog (many INT->INTEGER / VARCHAR->TEXT buckets are
  whole-statement parse-failure fallbacks on exotic DDL, not type-map gaps).
- Add row-level budget diffing so one fixed row cannot hide a new broken row.

Clean mechanical wins are largely exhausted. The remaining high-value buckets
are structural and warrant careful, interactive work: comment preservation
(~85/lane), QUALIFY -> subquery elimination (~87), ASOF JOIN (~31), source-aware
CROSS/OUTER APPLY (~30), bigquery array literals (~36), and pipe syntax `|>`
(~40). Several need a new field on a shared `crate::ast` struct that the perf
`internal_*` paths also construct, so they require an explicit decision before
proceeding.

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

## SQLGlot-Derived Codegen Inventories

Status: spike landed (`tools/sqlglot_codegen/`), not yet wired into production.

The default parity loop finds gaps one at a time: pick a report bucket, find the
one diverging case, fix it, lock a regression. Much of SQLGlot's per-dialect
knowledge is already declarative data, so we can mine it directly and turn "find
the next gap" into "diff two data sets".

Direction:

- Keep a codegen tool that imports a local Python SQLGlot checkout and
  introspects its dialect classes (no regex over Python source). It emits
  deterministic, version-stamped JSON inventories: dialect names, tokenizer
  `KEYWORDS`, generator `TYPE_MAPPING`, classified generator `TRANSFORMS`, parser
  `FUNCTIONS`, `TIME_MAPPING`, and the expression-class `arg_types` schema.
- Generate Rust dialect tables from the portable subset (e.g. function renames
  recovered from `rename_func`). Generated files are `@generated`, sorted, and
  rustfmt-clean static data.
- Port only structural parser/generator behavior by hand. Dynamic transforms
  (`lambda`/`named` helpers) are classified and fenced off, never auto-translated.
  This is data extraction, not py2many-style source translation.
- The parity harness stays the referee. Inventories say *where to look*; the
  oracle still decides correctness.
- Treat the extractor as a sync tool, not a report: re-running after a SQLGlot
  bump produces a clean diff of exactly what changed upstream, so inventories do
  not rot. A CI freshness check can gate drift once tables are wired in.

This supersedes the "generated reports" deliverable under AST Expansion: the
expression inventory is one of the JSON outputs above.

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

## Performance Architecture Program

Status: in progress.

Performance work must preserve SQLGlot parity and must be measured with the
checked-in MySQL -> SQLite, Postgres -> SQLite, and SQLite -> SQLite workloads.
Each landed slice should update [docs/PERFORMANCE.md](docs/PERFORMANCE.md),
refresh allocation reports when allocation behavior changes, and run the same
Rust/parity tests as behavior work.

### 1. Token Ownership And Source Spans

Goal: stop treating every parser-internal token view as an owned string while
keeping the public tokenizer API predictable. Tokens should carry `TokenType`,
source span, quote metadata, and location; future internal parser paths should
borrow text from source SQL unless decoding or normalization is required.

Current status: tokens now carry byte end spans and raw parser carriers use
source text where needed. Public punctuation token values remain populated for
API compatibility; the deeper allocation win requires a borrowed/internal token
representation rather than empty public values.

Acceptance:

- Tokenizer and parser preserve raw SQL slices exactly for raw carriers,
  comments, dollar-quoted bodies, table tails, and error messages.
- Tokenizer tests cover ASCII tokens, quoted identifiers, strings, escaped
  strings, Unicode text, comments, bracket identifiers, and multi-character
  operators.
- `cargo test --features cli --test test_transpile` stays green.
- Allocation reports show whether token-value ownership changed bytes/op and
  allocs/op.

### 2. AST String Ownership

Goal: keep the public AST owned and serde-friendly while reducing temporary
parse/transpile allocation. Candidate designs are a string interner, parse-local
arena, or borrowed internal AST that is converted to owned only at API
boundaries.

Execution plan:

1. Add a private internal text type, `SqlText<'sql>`, backed by borrowed source
   slices or owned rewrite strings. Use it first in parser/raw reconstruction
   helpers so the abstraction is exercised without changing public AST shapes.
2. Add a private internal AST subset for the hot transpile surface: SELECT,
   columns, aliases, literals, function calls, binary operations, casts, WHERE,
   ORDER BY, LIMIT, and the DDL shapes present in benchmark workloads.
3. Build `parse_internal(sql, dialect)` for that subset. It should borrow source
   text when semantic text equals the source slice, allocate only for decoded
   literals or normalized identifiers, and fall back to the current public parser
   for unsupported shapes during bring-up.
4. Add internal transform and generator paths so covered `transpile()` calls can
   run `parse_internal -> transform_internal -> generate_internal` without
   converting through the public owned AST. Output must match the current public
   path byte-for-byte before the internal path is enabled for a case.
5. Ratchet coverage by report bucket and benchmark case. Track which cases are
   fully internal versus fallback, gate internal output against the public path,
   and remove fallbacks only after SQLGlot parity and focused Rust tests stay
   green.

Current status: steps 1 and 2 are landed. Step 3 has started with a private
token-driven `parse_internal(sql, dialect)` for a narrow SELECT subset covering
simple select items, wildcard items, table aliases, decoded string literals,
borrowed identifiers/numbers, function calls, one simple binary predicate,
WHERE, GROUP BY, ORDER BY, LIMIT, OFFSET, `DISTINCT` function arguments, simple
window `OVER` specs, simple CTEs, and comma-from normalization. It also has a
SQLite-only raw identity carrier for the current DDL/INSERT/ALTER benchmark
rows. It intentionally falls back for non-SQLite raw statements, explicit joins,
deeper windows, qualified wildcards, and operator-precedence chains. Step 4 has
started with a private `generate_internal(...)` for the same subset, a guarded
internal transpile experiment, a no-oracle `transpile_internal_fast_experiment(...)`
for conservative identity cases, and a status report binary that classifies
internal fast-path coverage. The no-oracle path is deliberately limited to
dialect identity pairs and rejects pseudo-columns that the public generator
canonicalizes. The current SQLite identity workload report covers 8 of 8 rows
with 0 guarded output mismatches; the supported-row Criterion comparison shows a
diagnostic ~2x speedup for those rows. Public `transpile()` remains unchanged
until internal coverage broadens beyond this benchmark slice and the guarded
reports stay clean.

Expected benefit:

- Lower single-call allocation and latency for Rust, C ABI, PyO3, Node, Ruby,
  and Go bindings.
- Public `parse()` remains owned, serde-friendly, and lifetime-free.
- Hot `transpile()` can avoid allocating public AST strings that callers never
  inspect.
- Scoped allocation reports should move pressure out of parse; if they do not,
  the internal path is not earning its complexity.

Acceptance:

- Public AST/serde API remains stable or changes behind a clearly named
  experimental API.
- Parse/generate identity tests and SQLGlot bridge reports do not regress.
- Benchmarks include AST-returning APIs separately from `transpile(...)`.

### 3. Fast Paths Without Parity Drift

Goal: add safe fast paths only for cases where SQLGlot-equivalent output is
obvious and testable, such as raw command passthroughs or same-dialect identity
statements that need no rewrite.

Acceptance:

- Every fast path has a forced opt-in predicate and a fallback to full
  parse/transform/generate.
- SQLGlot parity fixtures cover each fast path and at least one near miss.
- No fast path bypasses dialect behavior that SQLGlot would normalize.

### 4. Generator Capacity And Output Buffers

Goal: reduce generator allocation for larger SQL by estimating output capacity
and supporting caller-owned buffers for FFI/binding paths.

Acceptance:

- `generate` and FFI/PyO3 benchmark reports distinguish owned-output and
  caller-buffer modes.
- Generated SQL remains byte-for-byte identical to Python SQLGlot expectations.

### 5. Case Normalization And Keyword Checks

Goal: remove repeated uppercase allocation and normalize keyword/context checks
through token kinds, ASCII-insensitive comparisons, or parser-local interned
views.

Acceptance:

- Clippy remains clean.
- Parser decision points have focused tests for case-insensitive keywords and
  case-sensitive quoted identifiers.
- Allocation reports distinguish tokenize/parse wins from transform/generate
  wins.

### 6. In-Place Transform Architecture

Note: interim. The Architecture Port drains the source->target transform layer
toward deletion (Phase 3), so in-place transform work applies only to rules that
have not yet been relocated into the parser or generator. Do not invest in
transform internals that an active port phase is about to remove.

Goal: make remaining dialect transforms mutate owned AST nodes in place wherever
possible, only allocating when a SQLGlot rewrite genuinely creates a different
shape.

Acceptance:

- Transform helpers take `&mut Expr` / `&mut Statement` for hot recursive paths.
- Existing SQLGlot parity reports do not regress.
- Allocation phase reports demonstrate improvements on expression-heavy rows.

### 7. Callsite-Aware Profiling

Goal: graduate from phase-level allocation counts to callsite-aware evidence
before deeper invasive rewrites.

Acceptance:

- Add documented workflows for Instruments/DTrace/samply/DHAT-style profiling
  on macOS.
- Keep generated reports small and reproducible; do not check in huge profiler
  artifacts.
- Use callsite evidence to choose between token spans, AST interning, generator
  buffers, and transform rewrites.

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
