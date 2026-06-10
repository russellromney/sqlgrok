# SQLGlot Codegen Inventory (spike)

This is a spike for mining Python SQLGlot as **source data** instead of
discovering parity gaps one at a time. It imports a local SQLGlot checkout,
introspects its dialect classes, and writes deterministic JSON inventories plus
one proof-of-concept Rust file. The checked-in generated sample is deliberately
small: postgres, mysql, sqlite, and a summary. The extractor can still generate
the full SQLGlot dialect inventory on demand.

The thesis: most of SQLGlot's per-dialect knowledge is already declarative
(keyword tables, type maps, function renames, format tables). We can extract
that directly and diff it against sqlgrok, rather than hand-discovering each
divergence through the parity harness.

## How to run

Requires `uv` and Python 3.10+. SQLGlot core is dependency-free, so the
extractor just adds the checkout to `sys.path` — nothing is installed and the
SQLGlot checkout is left pristine (no `.venv`, no build).

```bash
uv run --python 3.10 tools/sqlglot_codegen/extract.py \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --out generated/sqlglot_inventory \
  --dialects postgres,mysql,sqlite
```

Or use the wrapper (same defaults):

```bash
tools/sqlglot_codegen/run.sh
```

Flags:

- `--sqlglot PATH` — local Python SQLGlot checkout (required).
- `--out DIR` — output directory (required).
- `--dialects a,b,c` — restrict to a subset (default: all SQLGlot dialects).
- `--rust-dialect NAME` — which dialect to emit the Rust rename PoC for
  (default: `sqlite`).
- `--include-expressions` — also emit the large SQLGlot expression-class
  inventory.

Generate the full exploratory inventory when needed:

```bash
uv run --python 3.10 tools/sqlglot_codegen/extract.py \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --out /tmp/sqlgrok-full-inventory \
  --include-expressions
```

## What is extracted

Output lands under `generated/sqlglot_inventory/`:

- `meta.json` — SQLGlot version and commit the inventory was mined from.
- `dialects.json` — every dialect name SQLGlot exposes.
- `summary.json` — counts for the generated dialect subset, useful as a quick
  diff/sanity surface.
- `expressions.json` — optional full expression-class inventory with
  `arg_types` (SQLGlot's closest thing to an AST schema; 700+ classes). This is
  not checked in by default because it is large and not yet consumed.
- `dialects/<name>.json` — per dialect:
  - `tokenizer_keywords` — `KEYWORDS` (raw text → `TokenType` name), read from
    the tokenizer **class** (an instance compiles these into a trie and reports
    an empty dict).
  - `generator_type_mapping` — `TYPE_MAPPING` (canonical `DataType.Type` name →
    rendered SQL type).
  - `generator_transforms` — `TRANSFORMS` keys, classified (see below).
  - `parser_functions` — `FUNCTIONS` keys (function names parsed specially).
  - `time_mapping` — `TIME_MAPPING` (source format token → strftime token).
- `<dialect>_function_renames.rs` — Rust PoC emitter output.

The checked-in sample keeps only the priority dialects for the current project
surface:

- `dialects/postgres.json`
- `dialects/mysql.json`
- `dialects/sqlite.json`

That keeps the PR reviewable while preserving the ability to regenerate the
full SQLGlot inventory during oracle bumps or broader dialect work.

### Transform classification

`TRANSFORMS` values are Python callables, not data. The extractor classifies
each one so the portable/dynamic boundary is explicit:

- `rename` — produced by `rename_func("NAME")`. The function's only free
  variable is `name` bound to a string, so we recover the target name. **This is
  portable as static Rust data.**
- `named` — a named module-level helper (e.g. `_array_concat_sql`). Not directly
  portable, but the name tells you exactly which behavior to port by hand.
- `lambda` — an anonymous function. Behavior is code, not data; not portable.

Each entry also carries `dialect_specific` (whether it overrides the base
`Generator`), so consumers can focus on real divergence instead of inherited
defaults.

## The Rust emitter PoC

`<dialect>_function_renames.rs` proves we can turn extracted data into
plausible, stable Rust. It is a sorted `&[(&str, &str)]` of
`(expression_class, function_name)`, with a `@generated` header naming the
SQLGlot version/commit and the regenerate command. It is **static data, not
translated Python**, and it passes `rustfmt --check` unchanged.

It is intentionally **not** wired into the production crate yet. This spike is an
architecture proof, not a parity-burndown session.

## What is intentionally NOT extracted (yet)

- **Transform bodies.** `lambda`/`named` transforms are classified but not
  translated. Porting their behavior is deliberate hand work in the Rust
  generator.
- **Parser grammar.** `FUNCTIONS` keys are listed; the constructor logic behind
  each is not extracted.
- **Optimizer / semantic passes.** Out of scope for an inventory spike.
- **Inverse/round-trip time tables, settings, and the many smaller dialect class
  attributes.** Easy to add later; kept out to keep the first cut focused.

## How this replaces manual parity burndown

Today the loop is: pick a report bucket → find the one diverging case → fix →
add a regression. That finds gaps one at a time.

With generated inventories, the loop becomes a **diff between two data sets**:
SQLGlot's declared knowledge vs. sqlgrok's implementation. Missing keywords,
type mappings, function renames, and unported transforms show up as set
differences, not as individual mystery failures. The parity harness stays the
referee; this just front-loads *where to look*.

Because the output is deterministic and version-stamped, the same script can run
as a **sync tool**: re-extract after a SQLGlot bump and the JSON diff is exactly
what changed upstream — instead of producing yet another stale hand-written
report.

## Why this is not py2many-style source translation

py2many translates Python *source* into another language: it tries to port the
implementation. This spike does the opposite. It treats SQLGlot as a running
program, introspects its **declarative data**, and emits data. No Python control
flow is translated. Dynamic transforms are flagged as out-of-band on purpose, so
we never pretend a lambda became safe Rust. The Rust generator behavior is still
written by hand against the parity oracle; codegen only supplies the tables.

## Hostile review notes

- **Introspection stability.** Class-attribute introspection (`KEYWORDS`,
  `TYPE_MAPPING`, `TRANSFORMS`, `FUNCTIONS`, `TIME_MAPPING`, `arg_types`) is
  stable across SQLGlot versions; these are long-lived public-ish structures.
  The riskiest piece is `rename_func` detection, which depends on the closure
  having a single `name` free variable. If SQLGlot changes that helper's
  implementation, renames silently reclassify as `lambda` — visible as a count
  drop in `summary.json`, not a crash.
- **Class-name coupling.** Inventories are keyed by expression class name
  (`Chr`, `LogicalAnd`, …). If SQLGlot renames a class, the corresponding
  generated entry's key changes. That is a feature for a sync tool (the diff
  shows the rename) but means downstream Rust must not hard-code stale names.
- **Determinism.** All dicts/lists are sorted; no timestamps are emitted. Two
  runs against the same checkout are byte-identical (verified). The only
  intended variation is the SQLGlot version/commit in `meta.json` and file
  headers.
- **Dynamic transforms are explicitly fenced.** `lambda`/`named` are never
  presented as portable. Only `rename` entries reach the Rust emitter.
- **Sync-tool path.** Re-running on a SQLGlot bump produces a clean diff, so this
  can become a checked-in inventory with a CI freshness check rather than a
  report that rots.
