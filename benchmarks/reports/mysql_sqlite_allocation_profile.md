# sqlgrok Allocation Profile

Counts allocations in a dedicated helper binary while repeatedly measuring the `transpile` phase.

## Summary

- Case file: `benchmarks/cases/mysql_sqlite.jsonl`
- Phase: `transpile`
- Cases: `8`
- Iterations per case: `1000`
- Warmup iterations per case: `100`
- Operations: `8000`
- Output checksum: `586000`
- Allocated: `6.57 KiB/op` across `88.38` allocations/op
- Total allocated: `51.36 MiB`
- Net bytes after drops: `0`

## Notes

- This is allocation accounting, not wall-clock timing. Pair it with `bench-sqlglot` and Criterion phase benches.
- Counts include the output `String`, because normal callers also receive that allocation.
- The counting allocator lives only in this helper binary, so normal `xtask bench-sqlglot` timing is not perturbed.

## Per-Case Breakdown

| id | KiB/op | allocations/op | net bytes/op | tags |
| --- | ---: | ---: | ---: | --- |
| `mysql-if-cast-div` | 12.86 | 139.00 | 0.00 | `expression,function` |
| `mysql-computed-column` | 9.83 | 89.00 | 0.00 | `ddl,migration` |
| `mysql-limit-offset-comma` | 7.15 | 83.00 | 0.00 | `limit,orm` |
| `mysql-json-extract-key` | 6.38 | 85.00 | 0.00 | `json,orm` |
| `mysql-on-duplicate-key` | 5.48 | 107.00 | 0.00 | `ddl,orm` |
| `mysql-date-format` | 4.15 | 76.00 | 0.00 | `datetime,function` |
| `mysql-group-concat-order-separator` | 3.86 | 80.00 | 0.00 | `aggregate,orm` |
| `mysql-insert-ignore` | 2.88 | 48.00 | 0.00 | `ddl,orm` |

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
