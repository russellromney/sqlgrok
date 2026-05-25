# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl`

Mode: `forced-pair`
Requested pair: `mysql` -> `sqlite`

Total cases: `15164`
Observed helper attempts: `15164`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 7954 |
| `mismatch` | 3909 |
| `oracle-error` | 1743 |
| `rust-error` | 1421 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5808 |
| `mismatch` | `validate_all` | 2698 |
| `match` | `validate_identity` | 2061 |
| `mismatch` | `validate_identity` | 1135 |
| `oracle-error` | `validate_identity` | 1135 |
| `rust-error` | `validate_identity` | 728 |
| `rust-error` | `validate_all` | 680 |
| `oracle-error` | `validate_all` | 599 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 85 |
| `mismatch` | `validate` | 76 |
| `rust-error` | `validate` | 13 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 781 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 371 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 346 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 283 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 271 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 212 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 196 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 190 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 182 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 182 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 161 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 159 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 140 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 124 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 111 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 105 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 103 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 100 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 98 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 96 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |

## Examples

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x join`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mjoin[0m`

### `mismatch` `tests/test_transpile.py:750`

- test: `test_alter`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT`
- actual: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- error: ``

### `mismatch` `tests/test_transpile.py:754`

- test: `test_alter`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
- actual: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- error: ``

### `mismatch` `tests/test_transpile.py:645`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `-- aa */ SELECT * FROM secret_table --\nSELECT 1`
- expected: `/* aa * / SELECT * FROM secret_table -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:649`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `-- comment */ DROP TABLE users --\nSELECT 1`
- expected: `/* comment * / DROP TABLE users -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `oracle-error` `tests/test_transpile.py:654`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT c /* c1 /* c2 */ c3 */`
- expected: ``
- actual: ``
- error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 28.\n  SELECT c /* c1 /* c2 */ c3 [4m*[0m/`

### `oracle-error` `tests/test_transpile.py:658`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT c /* c1 /* c2 /* c3 */ */ */`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 32.\n  SELECT c /* c1 /* c2 /* c3 */ *[4m/[0m */`

### `oracle-error` `tests/test_transpile.py:119`

- test: `test_comments`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `select /* asfd /* asdf */ asdf */ 1`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 33.\n  select /* asfd /* asdf */ asdf *[4m/[0m 1`

