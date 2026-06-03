# SQLGlot Performance Comparison

Compares Python SQLGlot against a parity-clean sqlgrok execution path.

Run with a release build for meaningful numbers:

```bash
cargo run --release --bin xtask -- bench-sqlglot --mode python-binding --sqlglot /path/to/sqlglot
```

## Summary

- SQLGlot checkout: `/Users/russellromney/Documents/Github/sqlglot`
- Case file: `benchmarks/cases/mysql_sqlite.jsonl`
- Mode: `python-binding`
- Baseline: `Python SQLGlot`
- Candidate: `sqlgrok PyO3 single-call`
- Cases: `8`
- Iterations per case: `1000`
- Warmup iterations per case: `100`
- Samples: `5`
- Total measured operations: `8000`
- Python SQLGlot median: `359.930 us/op` (p95 `463.848`)
- sqlgrok PyO3 single-call median: `9.713 us/op` (p95 `12.729`)
- Median speedup: `37.06x`
- Output checksum: `586000`

## Distribution

| runner | min us/op | mean us/op | median us/op | p95 us/op | max us/op |
| --- | ---: | ---: | ---: | ---: | ---: |
| Python SQLGlot | 249.900 | 358.865 | 359.930 | 463.848 | 463.848 |
| sqlgrok PyO3 single-call | 8.210 | 10.286 | 9.713 | 12.729 | 12.729 |

## Samples

| sample | Python SQLGlot us/op | candidate us/op | speedup |
| ---: | ---: | ---: | ---: |
| 1 | 463.848 | 8.210 | 56.50x |
| 2 | 373.075 | 12.729 | 29.31x |
| 3 | 347.570 | 9.613 | 36.16x |
| 4 | 249.900 | 11.165 | 22.38x |
| 5 | 359.930 | 9.713 | 37.06x |

## Workload

| id | read | write | tags | SQL |
| --- | --- | --- | --- | --- |
| `mysql-group-concat-order-separator` | `mysql` | `sqlite` | `aggregate,orm` | `SELECT GROUP_CONCAT(v ORDER BY v SEPARATOR '\|') FROM gc` |
| `mysql-json-extract-key` | `mysql` | `sqlite` | `json,orm` | `SELECT JSON_EXTRACT(data, '$.k') FROM events WHERE id = 1` |
| `mysql-limit-offset-comma` | `mysql` | `sqlite` | `limit,orm` | `SELECT a, b FROM t WHERE a > 10 ORDER BY b DESC LIMIT 5, 10` |
| `mysql-date-format` | `mysql` | `sqlite` | `datetime,function` | `SELECT DATE_FORMAT(created_at, '%Y-%m-%d') FROM users` |
| `mysql-if-cast-div` | `mysql` | `sqlite` | `expression,function` | `SELECT IF(a > 0, CAST(a AS SIGNED INTEGER), 7 DIV 2), x / y FROM metrics` |
| `mysql-insert-ignore` | `mysql` | `sqlite` | `ddl,orm` | `INSERT IGNORE INTO users (id, email) VALUES (1, 'a@example.com')` |
| `mysql-on-duplicate-key` | `mysql` | `sqlite` | `ddl,orm` | `INSERT INTO users (id, email) VALUES (1, 'a@example.com') ON DUPLICATE KEY UPDATE email = VALUES(email)` |
| `mysql-computed-column` | `mysql` | `sqlite` | `ddl,migration` | `CREATE TABLE users (id INT PRIMARY KEY, email VARCHAR(255), email_lc VARCHAR(255) AS (LOWER(email)) STORED)` |
