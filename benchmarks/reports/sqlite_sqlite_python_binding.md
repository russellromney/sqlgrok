# SQLGlot Performance Comparison

Compares Python SQLGlot against a parity-clean sqlgrok execution path.

Run with a release build for meaningful numbers:

```bash
cargo run --release --bin xtask -- bench-sqlglot --mode python-binding --sqlglot /path/to/sqlglot
```

## Summary

- SQLGlot checkout: `/Users/russellromney/Documents/Github/sqlglot`
- Case file: `benchmarks/cases/sqlite_sqlite.jsonl`
- Mode: `python-binding`
- Baseline: `Python SQLGlot`
- Candidate: `sqlgrok PyO3 single-call`
- Cases: `8`
- Iterations per case: `1000`
- Warmup iterations per case: `100`
- Samples: `5`
- Total measured operations: `8000`
- Python SQLGlot median: `384.750 us/op` (p95 `619.147`)
- sqlgrok PyO3 single-call median: `9.727 us/op` (p95 `17.296`)
- Median speedup: `39.55x`
- Output checksum: `590000`

## Distribution

| runner | min us/op | mean us/op | median us/op | p95 us/op | max us/op |
| --- | ---: | ---: | ---: | ---: | ---: |
| Python SQLGlot | 205.959 | 379.395 | 384.750 | 619.147 | 619.147 |
| sqlgrok PyO3 single-call | 6.434 | 10.992 | 9.727 | 17.296 | 17.296 |

## Samples

| sample | Python SQLGlot us/op | candidate us/op | speedup |
| ---: | ---: | ---: | ---: |
| 1 | 205.959 | 9.727 | 21.17x |
| 2 | 619.147 | 6.434 | 96.23x |
| 3 | 384.750 | 12.905 | 29.81x |
| 4 | 434.645 | 17.296 | 25.13x |
| 5 | 252.475 | 8.596 | 29.37x |

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
