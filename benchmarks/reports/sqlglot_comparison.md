# SQLGlot Performance Comparison

Compares sqlgrok's in-process Rust transpiler against Python SQLGlot on parity-clean MySQL/Postgres to SQLite cases.

Run with a release build for meaningful numbers:

```bash
cargo run --release --bin xtask -- bench-sqlglot --sqlglot /path/to/sqlglot
```

## Summary

- SQLGlot checkout: `/Users/russellromney/Documents/Github/sqlglot`
- Cases: `10`
- Iterations per case: `500`
- Warmup iterations per case: `50`
- Total measured operations: `5000`
- Python SQLGlot total: `2678.308 ms` (`535.662 us/op`)
- sqlgrok total: `30.932 ms` (`6.186 us/op`)
- Speedup: `86.59x`
- Output checksum: `460500`

## Workload

| id | read | write | feature | SQL |
| --- | --- | --- | --- | --- |
| `mysql-group-concat` | `mysql` | `sqlite` | `aggregate` | `SELECT GROUP_CONCAT(v ORDER BY v SEPARATOR '\|') FROM gc` |
| `mysql-json-extract` | `mysql` | `sqlite` | `json` | `SELECT JSON_EXTRACT(data, '$.k') FROM events WHERE id = 1` |
| `mysql-limit-offset` | `mysql` | `sqlite` | `limit` | `SELECT a, b FROM t WHERE a > 10 ORDER BY b DESC LIMIT 5, 10` |
| `mysql-date-format` | `mysql` | `sqlite` | `datetime` | `SELECT DATE_FORMAT(created_at, '%Y-%m-%d') FROM users` |
| `mysql-if-cast-division` | `mysql` | `sqlite` | `expression` | `SELECT IF(a > 0, a, 7 DIV 2), x / y FROM metrics` |
| `postgres-distinct-on` | `postgres` | `sqlite` | `rewrite` | `SELECT DISTINCT ON (account_id) account_id, created_at FROM events ORDER BY account_id, created_at DESC` |
| `postgres-json-path` | `postgres` | `sqlite` | `json` | `SELECT payload #>> '{customer,0,name}' FROM events WHERE payload ->> 'kind' = 'signup'` |
| `postgres-extract-cast` | `postgres` | `sqlite` | `datetime` | `SELECT EXTRACT(YEAR FROM CAST(created_at AS DATE)), DATE_TRUNC('month', created_at) FROM events` |
| `postgres-rollup` | `postgres` | `sqlite` | `grouping` | `SELECT region, product, SUM(revenue) FROM sales GROUP BY ROLLUP(region, product)` |
| `postgres-window-nulls` | `postgres` | `sqlite` | `window` | `SELECT user_id, ROW_NUMBER() OVER (PARTITION BY account_id ORDER BY created_at) FROM events` |
