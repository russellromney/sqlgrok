# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_sqlite_postgres.jsonl`

Mode: `forced-pair`
Requested pair: `sqlite` -> `postgres`

Total cases: `15156`
Observed helper attempts: `15156`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 7639 |
| `mismatch` | 5256 |
| `oracle-error` | 1545 |
| `rust-error` | 579 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5155 |
| `mismatch` | `validate_all` | 3849 |
| `match` | `validate_identity` | 2419 |
| `mismatch` | `validate_identity` | 1301 |
| `oracle-error` | `validate_identity` | 995 |
| `oracle-error` | `validate_all` | 541 |
| `rust-error` | `validate_identity` | 344 |
| `rust-error` | `validate_all` | 232 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `mismatch` | `validate` | 106 |
| `match` | `validate` | 65 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 810 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 413 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 368 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 353 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 306 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 230 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 209 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 202 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 190 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 154 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 153 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 146 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 139 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 124 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 124 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 122 |
| `mismatch` | `tests/dialects/test_spark.py` | `test_spark` | 118 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 105 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_json` | 101 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `mismatch` | `tests/dialects/test_presto.py` | `test_presto` | 85 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 81 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 74 |

## Examples

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `SELECT x join`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mjoin[0m`

### `mismatch` `tests/test_transpile.py:750`

- test: `test_alter`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE VARCHAR`
- actual: `ALTER TABLE integers ALTER COLUMN i TYPE VARCHAR`
- error: ``

### `mismatch` `tests/test_transpile.py:754`

- test: `test_alter`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE VARCHAR COLLATE foo USING bar`
- actual: `ALTER TABLE integers ALTER COLUMN i TYPE VARCHAR COLLATE foo`
- error: ``

### `mismatch` `tests/test_transpile.py:645`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `-- aa */ SELECT * FROM secret_table --\nSELECT 1`
- expected: `/* aa * / SELECT * FROM secret_table -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:649`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `-- comment */ DROP TABLE users --\nSELECT 1`
- expected: `/* comment * / DROP TABLE users -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `oracle-error` `tests/test_transpile.py:654`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `SELECT c /* c1 /* c2 */ c3 */`
- expected: ``
- actual: ``
- error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 28.\n  SELECT c /* c1 /* c2 */ c3 [4m*[0m/`

### `oracle-error` `tests/test_transpile.py:658`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `SELECT c /* c1 /* c2 /* c3 */ */ */`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 32.\n  SELECT c /* c1 /* c2 /* c3 */ *[4m/[0m */`

### `oracle-error` `tests/test_transpile.py:119`

- test: `test_comments`
- helper: `validate`
- read/write: `sqlite` -> `postgres`
- sql: `select /* asfd /* asdf */ asdf */ 1`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 33.\n  select /* asfd /* asdf */ asdf *[4m/[0m 1`

