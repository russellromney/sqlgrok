# Porting SQLGlot's architecture: killing the source→target transform

## Goal

Converge sqlgrok onto SQLGlot's actual architecture so dialect work is O(N),
not O(N²):

```
transpile(sql, read, write) = generate(parse(sql, read), write)
```

No `transform(source, target)` middle. All dialect knowledge lives in two
homes:

- **Parser (read side):** resolves source-dialect ambiguity into a canonical,
  dialect-neutral AST. Anything that depends on `source` belongs here.
- **Generator (write side):** TYPE_MAPPING, function renames, node→SQL
  transforms, keyword filters, quoting. Everything keyed on `target` only.

Discipline: after parse the AST must be neutral enough that the generator
alone is correct. A generator that needs to know the source is a bug — the
parser didn't finish canonicalizing.

## Why this is mostly relocation, not rewriting

Both ends already exist and are already dialect-parameterized
(`sql_parser.rs` takes a `Dialect`; `sql_generator.rs` branches on target).
The `transform_*` layer in `dialects/mod.rs` is a vestigial middle doing work
that belongs in the two ends.

Audit of `dialects/mod.rs` (~5,365 lines):

- **175 target-only branches** → mechanical relocation into the generator.
- **69 source branches** → real read-normalization work, but concentrated:
  45 in `transform_expr`, 9 in `transform_statement`, ~15 scattered. These are
  the silent-wrong-output risks; each moves to the parser deliberately.
- **49 raw-text rewrite fns** → post-pass / largely throwaway pile.
- 163 fns thread `source`.

So: ~70% mechanical, ~30% careful, and the careful part is localized.

## The O(N²) hole, measured

Forced suite, second write target (baseline captured 2026-06):

- `* → sqlite`: ~91% match.
- `* → postgres`: ~50% match (mysql→pg 6354/6366, sqlite→pg 6482/6413).

Half the cases break the moment the target isn't sqlite. Breakage is (a)
generator thinness (`ALTER ... TYPE` vs `SET DATA TYPE`, types/functions not
rendered per-target) and (b) comment preservation (`-- x` → `/* x */`), a
cross-cutting fidelity gap. Relocating target rules into the generator
directly fills this hole.

## Rules as DATA — the bridge to zero-copy and to the SQLGlot port

Author the high-volume relocated rules as **data tables**, not match arms:

```rust
// src/dialects/rules.rs — representation-neutral, &'static str in/out, zero-alloc
pub(crate) fn rename_function(target: Dialect, upper_name: &str) -> Option<&'static str>
pub(crate) fn map_type(target: Dialect, canonical: &str) -> Option<&'static str>
// + unsupported / keyword-drop sets
```

Data is representation-neutral, so one artifact does three jobs:

1. **Kills the source gate** — target-keyed only.
2. **Feeds zero-copy** — both the owned `Generator` and the perf session's
   borrowed `InternalGenerator` emit by `push_str`-ing `&str` into one output
   buffer. A rename is `lookup → push_str(&'static str)`; the transformed text
   never becomes a per-node `String`, so it's **zero-allocation in both
   pipelines**. (`SqlText` is `Cow<Borrowed|Owned>`; no `Static` variant
   needed because lowering happens at generate-time, not in the AST.)
3. **Ports from SQLGlot** — the tables are the exact shape of SQLGlot's
   per-dialect dicts; transcribe (or script-generate) instead of
   reverse-engineering from suite diffs.

### Two ASTs, sharing rule-data not representation

The perf session's `internal_*` is a zero-copy fast path for identity
(read==write) transpiles, source-preserving, narrow SELECT subset, guarded
(validates against / falls back to the public pipeline). It is orthogonal and
insulated.

- **Do NOT** retrofit the main owned AST (`crate::ast`) to be
  lifetime-parameterized. It derives serde and feeds the optimizer, executor,
  FFI, and Python bindings — borrowed ASTs fight all of those. That blast
  radius is exactly why the perf session forked a separate `internal_ast`.
- Keep **two ASTs** — owned (correctness/optimizer/FFI) and borrowed (fast
  path) — and have them **share rule-data**. As more rules become tables, the
  fast path widens from identity-only to identity + table-driven transforms;
  the owned pipeline becomes the fallback only for structural rewrites
  (DISTINCT ON → ROW_NUMBER subquery, pivot lowering) that allocate anyway.

## Phased plan

Each step ratcheted on the forced suite across all lanes (now including
`write=postgres`); no real regressions; commit + merge per slice.

