# Cinch SQLite Correctness

sqlgrok has two validation lanes:

1. SQLGlot parity is the default contract. Normal transpilation should match Python SQLGlot exactly.
2. Cinch correctness checks run Python SQLGlot's SQLite-targeted output against stock SQLite and document cases where the oracle output is not executable or not sufficient for MySQL/Postgres-over-SQLite work.

A cinch correctness failure is not automatically a sqlgrok transpiler change. It is evidence for one of three follow-ups:

- upstream a SQLGlot SQLite rewrite;
- add an explicit opt-in sqlgrok/cinch compatibility mode after discussion;
- handle the behavior in the downstream execution layer when it is not a transpiler concern.

Do not change default sqlgrok output away from Python SQLGlot just to make a cinch correctness case pass.

## Running

```bash
cargo run --bin xtask -- check-sqlite-correctness \
  --sqlglot /Users/russellromney/Documents/Github/sqlglot \
  --input correctness/cases/cinch_sqlite.jsonl \
  --jsonl-output correctness/reports/cinch_sqlite.jsonl \
  --markdown-output correctness/reports/cinch_sqlite.md
```

The input corpus is JSONL. Each case supplies source SQL, the source dialect, optional setup SQL, tags, and notes. The checker asks Python SQLGlot for `write="sqlite"`, then executes that SQLite string with `sqlite3 :memory:`.

`sqlite-error` means SQLGlot produced SQLite-targeted SQL that stock SQLite rejected. Those are useful upstream/cinch candidates, not parity failures.
