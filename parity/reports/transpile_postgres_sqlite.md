# SQLGlot Import Report

Source: `parity/reports/transpile_postgres_sqlite.jsonl`

Total candidates: `686`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 435 |
| `mismatch` | 251 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 177 |
| `mismatch` | `CREATE` | 62 |
| `mismatch` | `SELECT` | 62 |
| `mismatch` | `CREATE TABLE` | 35 |
| `match` | `CREATE TABLE` | 21 |
| `match` | `CAST()` | 20 |
| `match` | `REVOKE` | 19 |
| `match` | `X` | 17 |
| `match` | `GRANT` | 16 |
| `match` | `WITH` | 12 |
| `mismatch` | `BEGIN` | 11 |
| `match` | `INSERT` | 9 |
| `match` | `INTERVAL` | 9 |
| `mismatch` | `CREATE INDEX` | 9 |
| `mismatch` | `ALTER TABLE` | 8 |
| `match` | `ROUND()` | 7 |
| `mismatch` | `TRUNCATE` | 7 |
| `match` | `MERGE` | 6 |
| `match` | `UPDATE` | 5 |
| `mismatch` | `WITH` | 5 |
| `mismatch` | `X` | 5 |
| `match` | `A` | 4 |
| `match` | `ANALYZE` | 4 |
| `match` | `CREATE` | 4 |
| `match` | `DROP INDEX` | 4 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 193 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 86 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 74 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 36 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres_create_trigger` | 34 |
| `match` | `tests/dialects/test_postgres.py` | `test_revoke` | 19 |
| `match` | `tests/dialects/test_postgres.py` | `test_grant` | 16 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_begin_transaction` | 11 |
| `match` | `tests/dialects/test_postgres.py` | `test_interval_span` | 10 |
| `match` | `tests/dialects/test_postgres.py` | `test_round` | 7 |
| `mismatch` | `tests/dialects/test_dune.py` | `test_dune` | 7 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 6 |
| `match` | `tests/dialects/test_doris.py` | `test_doris` | 6 |
| `match` | `tests/dialects/test_postgres.py` | `test_xmlelement` | 6 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 5 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 5 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 5 |
| `match` | `tests/dialects/test_dialect.py` | `test_heredoc_strings` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_regexp_instr` | 4 |
| `match` | `tests/dialects/test_doris.py` | `test_table_alias_conversion` | 4 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 4 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0131-test-clickhouse`: `TRUNC(3.14159, 2)`
  - expected: `TRUNC(3.14159)`
  - actual: `TRUNC(3.14159, 2)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0348-test-clickhouse`: `x = any(array[1])`
  - expected: `x = ANY(ARRAY(1))`
  - actual: `x = ANY(ARRAY[1])`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0354-test-clickhouse`: `any(array[1]) <> x`
  - expected: `ANY(ARRAY(1)) <> x`
  - actual: `any(ARRAY(1)) <> x`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0372-test-clickhouse`: `SELECT TIMESTAMP '2020-01-01' + INTERVAL '500 us'`
  - expected: `SELECT CAST('2020-01-01' AS TIMESTAMP) + INTERVAL '500' MICROSECOND`
  - actual: `SELECT CAST('2020-01-01' AS TIMESTAMP) + INTERVAL '500 us'`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-dialect-1490-test-array`: `ARRAY_PREPEND(x, arr)`
  - expected: `ARRAY_PREPEND(arr, x)`
  - actual: `ARRAY_PREPEND(x, arr)`