- **Phase 1 — relocate target rules into the generator (mechanical).**
  Order: function renames → type mappings → target-only expr/stmt lowerings.
  Authored as `dialects/rules.rs` data tables consumed by the generator. Drop
  `target` from transform arms as they empty. Improves `write=postgres` for
  free wherever a rule was actually general.
  - **Step 1 (in progress): function renames** → `rules::rename_function`.
- **Phase 2 — move the 69 source branches into the parser** as canonical-AST
  normalizations. Each is a silent-wrong risk → parser change + suite check.
  Concentrated in `transform_expr`.
  - **Prerequisite discovered (2026-06):** read-side canonicalization at parse
    time is *unsafe until function/type rendering moves into the generator*.
    The historical `transform_owned` identity short-circuit
    (`if from == to && !sqlite { return statement }`) made read-side
    canonicalization unsafe until function/type rendering moved into the
    generator. If the parser canonicalized a source spelling to a neutral name
    (e.g. mysql `BIT_AND` → `BITWISE_AND_AGG`), an identity transpile
    (mysql→mysql) skipped transform and leaked the neutral name. Proven by a
    reverted `BIT_*` spike: cross-dialect was correct, both identity cases
    regressed. **That prerequisite has landed: built-in transpile is now
    direct parse/generate, with rendering owned by generators**, which always
    runs. The `rules::rename_function`/`map_type` tables are now consumed from
    generator-owned rendering, so identity-transpile output is exercised on the
    same path as cross-dialect output.
  - **Phase 1.5 status:** function-name rendering moved into the generator
    (`Expr::Function` emission consults `rules::rename_function(target)`),
    verified zero-movement and identity-safe. The verified-correct sqlite
    renames (ANY_VALUE->MAX, GEN_RANDOM_UUID->UUID) have moved too.
  - **Phase 1.5 update (2026-06-09):** the divergent multi-target families
    are now ported faithfully per SQLGlot instead of relocated. RAND/RANDOM
    canonicalize at parse and render per target (RANDOM, DBMS_RANDOM.VALUE,
    randCanonical, RAND). NOW()/GETDATE() canonicalize into
    TypedFunction::CurrentTimestamp only for the sources whose SQLGlot
    parsers do (postgres/presto families, databricks, exasol; tsql family,
    redshift, snowflake), and bare CURRENT_TIMESTAMP parses to the node for
    every source but clickhouse; `rules::render_current_timestamp(target)`
    owns the GETDATE()/NOW()/bare/parens spellings. LEN/LENGTH/CHAR_LENGTH
    flow through TypedFunction::Length with a new `binary` flag mirroring
    SQLGlot's byte/char distinction (mysql family, clickhouse, snowflake,
    bigquery byte-LENGTH); mysql/clickhouse targets render CHAR_LENGTH for
    char length. SUBSTR/SUBSTRING render per target (SUBSTR for
    oracle/presto family, SQL-standard FROM/FOR for the postgres family).
    Type mappings: mysql TIMESTAMP -> TIMESTAMPTZ and SIGNED/UNSIGNED ->
    BIGINT/UBIGINT at parse (tsql TIMESTAMP -> ROWVERSION); the generator's
    Timestamp/DateTime arms hold the per-target TYPE_MAPPING table; mysql
    CAST lowers to its SIGNED/UNSIGNED/CHAR cast set; postgres casts render
    CAST(x AS T), never `::`.
  - **Phase 1.5 update (2026-06-10):** COALESCE-family spellings now have a
    typed AST home. `Expr::Coalesce` carries SQLGlot-style `is_null`,
    `is_nvl`, and source spelling metadata, so T-SQL/Fabric `ISNULL`, Oracle
    `NVL`, and BigQuery/ClickHouse `IFNULL`/`NVL` identity behavior render in
    the generator. MySQL-family `ISNULL` parses as the unary `(x IS NULL)`
    predicate. The remaining `map_function_name_for_target` transform hook was
    deleted.
  - **Phase 2 starter (2026-06-10):** date/time aliases started moving out of
    `transform_expr`. The parser now canonicalizes SQLGlot source-native
    `CURDATE`, `MAKETIME`/`MAKE_TIME`/`TIME_FROM_PARTS`, and one-argument
    `FROM_UNIXTIME`/`TO_TIMESTAMP` into typed nodes, and the generator owns the
    target spellings (`CURRENT_DATE`, `MAKETIME`, `MAKE_TIME`,
    `TIME_FROM_PARTS`, `FROM_UNIXTIME`, `TO_TIMESTAMP`, `UNIX_TO_TIME`,
    `TIMESTAMP_SECONDS`). The replaced sqlite-only transform arms are gone.
  - **Phase 2 update (2026-06-10):** Postgres
    `JSON_EXTRACT_PATH` / `JSON_EXTRACT_PATH_TEXT` now parse into a typed JSON
    path node that preserves segment lists for identity output and renders per
    target. The postgres-family `JSON_EXTRACT_PATH` transform arm is gone; the
    remaining `JSON_EXTRACT_PATH_TEXT` SQLite fallback is target-only for
    non-Postgres generic-function parses.
  - **Finding (2026-06): the multi-target renames in
    `map_function_name_for_source` (NOW, LEN/LENGTH, SUBSTR/SUBSTRING,
    IFNULL/ISNULL, NVL, RANDOM/RAND) DIVERGE from SQLGlot for identity** and
    cannot be pure-relocated. The transform's identity short-circuit
    (`from==to && !sqlite`) hides the divergence today. Examples (read==write
    vs SQLGlot): mysql `LEN`->our `LENGTH` but SQLGlot `CHAR_LENGTH`; mysql
    `SUBSTR`->our `SUBSTR` but SQLGlot `SUBSTRING`; mysql `IFNULL`->our
    `IFNULL` but SQLGlot `COALESCE`; postgres `NOW`->our `NOW` but SQLGlot
    `CURRENT_TIMESTAMP`. So moving these to the always-run generator would
    EXPOSE latent bugs. They must be ported faithfully against SQLGlot's
    per-dialect `TRANSFORMS` (Phase 4 work, per function family) rather than
    relocated. Doing so also lifts `write=postgres`/`write=mysql`. The
    sqlite-target renames are safe to move because sqlite identity is not
    short-circuited and those renames already match SQLGlot.
