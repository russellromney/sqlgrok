# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_postgres_sqlite.jsonl`

Mode: `forced-pair`
Requested pair: `postgres` -> `sqlite`

Total cases: `15164`
Observed helper attempts: `15164`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 8451 |
| `mismatch` | 3503 |
| `oracle-error` | 1457 |
| `rust-error` | 1616 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5989 |
| `mismatch` | `validate_all` | 2504 |
| `match` | `validate_identity` | 2377 |
| `oracle-error` | `validate_identity` | 949 |
| `mismatch` | `validate_identity` | 923 |
| `rust-error` | `validate_identity` | 810 |
| `rust-error` | `validate_all` | 790 |
| `oracle-error` | `validate_all` | 502 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 85 |
| `mismatch` | `validate` | 76 |
| `rust-error` | `validate` | 16 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 6 |
| `unsupported-harness-shape` | `validate` | 5 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 829 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 418 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 311 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 301 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 287 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 227 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 219 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 200 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 190 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 178 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 138 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 138 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 136 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 125 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 119 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 118 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 107 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 102 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 97 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 95 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 95 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 87 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |

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

