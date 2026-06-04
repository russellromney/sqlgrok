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
    `transform_owned` has an identity short-circuit
    (`if from == to && !sqlite { return statement }`), and function rendering
    currently lives in `transform_expr` (`map_function_name_for_source` call).
    So if the parser canonicalizes a source spelling to a neutral name (e.g.
    mysql `BIT_AND` → `BITWISE_AND_AGG`), an identity transpile (mysql→mysql)
    skips the transform and leaks the neutral name. Proven by a reverted
    `BIT_*` spike: cross-dialect was correct, both identity cases regressed.
    **Therefore Phase 2 must be preceded by Phase 1.5: relocate function (and
    type) rendering from `transform_expr` into the generator**, which always
    runs. The `rules::rename_function`/`map_type` tables already exist; only
    the *call site* must move (transform → generator). Mind that making the
    generator always render will change identity-transpile output for any name
    currently left raw — verify against the suite, and reconcile with the
    perf fast path which also handles identity.
  - **Phase 1.5 status:** function-name rendering moved into the generator
    (`Expr::Function` emission consults `rules::rename_function(target)`),
    verified zero-movement and identity-safe. Remaining Phase 1.5: move
    *type* rendering and the rest of `map_function_name_for_source`'s
    target-only arms (NOW, LENGTH, ANY_VALUE, ...) into the generator too,
    then read-side canonicalization (Phase 2) is unblocked.
- **Phase 3 — delete the transform layer** and the `(source, target)`
  signature. `transpile = generate(parse(read), write)`.
- **Phase 4 — port SQLGlot's per-target tables** to backfill thin generators;
  multi-target suite measures it. Adding a dialect = filling its tables.

## Standing decisions

- Single `sql_generator.rs` branching on target with target-keyed data tables
  (not per-dialect generator modules yet — churn without correctness benefit).
- Rule-of-thumb per relocated rule: **"can this be data?"** Yes → shared table
  (both pipelines + SQLGlot port). No (structural) → owned generator; fast
  path declines.
- Verification: forced suite is the ratchet, run for multiple read/write
  pairs. Rebuild Python bindings (uv venv in `python/.venv`) before measuring.

## Open coordination item

The perf session's `InternalGenerator` reading the shared `dialects::rules`
tables is a yes/no from them. Non-blocking: the owned generator consumes the
tables immediately; the fast path adopts when ready. Zero coupling pressure.