- **Phase 3 — delete the transform layer** and the `(source, target)`
  signature. `transpile = generate(parse(read), write)`.
- **Phase 4 — port SQLGlot's per-target tables** to backfill thin generators;
  multi-target suite measures it. Adding a dialect = filling its tables.

### Phase 3 retirement runway

Phase 3 is now concrete: the remaining work is structural, not another sweep
of ordinary function rewrites.

1. **Raw aggregate ORDER BY carriers.**
   Postgres aggregate calls with `ORDER BY` can still parse their args as raw
   text, and the transform layer used to inject SQLite `NULLS FIRST/LAST`
   semantics into that string. The first slice now carries that source
   semantics as parser-owned `RawOrderNullsPolicy` and renders it in the
   SQLite generator, deleting the source-gated `transform_expr` branch. The
   second slice adds structured `Expr::Function` fields for function-local
   `ORDER BY` and `LIMIT`, with parser/generator ownership for the common
   aggregate family. Remaining raw fallback is now limited to harder tails such
   as `HAVING`, `IGNORE`/`RESPECT NULLS`, and comma-style `LIMIT`.
2. **Raw table-source carriers.**
   `TableSource::Raw` still encodes behavior for `UNNEST(...)`, `WITH OFFSET`,
   Postgres VALUES alias cleanup, typed literals, and raw function name
   normalization. Replace those with richer table-source nodes/fields:
   ordinality, offset aliases, alias column lists, table-function tails,
   `ROWS FROM`, and JSON/XML table carriers as needed. The first execution
   slice now moves the existing raw-text rewrite policy into parser-owned
   `RawTableSourceNormalization`, so SQLite raw table-source rendering no
   longer keys off `source_dialect`. The next slice structures
   single-expression BigQuery/Postgres `UNNEST(...)`: alias column lists,
   `WITH OFFSET`, offset aliases, generated BigQuery offset aliases, and
   Postgres `WITH ORDINALITY` are now AST fields rendered by generators.
   Another slice adds `TableSource::RowsFrom` for Postgres `ROWS FROM (...)`,
   including function aliases, typed alias-column lists, table-level aliases,
   and `WITH ORDINALITY`. Another slice adds a semi-typed
   `TableSource::RawTableFunction` carrier for `JSON_TABLE(...)` and
   `XMLTABLE(...)`, keeping the complex inner body textual while moving alias
   handling and SQLite type normalization into parser/generator ownership.
   Another slice adds `TableSource::TableWithTails` for base tables followed
   by raw dialect tails such as `LATERAL VIEW`, `AT`, `BEFORE`, and `CHANGES`,
   letting helper passes see the structural base table while generators own
   tail rendering. The remaining work is to eliminate raw string carriers for
   multi-argument `UNNEST`, parenthesized/table-function fallback shapes, and
   tokenizer fallback cases where forced-read syntax is not expression-shaped.
