# SQLGlot Import Report

Source: `parity/reports/transpile_sqlite_sqlite.jsonl`

Total candidates: `144`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 76 |
| `mismatch` | 60 |
| `rust-error` | 8 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 32 |
| `mismatch` | `SELECT` | 25 |
| `mismatch` | `PRAGMA` | 12 |
| `match` | `CREATE` | 9 |
| `rust-error` | `SELECT` | 8 |
| `mismatch` | `CREATE TABLE` | 7 |
| `match` | `INSERT` | 5 |
| `match` | `X` | 5 |
| `match` | `CREATE TABLE` | 4 |
| `mismatch` | `ATTACH` | 4 |
| `mismatch` | `DATEDIFF()` | 3 |
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

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 30 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 24 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_ddl` | 17 |
| `match` | `tests/dialects/test_sqlite.py` | `test_ddl` | 14 |
| `mismatch` | `tests/dialects/test_hive.py` | `test_joins_without_on` | 6 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 6 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 4 |
| `match` | `tests/dialects/test_sqlite.py` | `test_strftime` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_limit` | 3 |
| `match` | `tests/dialects/test_sqlite.py` | `test_create_trigger` | 3 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_datediff` | 3 |
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
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-hive-1047-test-joins-without-on`: `SELECT * FROM t1 FULL OUTER JOIN t2`
  - expected: `SELECT * FROM t1 FULL OUTER JOIN t2 ON TRUE`
  - actual: `SELECT * FROM t1 FULL JOIN t2`

### `rust-error`

- `sqlglot-sqlite-to-sqlite-tests-dialects-test-dialect-3308-test-window-exclude`: `SELECT SUM(X) OVER (PARTITION BY x RANGE BETWEEN 1 PRECEDING AND CURRENT ROW EXCLUDE NO OTHERS)`
  - expected: `SELECT SUM(X) OVER (PARTITION BY x RANGE BETWEEN 1 PRECEDING AND CURRENT ROW EXCLUDE NO OTHERS)`
  - error: `Parser error: Expected RParen, got Identifier ('EXCLUDE') at line 1 col 78`
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-sqlite-0170-test-sqlite`: `SELECT * FROM station WHERE city IS NOT ''`
  - expected: `SELECT * FROM station WHERE NOT city IS ''`
  - error: `Parser error: Expected Null, got String ('') at line 1 col 41`
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-sqlite-0196-test-sqlite`: `SELECT * FROM t WHERE NULL IS y`
  - expected: `SELECT * FROM t WHERE NULL IS y`
  - error: `Parser error: Expected Null, got Identifier ('y') at line 1 col 31`
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-sqlite-0197-test-sqlite`: `SELECT * FROM t WHERE NULL IS NOT y`
  - expected: `SELECT * FROM t WHERE NOT NULL IS y`
  - error: `Parser error: Expected Null, got Identifier ('y') at line 1 col 35`
- `sqlglot-sqlite-to-sqlite-tests-dialects-test-sqlite-0240-test-hexadecimal-literal`: `SELECT 0XCC`
  - expected: `SELECT x'CC'`
  - error: `Unexpected token: Token { token_type: HexString, value: "0XCC", line: 1, col: 8, position: 7, quote_char: '\0' }`

