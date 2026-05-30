# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_postgres_sqlite.jsonl`

Mode: `forced-pair`
Requested pair: `postgres` -> `sqlite`

Total cases: `15156`
Observed helper attempts: `15156`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 11051 |
| `mismatch` | 1915 |
| `oracle-error` | 1456 |
| `rust-error` | 597 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 8000 |
| `match` | `validate_identity` | 2947 |
| `mismatch` | `validate_all` | 1032 |
| `oracle-error` | `validate_identity` | 949 |
| `mismatch` | `validate_identity` | 813 |
| `oracle-error` | `validate_all` | 501 |
| `rust-error` | `validate_identity` | 350 |
| `rust-error` | `validate_all` | 244 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 104 |
| `mismatch` | `validate` | 70 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 6 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 1016 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 612 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 418 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 317 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 314 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 246 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 244 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 220 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 172 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 167 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 157 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 142 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 125 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 122 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 120 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 107 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 105 |
| `match` | `tests/dialects/test_oracle.py` | `test_oracle` | 104 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 104 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 89 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 84 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 83 |

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

### `mismatch` `tests/test_transpile.py:754`

- test: `test_alter`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
- actual: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo`
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

### `mismatch` `tests/test_transpile.py:123`

- test: `test_comments`
- helper: `validate`
- read/write: `postgres` -> `sqlite`
- sql: `SELECT c /* foo */ AS alias`
- expected: `SELECT c AS alias /* foo */`
- actual: `SELECT c AS alias`
- error: ``

