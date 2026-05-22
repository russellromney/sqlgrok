# SQLGlot Import Report

Source: `parity/reports/transpile_postgres_sqlite.jsonl`

Total candidates: `682`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 263 |
| `mismatch` | 307 |
| `rust-error` | 112 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `mismatch` | `SELECT` | 88 |
| `rust-error` | `SELECT` | 76 |
| `match` | `SELECT` | 71 |
| `mismatch` | `CREATE` | 63 |
| `mismatch` | `CREATE TABLE` | 36 |
| `match` | `CREATE TABLE` | 20 |
| `match` | `REVOKE` | 19 |
| `match` | `CAST()` | 17 |
| `match` | `GRANT` | 16 |
| `mismatch` | `X` | 13 |
| `mismatch` | `BEGIN` | 11 |
| `match` | `INTERVAL` | 9 |
| `match` | `WITH` | 9 |
| `mismatch` | `CREATE INDEX` | 9 |
| `mismatch` | `ALTER TABLE` | 8 |
| `match` | `ROUND()` | 7 |
| `mismatch` | `TRUNCATE` | 7 |
| `rust-error` | `X` | 6 |
| `match` | `MERGE` | 5 |
| `mismatch` | `WITH` | 5 |
| `rust-error` | `INSERT` | 5 |
| `match` | `ANALYZE` | 4 |
| `match` | `DROP INDEX` | 4 |
| `match` | `INSERT` | 4 |
| `match` | `REGEXP_INSTR()` | 4 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 102 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 89 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 88 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres` | 72 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres_create_trigger` | 34 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 29 |
| `match` | `tests/dialects/test_postgres.py` | `test_revoke` | 19 |
| `match` | `tests/dialects/test_postgres.py` | `test_grant` | 16 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_begin_transaction` | 11 |
| `match` | `tests/dialects/test_postgres.py` | `test_interval_span` | 10 |
| `match` | `tests/dialects/test_postgres.py` | `test_round` | 7 |
| `mismatch` | `tests/dialects/test_dune.py` | `test_dune` | 7 |
| `match` | `tests/dialects/test_doris.py` | `test_doris` | 6 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 6 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_xmlelement` | 6 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 5 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 5 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_ddl` | 5 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_regexp_instr` | 4 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_analyze` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_json_extract` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_locks` | 4 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_heredoc_strings` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-0709-test-bigquery`: `SELECT MAKE_TIME(15, 30, 00)`
  - expected: `SELECT TIME_FROM_PARTS(15, 30, 00)`
  - actual: `SELECT MAKE_TIME(15, 30, 00)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0131-test-clickhouse`: `TRUNC(3.14159, 2)`
  - expected: `TRUNC(3.14159)`
  - actual: `TRUNC(3.14159, 2)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0309-test-clickhouse`: `SELECT TO_DATE('05 12 2000', 'DD MM YYYY')`
  - expected: `SELECT STR_TO_DATE('05 12 2000', '%d %m %Y')`
  - actual: `SELECT TO_DATE('05 12 2000', 'DD MM YYYY')`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0348-test-clickhouse`: `x = any(array[1])`
  - expected: `x = ANY(ARRAY(1))`
  - actual: `x = ANY(ARRAY[1])`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0372-test-clickhouse`: `SELECT TIMESTAMP '2020-01-01' + INTERVAL '500 us'`
  - expected: `SELECT CAST('2020-01-01' AS TIMESTAMP) + INTERVAL '500' MICROSECOND`
  - actual: `SELECT CAST('2020-01-01' AS TIMESTAMP) + INTERVAL '500 us'`

### `rust-error`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-1576-test-bigquery`: `SELECT * FROM (VALUES (1)) AS t1`
  - expected: `SELECT * FROM (VALUES (1)) AS t1`
  - error: `Parser error: Expected identifier, got LParen ('(') at line 1 col 15`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-1583-test-bigquery`: `SELECT * FROM (VALUES (1)) AS t1(id) CROSS JOIN (VALUES (1)) AS t2(id)`
  - expected: `SELECT * FROM (VALUES (1)) AS t1 CROSS JOIN (VALUES (1)) AS t2`
  - error: `Parser error: Expected identifier, got LParen ('(') at line 1 col 15`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0354-test-clickhouse`: `any(array[1]) <> x`
  - expected: `ANY(ARRAY(1)) <> x`
  - error: `Unexpected token: Token { token_type: Any, value: "any", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-dialect-2248-test-operators`: `POSITION(needle in haystack)`
  - expected: `INSTR(haystack, needle)`
  - error: `Parser error: Expected LParen, got Identifier ('haystack') at line 1 col 20`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-dialect-2951-test-nullsafe-eq`: `SELECT a IS NOT DISTINCT FROM b`
  - expected: `SELECT a IS NOT DISTINCT FROM b`
  - error: `Parser error: Expected Null, got Distinct ('DISTINCT') at line 1 col 17`

