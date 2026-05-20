# SQLGlot Parity Harness

sqlgrok uses Python SQLGlot as a behavioral oracle while keeping the implementation pure Rust.

## Case Format

Parity cases are JSON Lines files under `parity/cases/`:

```json
{"id":"mysql-group-concat-separator-to-sqlite","sql":"SELECT GROUP_CONCAT(v SEPARATOR '|') FROM gc","read":"mysql","write":"sqlite"}
```

Fields:

- `id`: stable case identifier.
- `sql`: source SQL.
- `read`: source dialect name.
- `write`: target dialect name.
- `accepted_rust`: optional known-divergence output. If omitted, Rust must match Python exactly.
- `note`: optional explanation for known divergences.

## Running

Point the test at a Python SQLGlot checkout:

```bash
SQLGLOT_PYTHON_PATH=/path/to/sqlglot cargo test sqlglot_python_smoke_parity --features cli -- --nocapture
```

If `SQLGLOT_PYTHON_PATH` is not set, the test also checks for a sibling checkout at `../sqlglot`.

## Ratchet

The intended workflow is:

1. Add or import a failing SQLGlot case.
2. Mark it with `accepted_rust` only when the divergence is intentional and documented.
3. Fix sqlgrok.
4. Remove `accepted_rust` once exact parity is reached.
5. Add a narrow Rust regression test for the fixed gap.
