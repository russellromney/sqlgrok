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

- Added [docs/ROADMAP.md](docs/ROADMAP.md) with executable parity milestones and implementation sessions.
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
