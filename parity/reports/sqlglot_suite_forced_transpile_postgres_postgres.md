# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_postgres_postgres.jsonl`

Mode: `forced-pair`
Requested pair: `postgres` -> `postgres`

Total cases: `15156`
Observed helper attempts: `15156`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 8335 |
| `mismatch` | 4639 |
| `oracle-error` | 1456 |
| `rust-error` | 589 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5661 |
| `mismatch` | `validate_all` | 3374 |
| `match` | `validate_identity` | 2606 |
| `mismatch` | `validate_identity` | 1159 |
| `oracle-error` | `validate_identity` | 951 |
| `oracle-error` | `validate_all` | 499 |
| `rust-error` | `validate_identity` | 343 |
| `rust-error` | `validate_all` | 243 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `mismatch` | `validate` | 106 |
| `match` | `validate` | 68 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 6 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 864 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 451 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 352 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 319 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 283 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 275 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 230 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 171 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 169 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 145 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 142 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 140 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 123 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 122 |
| `mismatch` | `tests/dialects/test_spark.py` | `test_spark` | 120 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 115 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 109 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 109 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 108 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 97 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 90 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 89 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_json` | 88 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |

## Examples

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `SELECT x join`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mjoin[0m`

### `mismatch` `tests/test_transpile.py:750`

- test: `test_alter`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE VARCHAR`
- actual: `ALTER TABLE integers ALTER COLUMN i TYPE VARCHAR`
- error: ``

### `mismatch` `tests/test_transpile.py:754`

- test: `test_alter`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE VARCHAR COLLATE foo USING bar`
- actual: `ALTER TABLE integers ALTER COLUMN i TYPE VARCHAR COLLATE foo`
- error: ``

### `mismatch` `tests/test_transpile.py:645`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `-- aa */ SELECT * FROM secret_table --\nSELECT 1`
- expected: `/* aa * / SELECT * FROM secret_table -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:649`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `-- comment */ DROP TABLE users --\nSELECT 1`
- expected: `/* comment * / DROP TABLE users -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:654`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `SELECT c /* c1 /* c2 */ c3 */`
- expected: `SELECT c /* c1 / * c2 * / c3 */`
- actual: `SELECT c`
- error: ``

### `mismatch` `tests/test_transpile.py:658`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `SELECT c /* c1 /* c2 /* c3 */ */ */`
- expected: `SELECT c /* c1 / * c2 / * c3 * / * / */`
- actual: `SELECT c`
- error: ``

### `mismatch` `tests/test_transpile.py:119`

- test: `test_comments`
- helper: `validate`
- read/write: `postgres` -> `postgres`
- sql: `select /* asfd /* asdf */ asdf */ 1`
- expected: `/* asfd / * asdf * / asdf */ SELECT 1`
- actual: `SELECT 1`
- error: ``

