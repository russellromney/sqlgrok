# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_sqlite_sqlite.jsonl`

Mode: `forced-pair`
Requested pair: `sqlite` -> `sqlite`

Total cases: `15156`
Observed helper attempts: `15156`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 9818 |
| `mismatch` | 3024 |
| `oracle-error` | 1545 |
| `rust-error` | 632 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 7027 |
| `match` | `validate_identity` | 2692 |
| `mismatch` | `validate_all` | 1965 |
| `oracle-error` | `validate_identity` | 993 |
| `mismatch` | `validate_identity` | 987 |
| `oracle-error` | `validate_all` | 543 |
| `rust-error` | `validate_identity` | 387 |
| `rust-error` | `validate_all` | 242 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 99 |
| `mismatch` | `validate` | 72 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 869 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 571 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 361 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 309 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 272 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 245 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 231 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 227 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 195 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 189 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 145 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 143 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 139 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 125 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 120 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 117 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 96 |
| `match` | `tests/dialects/test_oracle.py` | `test_oracle` | 93 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 84 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 82 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 80 |

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

