# SQLGlot Import Report

Source: `parity/reports/transpile_sqlite_sqlite.jsonl`

Total candidates: `144`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 82 |
| `mismatch` | 62 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 35 |
| `mismatch` | `SELECT` | 30 |
| `mismatch` | `PRAGMA` | 12 |
| `match` | `CREATE` | 9 |
| `mismatch` | `CREATE TABLE` | 7 |
| `match` | `INSERT` | 5 |
| `match` | `X` | 5 |
| `match` | `CREATE TABLE` | 4 |
| `mismatch` | `ATTACH` | 4 |
| `match` | `DATEDIFF()` | 3 |
| `match` | `ANALYZE` | 2 |
| `match` | `LOG()` | 2 |
| `mismatch` | `MIN()` | 2 |
| `match` | `ALTER TABLE` | 1 |
| `match` | `CREATE INDEX` | 1 |
| `match` | `CURRENT_DATE` | 1 |
| `match` | `CURRENT_TIME` | 1 |
| `match` | `CURRENT_TIMESTAMP` | 1 |
| `match` | `EDITDIST3()` | 1 |
| `match` | `HEX()` | 1 |
| `match` | `INSTR()` | 1 |
| `match` | `LOWER()` | 1 |
| `match` | `MIN()` | 1 |
| `match` | `RANDOM()` | 1 |
| `match` | `REPLACE` | 1 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 32 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 28 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_ddl` | 17 |
| `match` | `tests/dialects/test_sqlite.py` | `test_ddl` | 14 |
| `mismatch` | `tests/dialects/test_hive.py` | `test_joins_without_on` | 6 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 4 |
| `match` | `tests/dialects/test_sqlite.py` | `test_strftime` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_limit` | 3 |
| `match` | `tests/dialects/test_sqlite.py` | `test_create_trigger` | 3 |
| `match` | `tests/dialects/test_sqlite.py` | `test_datediff` | 3 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 2 |
| `match` | `tests/dialects/test_dialect.py` | `test_escaped_identifier_delimiter` | 2 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 2 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 2 |
| `match` | `tests/dialects/test_sqlite.py` | `test_analyze` | 2 |
| `match` | `tests/dialects/test_teradata.py` | `test_time` | 2 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 2 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_logarithm` | 2 |
| `match` | `tests/dialects/test_dialect.py` | `test_random` | 1 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_hexadecimal_literal` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_trunc` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_window_null_treatment` | 1 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 1 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_alias` | 1 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-sqlite-to-sqlite-tests-dialects-test-bigquery-0884-test-bigquery`: `MIN(x, y)`
  - expected: `MIN(x, y)`
  - actual: `MIN(x)`
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-dialect-2881-test-alias`: `SELECT 1 'foo'`
  - expected: `SELECT 1 AS "foo"`
  - actual: `SELECT 1`
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-dialect-3132-test-logarithm`: `LOG2(a)`
  - expected: `LOG(2, a)`
  - actual: `LOG2(a)`
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-dialect-3132-test-logarithm-2`: `LOG10(a)`
  - expected: `LOG(10, a)`
  - actual: `LOG10(a)`
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-dialect-3308-test-window-exclude`: `SELECT SUM(X) OVER (PARTITION BY x RANGE BETWEEN 1 PRECEDING AND CURRENT ROW EXCLUDE NO OTHERS)`
  - expected: `SELECT SUM(X) OVER (PARTITION BY x RANGE BETWEEN 1 PRECEDING AND CURRENT ROW EXCLUDE NO OTHERS)`
  - actual: `SELECT SUM(X) OVER (PARTITION BY x RANGE BETWEEN 1 PRECEDING AND CURRENT ROW)`

