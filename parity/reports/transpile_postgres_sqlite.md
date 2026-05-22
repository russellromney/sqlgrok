# SQLGlot Import Report

Source: `parity/reports/transpile_postgres_sqlite.jsonl`

Total candidates: `21`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 7 |
| `mismatch` | 1 |
| `oracle-error` | 1 |
| `rust-error` | 12 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 5 |
| `rust-error` | `CREATE TABLE` | 3 |
| `rust-error` | `DATEDIFF()` | 3 |
| `rust-error` | `SELECT` | 3 |
| `match` | `CREATE TABLE` | 2 |
| `mismatch` | `SELECT` | 1 |
| `oracle-error` | `SELECT` | 1 |
| `rust-error` | `1` | 1 |
| `rust-error` | `EDITDIST3()` | 1 |
| `rust-error` | `JSON_EXTRACT_PATH()` | 1 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_datediff` | 3 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_ddl` | 3 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 3 |
| `match` | `tests/dialects/test_sqlite.py` | `test_strftime` | 2 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres` | 2 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_ddl` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_longvarchar_dtype` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_window_null_treatment` | 1 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 1 |
| `oracle-error` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 1 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_hexadecimal_literal` | 1 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-sqlite-0144-test-sqlite`: `SELECT fname, lname, age FROM person ORDER BY age DESC NULLS FIRST, fname ASC NULLS LAST, lname`
  - expected: `SELECT fname, lname, age FROM person ORDER BY age DESC NULLS FIRST, fname ASC NULLS LAST, lname NULLS LAST`
  - actual: `SELECT fname, lname, age FROM person ORDER BY age DESC NULLS FIRST, fname NULLS LAST, lname NULLS LAST`

### `oracle-error`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-sqlite-0126-test-sqlite`: `SELECT CAST([a].[b] AS SMALLINT) FROM foo`
  - error: `Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Dot'>. Line 1, Col: 17. SELECT CAST([a].[b] AS SMALLINT) FROM foo`

### `rust-error`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-0587-test-postgres`: `JSON_EXTRACT_PATH('{"f2":{"f3":1},"f4":{"f5":99,"f6":"foo"}}','f4')`
  - expected: `'{"f2":{"f3":1},"f4":{"f5":99,"f6":"foo"}}' -> '$.f4'`
  - error: `Unexpected token: Token { token_type: Identifier, value: "JSON_EXTRACT_PATH", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-0910-test-postgres`: `1 / DIV(4, 2)`
  - expected: `1 / CAST(CAST(CAST(4 AS REAL) / 2 AS INTEGER) AS REAL)`
  - error: `Unexpected token: Token { token_type: Number, value: "1", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-sqlite-0133-test-sqlite`: `EDITDIST3(col1, col2)`
  - expected: `EDITDIST3(col1, col2)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "EDITDIST3", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-sqlite-0226-test-datediff`: `DATEDIFF(a, b, 'day')`
  - expected: `CAST((JULIANDAY(a) - JULIANDAY(b)) AS INTEGER)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "DATEDIFF", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-sqlite-0230-test-datediff`: `DATEDIFF(a, b, 'hour')`
  - expected: `CAST((JULIANDAY(a) - JULIANDAY(b)) * 24.0 AS INTEGER)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "DATEDIFF", line: 1, col: 1, position: 0, quote_char: '\0' }`

