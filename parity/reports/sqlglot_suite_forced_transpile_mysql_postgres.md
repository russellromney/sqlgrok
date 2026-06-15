# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_forced_transpile_mysql_postgres.jsonl`

Mode: `forced-pair`
Requested pair: `mysql` -> `postgres`

Total cases: `15156`
Observed helper attempts: `15156`
Filtered by read/write: `0`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 7725 |
| `mismatch` | 4995 |
| `oracle-error` | 1742 |
| `rust-error` | 557 |
| `unsupported-harness-shape` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5372 |
| `mismatch` | `validate_all` | 3563 |
| `match` | `validate_identity` | 2286 |
| `mismatch` | `validate_identity` | 1329 |
| `oracle-error` | `validate_identity` | 1137 |
| `oracle-error` | `validate_all` | 596 |
| `rust-error` | `validate_identity` | 307 |
| `rust-error` | `validate_all` | 246 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `mismatch` | `validate` | 103 |
| `match` | `validate` | 67 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 4 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 862 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 410 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 353 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 325 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 307 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 208 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 193 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 171 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 159 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 156 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 149 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 141 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 140 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 139 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 125 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 124 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 123 |
| `mismatch` | `tests/dialects/test_spark.py` | `test_spark` | 103 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_json` | 101 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 93 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 88 |
| `mismatch` | `tests/dialects/test_presto.py` | `test_presto` | 86 |
| `oracle-error` | `tests/dialects/test_tsql.py` | `test_option` | 86 |

## Examples

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `SELECT x union`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14.\n  SELECT x [4munion[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `SELECT x from`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mfrom[0m`

### `oracle-error` `tests/test_transpile.py:55`

- test: `test_alias`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `SELECT x join`
- expected: ``
- actual: ``
- error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13.\n  SELECT x [4mjoin[0m`

### `mismatch` `tests/test_transpile.py:645`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `-- aa */ SELECT * FROM secret_table --\nSELECT 1`
- expected: `/* aa * / SELECT * FROM secret_table -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `mismatch` `tests/test_transpile.py:649`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `-- comment */ DROP TABLE users --\nSELECT 1`
- expected: `/* comment * / DROP TABLE users -- */ SELECT 1`
- actual: `SELECT 1`
- error: ``

### `oracle-error` `tests/test_transpile.py:654`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `SELECT c /* c1 /* c2 */ c3 */`
- expected: ``
- actual: ``
- error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 28.\n  SELECT c /* c1 /* c2 */ c3 [4m*[0m/`

### `oracle-error` `tests/test_transpile.py:658`

- test: `test_comment_single_line_with_block_close`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `SELECT c /* c1 /* c2 /* c3 */ */ */`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 32.\n  SELECT c /* c1 /* c2 /* c3 */ *[4m/[0m */`

### `oracle-error` `tests/test_transpile.py:119`

- test: `test_comments`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `select /* asfd /* asdf */ asdf */ 1`
- expected: ``
- actual: ``
- error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 33.\n  select /* asfd /* asdf */ asdf *[4m/[0m 1`

### `mismatch` `tests/test_transpile.py:123`

- test: `test_comments`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `SELECT c /* foo */ AS alias`
- expected: `SELECT c AS alias /* foo */`
- actual: `SELECT c AS alias`
- error: ``

### `rust-error` `tests/test_transpile.py:127`

- test: `test_comments`
- helper: `validate`
- read/write: `mysql` -> `postgres`
- sql: `SELECT c AS /* foo */ (a, b, c) FROM t`
- expected: `SELECT c AS (a, b, c) /* foo */ FROM t`
- actual: ``
- error: `ValueError: Parser error: Expected identifier, got LParen ('(') at line 1 col 23`

