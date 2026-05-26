# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_sqlite_sqlite.jsonl`

Mode: `forced-pair`
Requested pair: `sqlite` -> `sqlite`

Total cases: `15170`
Observed helper attempts: `15170`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 8877 |
| `mismatch` | 3782 |
| `oracle-error` | 1546 |
| `rust-error` | 826 |
| `unsupported-harness-shape` | 139 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 6241 |
| `mismatch` | `validate_all` | 2658 |
| `match` | `validate_identity` | 2548 |
| `mismatch` | `validate_identity` | 1042 |
| `oracle-error` | `validate_identity` | 993 |
| `oracle-error` | `validate_all` | 544 |
| `rust-error` | `validate_identity` | 477 |
| `rust-error` | `validate_all` | 345 |
| `unsupported-harness-shape` | `validate_all` | 124 |
| `match` | `validate` | 88 |
| `mismatch` | `validate` | 82 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 4 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 857 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 483 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 315 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 314 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 268 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 235 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 224 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 193 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 187 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 177 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 169 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 128 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 120 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 118 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 112 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 101 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 100 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 90 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 90 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 88 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 84 |

## Examples

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT x join`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mjoin[0m`

### `mismatch` `tests/test_transpile.py:750`

- test: `test_alter`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT`
- actual: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- error: ``

### `mismatch` `tests/test_transpile.py:754`

- test: `test_alter`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
- actual: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- error: ``

### `mismatch` `tests/test_transpile.py:645`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `-- aa */ SELECT * FROM secret_table --\nSELECT 1`
- expected: `/* aa * / SELECT * FROM secret_table -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:649`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `-- comment */ DROP TABLE users --\nSELECT 1`
- expected: `/* comment * / DROP TABLE users -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `oracle-error` `tests/test_transpile.py:654`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT c /* c1 /* c2 */ c3 */`
- expected: ``
- actual: ``
- error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 28.\n  SELECT c /* c1 /* c2 */ c3 [4m*[0m/`

### `oracle-error` `tests/test_transpile.py:658`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT c /* c1 /* c2 /* c3 */ */ */`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 32.\n  SELECT c /* c1 /* c2 /* c3 */ *[4m/[0m */`

### `oracle-error` `tests/test_transpile.py:119`

- test: `test_comments`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `select /* asfd /* asdf */ asdf */ 1`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 33.\n  select /* asfd /* asdf */ asdf *[4m/[0m 1`

