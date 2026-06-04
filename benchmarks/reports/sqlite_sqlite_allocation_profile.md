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
- Allocated: `4.83 KiB/op` across `56.25` allocations/op
- Total allocated: `37.71 MiB`
- Net bytes after drops: `0`

## Notes

- This is allocation accounting, not wall-clock timing. Pair it with `bench-sqlglot` and Criterion phase benches.
- Counts include the output `String`, because normal callers also receive that allocation.
- The counting allocator lives only in this helper binary, so normal `xtask bench-sqlglot` timing is not perturbed.

## Scoped Allocation Breakdown

| scope | KiB/op | allocations/op |
| --- | ---: | ---: |
| `parse` | 4.08 | 42.12 |
| `transform` | 0.48 | 6.75 |
| `generate` | 0.27 | 7.38 |

## Per-Case Breakdown

| id | KiB/op | allocations/op | net bytes/op | tags |
| --- | ---: | ---: | ---: | --- |
| `sqlite-cte` | 10.57 | 92.00 | 0.00 | `cte,orm` |
| `sqlite-simple-select` | 6.59 | 70.00 | 0.00 | `select,orm` |
| `sqlite-window` | 4.39 | 68.00 | 0.00 | `window,orm` |
| `sqlite-create-table` | 4.39 | 50.00 | 0.00 | `ddl,migration` |
| `sqlite-alter-table` | 3.72 | 30.00 | 0.00 | `ddl,migration` |
| `sqlite-insert-or-ignore` | 3.36 | 23.00 | 0.00 | `ddl,orm` |
| `sqlite-json-type` | 2.89 | 63.00 | 0.00 | `json,orm` |
| `sqlite-count-distinct` | 2.70 | 54.00 | 0.00 | `aggregate,orm` |

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
