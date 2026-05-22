# SQLGlot Import Report

Source: `parity/reports/transpile_postgres_sqlite.jsonl`

Total candidates: `682`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 190 |
| `mismatch` | 226 |
| `rust-error` | 266 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `mismatch` | `SELECT` | 88 |
| `rust-error` | `SELECT` | 76 |
| `match` | `SELECT` | 71 |
| `rust-error` | `CREATE` | 66 |
| `rust-error` | `CREATE TABLE` | 34 |
| `rust-error` | `REVOKE` | 20 |
| `match` | `CREATE TABLE` | 18 |
| `rust-error` | `GRANT` | 17 |
| `mismatch` | `X` | 16 |
| `match` | `CAST()` | 14 |
| `mismatch` | `BEGIN` | 11 |
| `match` | `INTERVAL` | 9 |
| `match` | `WITH` | 9 |
| `rust-error` | `ALTER TABLE` | 9 |
| `mismatch` | `CAST()` | 7 |
| `mismatch` | `TRUNCATE` | 7 |
| `mismatch` | `CREATE INDEX` | 6 |
| `rust-error` | `X` | 6 |
| `match` | `MERGE` | 5 |
| `mismatch` | `ROUND()` | 5 |
| `mismatch` | `WITH` | 5 |
| `rust-error` | `INSERT` | 5 |
| `match` | `DROP INDEX` | 4 |
| `match` | `INSERT` | 4 |
| `match` | `REGEXP_INSTR()` | 4 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 112 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_ddl` | 79 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 77 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres` | 74 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres_create_trigger` | 34 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 23 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 20 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_revoke` | 20 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_grant` | 17 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_begin_transaction` | 11 |
| `match` | `tests/dialects/test_postgres.py` | `test_interval_span` | 10 |
| `mismatch` | `tests/dialects/test_dune.py` | `test_dune` | 7 |
| `match` | `tests/dialects/test_doris.py` | `test_doris` | 6 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 6 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_xmlelement` | 6 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 5 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_round` | 5 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 5 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_regexp_instr` | 4 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_json_extract` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_locks` | 4 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_heredoc_strings` | 4 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-3558-test-bit-aggs`: `BIT_AND(x)`
  - expected: `BITWISE_AND_AGG(x)`
  - actual: `BIT_AND(x)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-3578-test-bit-aggs`: `BIT_OR(x)`
  - expected: `BITWISE_OR_AGG(x)`
  - actual: `BIT_OR(x)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-3598-test-bit-aggs`: `BIT_XOR(x)`
  - expected: `BITWISE_XOR_AGG(x)`
  - actual: `BIT_XOR(x)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-0709-test-bigquery`: `SELECT MAKE_TIME(15, 30, 00)`
  - expected: `SELECT TIME_FROM_PARTS(15, 30, 00)`
  - actual: `SELECT MAKE_TIME(15, 30, 00)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0131-test-clickhouse`: `TRUNC(3.14159, 2)`
  - expected: `TRUNC(3.14159)`
  - actual: `TRUNC(3.14159, 2)`

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

