# SQLGlot Import Report

Source: `parity/reports/transpile_mysql_sqlite.jsonl`

Total candidates: `34`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 12 |
| `mismatch` | 2 |
| `oracle-error` | 1 |
| `rust-error` | 19 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `rust-error` | `GROUP_CONCAT()` | 8 |
| `match` | `CREATE TABLE` | 7 |
| `match` | `SELECT` | 5 |
| `rust-error` | `DATEDIFF()` | 3 |
| `rust-error` | `SELECT` | 3 |
| `mismatch` | `SELECT` | 2 |
| `rust-error` | `CREATE TABLE` | 2 |
| `oracle-error` | `SELECT` | 1 |
| `rust-error` | `A` | 1 |
| `rust-error` | `CHAR()` | 1 |
| `rust-error` | `EDITDIST3()` | 1 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_mysql` | 8 |
| `match` | `tests/dialects/test_sqlite.py` | `test_ddl` | 3 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_datediff` | 3 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 3 |
| `match` | `tests/dialects/test_mysql.py` | `test_ddl` | 2 |
| `match` | `tests/dialects/test_sqlite.py` | `test_strftime` | 2 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_ddl` | 2 |
| `match` | `tests/dialects/test_mysql.py` | `test_canonical_functions` | 1 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_longvarchar_dtype` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_window_null_treatment` | 1 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql` | 1 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 1 |
| `oracle-error` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 1 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_canonical_functions` | 1 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_safe_div` | 1 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_hexadecimal_literal` | 1 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-0986-test-mysql`: `SELECT JSON_EXTRACT('[10, 20, [30, 40]]', '$[1]', '$[0]')`
  - expected: `SELECT JSON_EXTRACT('[10, 20, [30, 40]]', '$[1]', '$[0]')`
  - actual: `SELECT '[10, 20, [30, 40]]' -> '$[1]'`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-sqlite-0144-test-sqlite`: `SELECT fname, lname, age FROM person ORDER BY age DESC NULLS FIRST, fname ASC NULLS LAST, lname`
  - expected: `SELECT fname, lname, age FROM person ORDER BY age DESC NULLS FIRST, fname ASC NULLS LAST, lname`
  - actual: `SELECT fname, lname, age FROM person ORDER BY age DESC NULLS FIRST, fname NULLS LAST, lname`

### `oracle-error`

- `sqlglot-mysql-to-sqlite-tests-dialects-test-sqlite-0126-test-sqlite`: `SELECT CAST([a].[b] AS SMALLINT) FROM foo`
  - error: `Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Dot'>. Line 1, Col: 17. SELECT CAST([a].[b] AS SMALLINT) FROM foo`

### `rust-error`

- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-1104-test-mysql`: `GROUP_CONCAT(DISTINCT x ORDER BY y DESC)`
  - expected: `GROUP_CONCAT(DISTINCT x)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "GROUP_CONCAT", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-1114-test-mysql`: `GROUP_CONCAT(x ORDER BY y SEPARATOR z)`
  - expected: `GROUP_CONCAT(x, z)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "GROUP_CONCAT", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-1124-test-mysql`: `GROUP_CONCAT(DISTINCT x ORDER BY y DESC SEPARATOR '')`
  - expected: `GROUP_CONCAT(DISTINCT x, '')`
  - error: `Unexpected token: Token { token_type: Identifier, value: "GROUP_CONCAT", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-1134-test-mysql`: `GROUP_CONCAT(a, b, c SEPARATOR ',')`
  - expected: `GROUP_CONCAT(a || b || c, ',')`
  - error: `Unexpected token: Token { token_type: Identifier, value: "GROUP_CONCAT", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-1145-test-mysql`: `GROUP_CONCAT(a, b, c SEPARATOR '')`
  - expected: `GROUP_CONCAT(a || b || c, '')`
  - error: `Unexpected token: Token { token_type: Identifier, value: "GROUP_CONCAT", line: 1, col: 1, position: 0, quote_char: '\0' }`

