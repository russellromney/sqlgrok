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
| `match` | 7420 |
| `mismatch` | 4187 |
| `oracle-error` | 1743 |
| `rust-error` | 1677 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5384 |
| `mismatch` | `validate_all` | 2924 |
| `match` | `validate_identity` | 1975 |
| `mismatch` | `validate_identity` | 1169 |
| `oracle-error` | `validate_identity` | 1135 |
| `rust-error` | `validate_all` | 878 |
| `rust-error` | `validate_identity` | 780 |
| `oracle-error` | `validate_all` | 599 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `mismatch` | `validate` | 94 |
| `match` | `validate` | 61 |
| `rust-error` | `validate` | 19 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 752 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 359 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 317 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 297 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 240 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 199 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 197 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 196 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 166 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 157 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 154 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 144 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 143 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 137 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 132 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 127 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 122 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 118 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 102 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 97 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 95 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `mismatch` | `tests/dialects/test_presto.py` | `test_presto` | 86 |

## Examples

### `rust-error` `tests/test_transpile.py:51`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x AS union`
- expected: `SELECT x AS union`
- actual: ``
- error: `ValueError: Parser error: Expected identifier, got Union ('union') at line 1 col 13`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `rust-error` `tests/test_transpile.py:51`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x AS from`
- expected: `SELECT x AS from`
- actual: ``
- error: `ValueError: Parser error: Expected identifier, got From ('from') at line 1 col 13`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `rust-error` `tests/test_transpile.py:51`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `sqlite`
- sql: `SELECT x AS join`
- expected: `SELECT x AS join`
- actual: ``
- error: `ValueError: Parser error: Expected identifier, got Join ('join') at line 1 col 13`

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