3. **Raw statement normalization.**
   `RawStatement` still rewrites Postgres enum/raw recursive CTE/COPY, MySQL
   `SHOW`, raw `PIVOT`/`UNPIVOT`, and insert-into-function text. Add typed or
   semi-typed statement variants (`CommandKind` where full modeling is not
   worth it yet) so raw passthrough is inert. The first execution slice now
   carries the existing rewrite policy as parser-owned
   `RawStatementNormalization`, so SQLite raw statement rendering no longer
   keys off `source_dialect`; the remaining work is to replace raw text with
   typed or semi-typed statement variants where practical. The next slice adds
   `Statement::Command` / `CommandKind` for `COPY`, MySQL `SHOW`, standalone
   `PIVOT` / `UNPIVOT`, statement-level `REPLACE(...)`, SQLite database
   commands, and Postgres `CREATE TYPE ... AS ENUM`, slimming
   `RawStatementNormalization` down to the remaining recursive CTE and
   insert-into-function raw gaps.
4. **Target-only generator lowerings.**
   `ILIKE`, `DISTINCT ON`, `SEMI`/`ANTI` joins, SQLite `WITHIN GROUP`
   dropping, limit/top/fetch normalization, lock dropping, quote conversion,
   and SQLite identity join cleanup now belong to generator-owned rendering or
   explicit parser canonicalization. Current progress: `ILIKE` fallback, SQLite
   `GROUP_CONCAT`/`STRING_AGG` `WITHIN GROUP` omission, lock omission, and
   quoted identifier spelling, SQLite identity join cleanup, SQLite
   `DISTINCT ON` lowering, SEMI/ANTI join lowering for unsupported targets,
   and limit/top/fetch normalization are now generator-owned, with parser-owned
   style metadata preserving identity spelling where needed.
5. **AST gap closure.**
   Do not relocate raw-text rewrites into another helper and call that done.
   Add the missing AST shape first, then drain behavior from raw carriers and
   parser fallbacks. The transform layer is gone; do not recreate pair-keyed
   behavior elsewhere.

Deletion gate:

- Landed: replace `transform_owned` with a no-op compatibility shim and run
  the full pair matrix as switch-over proof.
- Landed: delete the shim and builtin `(source, target)` transform signatures.
- Remaining guardrail: new dialect behavior must be parser-owned,
  generator-owned, or backed by new AST structure. Do not reintroduce pair-keyed
  builtin transform code.

## Standing decisions

- Single `sql_generator.rs` branching on target with target-keyed data tables
  (not per-dialect generator modules yet — churn without correctness benefit).
- Rule-of-thumb per relocated rule: **"can this be data?"** Yes → shared table
  (both pipelines + SQLGlot port). No (structural) → owned generator; fast
  path declines.
- Verification: forced suite is the ratchet, run for multiple read/write
  pairs. Rebuild Python bindings (uv venv in `python/.venv`) before measuring.

## Phase 4 — transcribing SQLGlot's dicts (the efficiency unlock)

`parity/scripts/transcribe_sqlglot_types.py` reads each builtin dialect's
`Generator.TYPE_MAPPING` and emits per-target Rust `map_type_<dialect>`
tables. This is the "port the dicts as data" tool — re-runnable, no
hand-discovery.

**Critical constraint discovered (2026-06): `TYPE_MAPPING` is keyed by the
canonical `DataType.Type` enum, which the parser only assigns when the SOURCE
dialect recognizes the spelling.** So it is NOT a flat source-independent
string table. Examples:
- `DATETIME2 -> TIMESTAMP` (sqlite target) fires only for a TSQL source;
  sqlite/postgres source leaves `DATETIME2` unchanged (it's an unrecognized
  UDT there). A flat `map_type(Sqlite, "DATETIME2") -> "TIMESTAMP"` would
  WRONGLY convert it for every source.
- Safe to flat-transcribe: types recognized as the same canonical by every
  source (TINYINT, DOUBLE, FLOAT, NCHAR, NVARCHAR, ...). e.g.
  `TINYINT -> SMALLINT` for a postgres target is source-independent.

So consuming the transcription requires splitting each dialect's TYPE_MAPPING
into (a) the source-independent subset → flat `map_type` tables (safe now),
and (b) the source-exclusive subset → typed `DataType` variants assigned by a
per-source parser (the bigger lift, mirrors SQLGlot's parser FUNCTIONS/
KEYWORDS). The script output should be filtered to (a) before integration.

Integration also needs the generator's `gen_data_type` to consult
`map_type(target, canonical_name)` for TYPED variants (DataType::TinyInt ->
"TINYINT" -> table), not only the `Unknown(spelling)` path — that's what wires
the transcribed non-sqlite tables in and lifts `write=postgres`/`write=mysql`.

## Open coordination item

The perf session's `InternalGenerator` reading the shared `dialects::rules`
tables is a yes/no from them. Non-blocking: the owned generator consumes the
tables immediately; the fast path adopts when ready. Zero coupling pressure.
