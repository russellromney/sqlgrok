# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_postgres_duckdb.jsonl`

Mode: `forced-pair`
Requested pair: `postgres` -> `duckdb`

Total cases: `15156`
Observed helper attempts: `15156`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 6736 |
| `mismatch` | 6238 |
| `oracle-error` | 1456 |
| `rust-error` | 589 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `mismatch` | `validate_all` | 4731 |
| `match` | `validate_all` | 4302 |
| `match` | `validate_identity` | 2380 |
| `mismatch` | `validate_identity` | 1387 |
| `oracle-error` | `validate_identity` | 949 |
| `oracle-error` | `validate_all` | 501 |
| `rust-error` | `validate_identity` | 343 |
| `rust-error` | `validate_all` | 243 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `mismatch` | `validate` | 120 |
| `match` | `validate` | 54 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 6 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 662 |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 521 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 402 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 330 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 295 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 286 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 237 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 220 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 174 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 170 |
| `mismatch` | `tests/dialects/test_spark.py` | `test_spark` | 164 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 154 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 150 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 113 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_cast` | 110 |
| `mismatch` | `tests/dialects/test_presto.py` | `test_presto` | 105 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 104 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_json` | 103 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 102 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 101 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 92 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `mismatch` | `tests/dialects/test_oracle.py` | `test_oracle` | 86 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_array` | 85 |
| `oracle-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 84 |

## Examples

### `mismatch` `tests/test_transpile.py:51`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `SELECT x AS union`
- expected: `SELECT x AS "union"`
- actual: `SELECT x AS union`
- error: ``

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `mismatch` `tests/test_transpile.py:51`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `SELECT x AS from`
- expected: `SELECT x AS "from"`
- actual: `SELECT x AS from`
- error: ``

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `SELECT x join`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mjoin[0m`

### `mismatch` `tests/test_transpile.py:750`

- test: `test_alter`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT`
- actual: `ALTER TABLE integers ALTER COLUMN i TYPE VARCHAR`
- error: ``

### `mismatch` `tests/test_transpile.py:754`

- test: `test_alter`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
- actual: `ALTER TABLE integers ALTER COLUMN i TYPE VARCHAR COLLATE foo`
- error: ``

### `mismatch` `tests/test_transpile.py:645`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `-- aa */ SELECT * FROM secret_table --\nSELECT 1`
- expected: `/* aa * / SELECT * FROM secret_table -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:649`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `-- comment */ DROP TABLE users --\nSELECT 1`
- expected: `/* comment * / DROP TABLE users -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:654`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `duckdb`
- sql: `SELECT c /* c1 /* c2 */ c3 */`
- expected: `SELECT c /* c1 / * c2 * / c3 */`
- actual: `SELECT c`
- error: ``

