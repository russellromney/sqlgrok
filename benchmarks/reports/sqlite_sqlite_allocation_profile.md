# sqlgrok Allocation Profile

Counts allocations in a dedicated helper binary while repeatedly measuring the `transpile` phase.

## Summary

- Case file: `benchmarks/cases/sqlite_sqlite.jsonl`
- Phase: `transpile`
- Cases: `8`
- Iterations per case: `1000`
- Warmup iterations per case: `100`
- Operations: `8000`
- Output checksum: `590000`
- Allocated: `6.42 KiB/op` across `86.38` allocations/op
- Total allocated: `50.13 MiB`
- Net bytes after drops: `0`

## Notes

- This is allocation accounting, not wall-clock timing. Pair it with `bench-sqlglot` and Criterion phase benches.
- Counts include the output `String`, because normal callers also receive that allocation.
- The counting allocator lives only in this helper binary, so normal `xtask bench-sqlglot` timing is not perturbed.

## Per-Case Breakdown

| id | KiB/op | allocations/op | net bytes/op | tags |
| --- | ---: | ---: | ---: | --- |
| `sqlite-cte` | 14.99 | 147.00 | 0.00 | `cte,orm` |
| `sqlite-simple-select` | 7.74 | 108.00 | 0.00 | `select,orm` |
| `sqlite-create-table` | 6.80 | 102.00 | 0.00 | `ddl,migration` |
| `sqlite-window` | 5.82 | 92.00 | 0.00 | `window,orm` |
| `sqlite-alter-table` | 4.83 | 51.00 | 0.00 | `ddl,migration` |
| `sqlite-json-type` | 3.76 | 83.00 | 0.00 | `json,orm` |
| `sqlite-insert-or-ignore` | 3.70 | 34.00 | 0.00 | `ddl,orm` |
| `sqlite-count-distinct` | 3.69 | 74.00 | 0.00 | `aggregate,orm` |

## Workload

| id | read | write | tags | SQL |
| --- | --- | --- | --- | --- |
| `sqlite-simple-select` | `sqlite` | `sqlite` | `select,orm` | `SELECT a, b FROM t WHERE a > 10 ORDER BY b DESC LIMIT 10 OFFSET 5` |
| `sqlite-json-type` | `sqlite` | `sqlite` | `json,orm` | `SELECT JSON_TYPE(data, '$.k') FROM events` |
| `sqlite-create-table` | `sqlite` | `sqlite` | `ddl,migration` | `CREATE TABLE users (id INTEGER PRIMARY KEY, email TEXT NOT NULL UNIQUE, created_at TEXT DEFAULT CURRENT_TIMESTAMP)` |
| `sqlite-insert-or-ignore` | `sqlite` | `sqlite` | `ddl,orm` | `INSERT OR IGNORE INTO users (id, email) VALUES (1, 'a@example.com')` |
| `sqlite-window` | `sqlite` | `sqlite` | `window,orm` | `SELECT user_id, ROW_NUMBER() OVER (PARTITION BY account_id ORDER BY created_at) FROM events` |
| `sqlite-cte` | `sqlite` | `sqlite` | `cte,orm` | `WITH a AS (SELECT 1 AS x), b AS (SELECT 2 AS y) SELECT a.x + b.y FROM a, b` |
| `sqlite-count-distinct` | `sqlite` | `sqlite` | `aggregate,orm` | `SELECT COUNT(DISTINCT name) FROM users GROUP BY account_id` |
| `sqlite-alter-table` | `sqlite` | `sqlite` | `ddl,migration` | `ALTER TABLE users ADD COLUMN created_at TEXT DEFAULT CURRENT_TIMESTAMP` |
