# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_transpile_mysql_sqlite.jsonl`

Total cases: `23`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 18 |
| `mismatch` | 5 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 18 |
| `mismatch` | `validate_all` | 5 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 9 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 3 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_bits_literal` | 2 |
| `match` | `tests/dialects/test_mysql.py` | `test_canonical_functions` | 2 |
| `match` | `tests/dialects/test_mysql.py` | `test_ddl` | 2 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 2 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql` | 1 |
| `match` | `tests/dialects/test_mysql.py` | `test_safe_div` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_ddl` | 1 |

## Examples

### `mismatch` `tests/dialects/test_mysql.py:635`

- test: `test_bits_literal`
- helper: `validate_all`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT 0b1011`
- expected: `SELECT 11`
- actual: `SELECT 0 AS b1011`
- error: ``

### `mismatch` `tests/dialects/test_mysql.py:636`

- test: `test_bits_literal`
- helper: `validate_all`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT b'1011'`
- expected: `SELECT 11`
- actual: `SELECT b`
- error: ``

### `mismatch` `tests/dialects/test_mysql.py:610`

- test: `test_hexadecimal_literal`
- helper: `validate_all`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x'CC'`
- expected: `SELECT x'CC'`
- actual: `SELECT x`
- error: ``

### `mismatch` `tests/dialects/test_mysql.py:612`

- test: `test_hexadecimal_literal`
- helper: `validate_all`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x'0000CC'`
- expected: `SELECT x'0000CC'`
- actual: `SELECT x`
- error: ``

### `mismatch` `tests/dialects/test_mysql.py:986`

- test: `test_mysql`
- helper: `validate_all`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT JSON_EXTRACT('[10, 20, [30, 40]]', '$[1]', '$[0]')`
- expected: `SELECT JSON_EXTRACT('[10, 20, [30, 40]]', '$[1]', '$[0]')`
- actual: `SELECT '[10, 20, [30, 40]]' -> '$[1]'`
- error: ``

