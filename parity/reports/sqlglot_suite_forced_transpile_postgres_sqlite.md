# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_postgres_sqlite.jsonl`

Mode: `forced-pair`
Requested pair: `postgres` -> `sqlite`

Total cases: `15170`
Observed helper attempts: `15170`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 9422 |
| `mismatch` | 3500 |
| `oracle-error` | 1457 |
| `rust-error` | 652 |
| `unsupported-harness-shape` | 139 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 6627 |
| `match` | `validate_identity` | 2697 |
| `mismatch` | `validate_all` | 2406 |
| `mismatch` | `validate_identity` | 1018 |
| `oracle-error` | `validate_identity` | 949 |
| `oracle-error` | `validate_all` | 502 |
| `rust-error` | `validate_identity` | 396 |
| `rust-error` | `validate_all` | 253 |
| `unsupported-harness-shape` | `validate_all` | 124 |
| `match` | `validate` | 98 |
| `mismatch` | `validate` | 76 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 6 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 895 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 516 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 382 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 311 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 288 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 227 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 218 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 200 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 191 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 189 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 139 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 125 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 119 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 117 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 111 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 111 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 98 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 97 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 93 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 83 |
| `oracle-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 82 |

## Examples

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `SELECT x join`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mjoin[0m`

### `mismatch` `tests/test_transpile.py:750`

- test: `test_alter`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT`
- actual: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- error: ``

### `mismatch` `tests/test_transpile.py:754`

- test: `test_alter`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
- actual: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- error: ``

### `mismatch` `tests/test_transpile.py:645`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `-- aa */ SELECT * FROM secret_table --\nSELECT 1`
- expected: `/* aa * / SELECT * FROM secret_table -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:649`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `-- comment */ DROP TABLE users --\nSELECT 1`
- expected: `/* comment * / DROP TABLE users -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:654`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `SELECT c /* c1 /* c2 */ c3 */`
- expected: `SELECT c /* c1 / * c2 * / c3 */`
- actual: `SELECT c`
- error: ``

### `mismatch` `tests/test_transpile.py:658`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `SELECT c /* c1 /* c2 /* c3 */ */ */`
- expected: `SELECT c /* c1 / * c2 / * c3 * / * / */`
- actual: `SELECT c`
- error: ``

### `mismatch` `tests/test_transpile.py:119`

- test: `test_comments`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `select /* asfd /* asdf */ asdf */ 1`
- expected: `/* asfd / * asdf * / asdf */ SELECT 1`
- actual: `SELECT 1`
- error: ``

