# SQLGlot Import Report

Source: `parity/reports/transpile_postgres_sqlite.jsonl`

Total candidates: `682`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 118 |
| `mismatch` | 125 |
| `rust-error` | 439 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `mismatch` | `SELECT` | 86 |
| `rust-error` | `SELECT` | 79 |
| `match` | `SELECT` | 70 |
| `rust-error` | `CREATE` | 66 |
| `rust-error` | `CREATE TABLE` | 34 |
| `rust-error` | `CAST()` | 23 |
| `rust-error` | `X` | 22 |
| `rust-error` | `REVOKE` | 20 |
| `match` | `CREATE TABLE` | 18 |
| `rust-error` | `GRANT` | 17 |
| `mismatch` | `BEGIN` | 11 |
| `match` | `WITH` | 9 |
| `rust-error` | `ALTER TABLE` | 9 |
| `rust-error` | `INTERVAL` | 9 |
| `mismatch` | `TRUNCATE` | 7 |
| `rust-error` | `A` | 7 |
| `rust-error` | `ROUND()` | 7 |
| `mismatch` | `CREATE INDEX` | 6 |
| `match` | `MERGE` | 5 |
| `mismatch` | `WITH` | 5 |
| `rust-error` | `INSERT` | 5 |
| `match` | `DROP INDEX` | 4 |
| `match` | `INSERT` | 4 |
| `mismatch` | `CREATE TABLE` | 4 |
| `rust-error` | `ANALYZE` | 4 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres` | 165 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_ddl` | 79 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 52 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 46 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres_create_trigger` | 34 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 23 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 20 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_revoke` | 20 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_grant` | 17 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_begin_transaction` | 11 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_interval_span` | 10 |
| `mismatch` | `tests/dialects/test_dune.py` | `test_dune` | 7 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_round` | 7 |
| `match` | `tests/dialects/test_doris.py` | `test_doris` | 6 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_xmlelement` | 6 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 5 |
| `rust-error` | `tests/dialects/test_dialect.py` | `test_operators` | 5 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_json_extract` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_locks` | 4 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 4 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_heredoc_strings` | 4 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 4 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 4 |
| `rust-error` | `tests/dialects/test_dialect.py` | `test_logarithm` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-0709-test-bigquery`: `SELECT MAKE_TIME(15, 30, 00)`
  - expected: `SELECT TIME_FROM_PARTS(15, 30, 00)`
  - actual: `SELECT MAKE_TIME(15, 30, 00)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0309-test-clickhouse`: `SELECT TO_DATE('05 12 2000', 'DD MM YYYY')`
  - expected: `SELECT STR_TO_DATE('05 12 2000', '%d %m %Y')`
  - actual: `SELECT TO_DATE('05 12 2000', 'DD MM YYYY')`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0372-test-clickhouse`: `SELECT TIMESTAMP '2020-01-01' + INTERVAL '500 us'`
  - expected: `SELECT CAST('2020-01-01' AS TIMESTAMP) + INTERVAL '500' MICROSECOND`
  - actual: `SELECT TIMESTAMP`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0398-test-clickhouse`: `SELECT 'ThOmAs' ~* 'thomas'`
  - expected: `SELECT REGEXP_I_LIKE('ThOmAs', 'thomas')`
  - actual: `SELECT 'ThOmAs'`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0404-test-clickhouse`: `SELECT 'ThOmAs' ~* x FROM t`
  - expected: `SELECT REGEXP_I_LIKE('ThOmAs', x) FROM t`
  - actual: `SELECT 'ThOmAs'`

### `rust-error`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-1070-test-bigquery`: `SHA256(x)`
  - expected: `SHA2(x, 256)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "SHA256", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-1576-test-bigquery`: `SELECT * FROM (VALUES (1)) AS t1`
  - expected: `SELECT * FROM (VALUES (1)) AS t1`
  - error: `Parser error: Expected identifier, got LParen ('(') at line 1 col 15`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-1583-test-bigquery`: `SELECT * FROM (VALUES (1)) AS t1(id) CROSS JOIN (VALUES (1)) AS t2(id)`
  - expected: `SELECT * FROM (VALUES (1)) AS t1 CROSS JOIN (VALUES (1)) AS t2`
  - error: `Parser error: Expected identifier, got LParen ('(') at line 1 col 15`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-3558-test-bit-aggs`: `BIT_AND(x)`
  - expected: `BITWISE_AND_AGG(x)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "BIT_AND", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-3578-test-bit-aggs`: `BIT_OR(x)`
  - expected: `BITWISE_OR_AGG(x)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "BIT_OR", line: 1, col: 1, position: 0, quote_char: '\0' }`

