# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_sqlite_sqlite.jsonl`

Mode: `forced-pair`
Requested pair: `sqlite` -> `sqlite`

Total cases: `15164`
Observed helper attempts: `15164`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 7965 |
| `mismatch` | 3811 |
| `oracle-error` | 1549 |
| `rust-error` | 1702 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5644 |
| `mismatch` | `validate_all` | 2743 |
| `match` | `validate_identity` | 2249 |
| `oracle-error` | `validate_identity` | 993 |
| `mismatch` | `validate_identity` | 985 |
| `rust-error` | `validate_all` | 851 |
| `rust-error` | `validate_identity` | 832 |
| `oracle-error` | `validate_all` | 547 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `mismatch` | `validate` | 83 |
| `match` | `validate` | 72 |
| `rust-error` | `validate` | 19 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 798 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 358 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 319 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 280 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 259 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 225 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 224 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 193 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 177 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 175 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 162 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 150 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 136 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 131 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 128 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 118 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 117 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 115 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 108 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 95 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 87 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 84 |

## Examples

### `rust-error` `tests/test_transpile.py:51`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT x AS union`
- expected: `SELECT x AS union`
- actual: ``
- error: `ValueError: Parser error: Expected identifier, got Union ('union') at line 1 col 13`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `rust-error` `tests/test_transpile.py:51`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT x AS from`
- expected: `SELECT x AS from`
- actual: ``
- error: `ValueError: Parser error: Expected identifier, got From ('from') at line 1 col 13`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `rust-error` `tests/test_transpile.py:51`

- test: `test_alias`
- helper: `validate`
- read/write: `sqlite` -> `sqlite`
- sql: `SELECT x AS join`
- expected: `SELECT x AS join`
- actual: ``
- error: `ValueError: Parser error: Expected identifier, got Join ('join') at line 1 col 13`

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

