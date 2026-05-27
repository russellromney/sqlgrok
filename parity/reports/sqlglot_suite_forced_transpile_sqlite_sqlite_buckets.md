# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_sqlite_sqlite.jsonl`

Total rows: `15170`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 8952 |
| `mismatch` | 3803 |
| `oracle-error` | 1546 |
| `rust-error` | 730 |
| `unsupported-harness-shape` | 139 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `sqlite` | `sqlite` | 8952 |
| `mismatch` | `sqlite` | `sqlite` | 3803 |
| `oracle-error` | `sqlite` | `sqlite` | 1546 |
| `rust-error` | `sqlite` | `sqlite` | 730 |
| `unsupported-harness-shape` | `sqlite` | `sqlite` | 139 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 6284 |
| `mismatch` | `validate_all` | 2667 |
| `match` | `validate_identity` | 2579 |
| `mismatch` | `validate_identity` | 1054 |
| `oracle-error` | `validate_identity` | 993 |
| `oracle-error` | `validate_all` | 544 |
| `rust-error` | `validate_identity` | 434 |
| `rust-error` | `validate_all` | 293 |
| `unsupported-harness-shape` | `validate_all` | 124 |
| `match` | `validate` | 89 |
| `mismatch` | `validate` | 82 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 670 |
| `match` | `CAST()` | 532 |
| `match` | `SELECT operator multiply` | 300 |
| `mismatch` | `CREATE TABLE` | 238 |
| `match` | `CREATE TABLE` | 233 |
| `match` | `SHOW` | 215 |
| `oracle-error` | `SELECT` | 197 |
| `mismatch` | `SELECT` | 188 |
| `mismatch` | `CREATE` | 179 |
| `match` | `CREATE` | 165 |
| `match` | `TRUNC()` | 162 |
| `oracle-error` | `SELECT operator multiply` | 143 |
| `mismatch` | `SELECT operator multiply` | 126 |
| `match` | `ALTER TABLE` | 116 |
| `oracle-error` | `CREATE TABLE` | 113 |
| `match` | `X` | 107 |
| `match` | `WITH` | 90 |
| `mismatch` | `SELECT UNNEST()` | 88 |
| `match` | `SELECT DATEDIFF()` | 83 |
| `match` | `SELECT CAST()` | 78 |
| `match` | `SET` | 78 |
| `mismatch` | `DATE_ADD()` | 78 |
| `rust-error` | `SELECT` | 73 |
| `match` | `DATE_TRUNC()` | 71 |
| `rust-error` | `CREATE TABLE` | 68 |
| `match` | `LOG()` | 67 |
| `rust-error` | `SELECT operator multiply` | 64 |
| `match` | `GRANT` | 62 |
| `match` | `ANALYZE` | 60 |
| `mismatch` | `WITH` | 59 |
| `match` | `A` | 57 |
| `match` | `SELECT UNNEST()` | 57 |
| `mismatch` | `TIME_STR_TO_TIME()` | 57 |
| `match` | `REGEXP_INSTR()` | 56 |
| `match` | `REVOKE` | 56 |
| `match` | `SELECT SUM()` | 56 |
| `match` | `SELECT TO_TIMESTAMP()` | 55 |
| `oracle-error` | `WITH` | 52 |
| `mismatch` | `CAST()` | 51 |
| `rust-error` | `FROM` | 50 |

## Rust/Oracle/Unsupported Error Buckets

| Status | Error Bucket | Count |
| --- | --- | ---: |
| `oracle-error` | `oracle parse: Invalid expression / Unexpected token` | 752 |
| `oracle-error` | `oracle parse: Expecting )` | 472 |
| `oracle-error` | `oracle parse: Required keyword missing` | 132 |
| `unsupported-harness-shape` | `SQLGlot expects UnsupportedError` | 121 |
| `rust-error` | `parser: Expected identifier` | 48 |
| `oracle-error` | `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)` | 24 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }` | 21 |
| `oracle-error` | `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)` | 19 |
| `oracle-error` | `oracle parse: Expected AS after CAST` | 18 |
| `oracle-error` | `oracle parse: Expecting (` | 16 |
| `unsupported-harness-shape` | `identify helper option is not supported yet` | 14 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 14, start: 13, end: 13, comments: []>` | 12 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }` | 10 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 15, start: 14, end: 14, comments: []>` | 9 |
| `oracle-error` | `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)` | 9 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }` | 8 |
| `rust-error` | `parser: Expected RParen, got Comma (',')` | 8 |
| `rust-error` | `parser: Expected RParen, got Dot ('.')` | 8 |
| `rust-error` | `parser: Expected RParen, got Union ('UNION')` | 8 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 39, start: 38, end: 38, comments: []>` | 7 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>` | 7 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Into, value: "INTO", line: 1, col: 34, position: 33, quote_char: '\0' }` | 7 |
| `rust-error` | `parser: Expected RParen, got Order ('ORDER')` | 7 |
| `oracle-error` | `oracle token: Error tokenizing ' ARRAY[2], ARRAY[3]]) AS MAP(VARCHAR, ARRAY(INT))'` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Xor, value: "xor", line: 1, col: 8, position: 7, quote_char: '\0' }` | 6 |
| `rust-error` | `parser: Expected As, got Eof ('')` | 6 |
| `rust-error` | `parser: Expected LParen, got Unnest ('UNNEST')` | 6 |
| `rust-error` | `parser: Expected RParen, got Hour ('HOUR')` | 6 |
| `rust-error` | `parser: Expected RParen, got Identifier ('PLAN')` | 6 |
| `rust-error` | `parser: Expected RParen, got With ('WITH')` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: As, value: "AS", line: 1, col: 8, position: 7, quote_char: '\0' }` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Group, value: "group", line: 1, col: 51, position: 50, quote_char: '\0' }` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 24, position: 23, quote_char: '\0' }` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Using, value: "USING", line: 1, col: 19, position: 18, quote_char: '\0' }` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Xor, value: "XOR", line: 1, col: 1, position: 0, quote_char: '\0' }` | 5 |
| `rust-error` | `parser: Expected RParen, got Day ('DAY')` | 5 |
| `rust-error` | `parser: Expected RParen, got Group ('GROUP')` | 5 |
| `rust-error` | `parser: Expected RParen, got Identifier ('device_data')` | 5 |
| `rust-error` | `parser: Expected RParen, got LParen ('(')` | 5 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.NUMBER, text: 25, line: 1, col: 24, start: 22, end: 23, comments: []>` | 4 |

## Mismatch Signature Buckets

| Status | Signature | Count |
| --- | --- | ---: |
| `mismatch` | `missing AS or alias rendering` | 463 |
| `mismatch` | `missing quoted identifier` | 254 |
| `mismatch` | `DDL/create-table rendering` | 225 |
| `mismatch` | `case-only rendering difference` | 225 |
| `mismatch` | `SELECT` | 120 |
| `mismatch` | `SELECT operator multiply` | 114 |
| `mismatch` | `CREATE` | 101 |
| `mismatch` | `date/time rendering: DATE_ADD()` | 69 |
| `mismatch` | `date/time rendering: TIME_STR_TO_TIME()` | 57 |
| `mismatch` | `date/time rendering: SELECT DATE_SUB()` | 49 |
| `mismatch` | `ALTER TABLE` | 43 |
| `mismatch` | `cast/type rendering: CAST()` | 43 |
| `mismatch` | `cast/type rendering: SELECT TO_CHAR()` | 41 |
| `mismatch` | `quote-style difference` | 39 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 34 |
| `mismatch` | `date/time rendering: SELECT DATEADD()` | 32 |
| `mismatch` | `REPLACE()` | 31 |
| `mismatch` | `date/time rendering: STR_TO_TIME()` | 30 |
| `mismatch` | `REGEXP_EXTRACT()` | 29 |
| `mismatch` | `date/time rendering: SELECT DATE_ADD()` | 29 |
| `mismatch` | `date/time rendering: SELECT DATE_FORMAT()` | 29 |
| `mismatch` | `LEVENSHTEIN()` | 28 |
| `mismatch` | `MEDIAN()` | 28 |
| `mismatch` | `POSITION()` | 27 |
| `mismatch` | `REGEXP_REPLACE()` | 27 |
| `mismatch` | `json rendering: JSON_EXTRACT()` | 27 |
| `mismatch` | `MONTH()` | 26 |
| `mismatch` | `WITH` | 25 |
| `mismatch` | `YEAR()` | 24 |
| `mismatch` | `date/time rendering: CREATE` | 24 |
| `mismatch` | `SELECT REGEXP_EXTRACT()` | 22 |
| `mismatch` | `cast/type rendering: SELECT EXTRACT()` | 22 |
| `mismatch` | `SHA256()` | 21 |
| `mismatch` | `DAY()` | 20 |
| `mismatch` | `SELECT COUNT_IF()` | 20 |
| `mismatch` | `date/time rendering: EOMONTH()` | 20 |
| `mismatch` | `STRPOS()` | 19 |
| `mismatch` | `date/time rendering: SELECT UNNEST()` | 17 |
| `mismatch` | `LTRIM()` | 16 |
| `mismatch` | `MOD()` | 16 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 858 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 484 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 317 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 316 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 267 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 235 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 224 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 193 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 190 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 177 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 171 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 128 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 127 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 120 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 112 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 101 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 100 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 99 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 93 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 90 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 84 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 80 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 75 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `mismatch` | `tests/dialects/test_spark.py` | `test_spark` | 74 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 74 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 70 |
| `match` | `tests/dialects/test_databricks.py` | `test_databricks` | 69 |
| `match` | `tests/dialects/test_duckdb.py` | `test_time` | 68 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 67 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 67 |
| `match` | `tests/dialects/test_dialect.py` | `test_set_operators` | 66 |
| `match` | `tests/dialects/test_dialect.py` | `test_string_functions` | 64 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 64 |
| `match` | `tests/dialects/test_oracle.py` | `test_oracle` | 63 |
| `mismatch` | `tests/dialects/test_oracle.py` | `test_oracle` | 63 |

## Bucket Examples

### `mismatch` `ALTER TABLE`

- `tests/test_transpile.py`:750 `test_alter` via `validate`: `ALTER TABLE integers ALTER i TYPE VARCHAR`
  - expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT`
  - actual: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- `tests/test_transpile.py`:754 `test_alter` via `validate`: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
  - expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
  - actual: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- `tests/dialects/test_hive.py`:213 `test_ddl` via `validate_identity`: `ALTER TABLE X ADD COLUMNS (y INT, z STRING)`
  - expected: `ALTER TABLE X ADD COLUMNS (y INTEGER, z TEXT)`
  - actual: `ALTER TABLE X ADD COLUMNS (y INT, z STRING)`

### `mismatch` `CREATE`

- `tests/dialects/test_bigquery.py`:104 `test_bigquery` via `validate_identity`: `CREATE SCHEMA x DEFAULT COLLATE 'en'`
  - expected: `CREATE SCHEMA x`
  - actual: `CREATE SCHEMA x DEFAULT COLLATE 'en'`
- `tests/dialects/test_bigquery.py`:381 `test_bigquery` via `validate_identity`: `CREATE TEMPORARY FUNCTION FOO() RETURNS STRING LANGUAGE js AS 'return "Hello world!"'`
  - expected: `CREATE TEMPORARY FUNCTION FOO() AS 'return "Hello world!"'`
  - actual: `CREATE TEMPORARY FUNCTION FOO() RETURNS STRING LANGUAGE js AS 'return "Hello world!"'`
- `tests/dialects/test_clickhouse.py`:207 `test_clickhouse` via `validate_identity`: `CREATE MATERIALIZED VIEW test_view TO db.table1 (id UInt8) AS SELECT * FROM test_data`
  - expected: `CREATE VIEW test_view (id UInt8) AS SELECT * FROM test_data`
  - actual: `CREATE MATERIALIZED VIEW test_view TO db.table1 (id UInt8) AS SELECT * FROM test_data`

### `mismatch` `DDL/create-table rendering`

- `tests/test_transpile.py`:374 `test_comments` via `validate`: `-- comment4 CREATE TABLE db.tba AS SELECT a, b, c FROM tb_01 WHERE -- comment5 a = 1 AND b = 2 --comment6 -- and c = 1 -- comment7 ;`
  - expected: `/* comment4 */ CREATE TABLE db.tba AS SELECT a, b, c FROM tb_01 WHERE a /* comment5 */ = 1 AND b = 2 /* comment6 */ /* and c = 1 */ /* comment7 */`
  - actual: `CREATE TABLE db.tba AS SELECT a, b, c FROM tb_01 WHERE a = 1 AND b = 2`
- `tests/dialects/test_athena.py`:43 `test_ddl` via `validate_identity`: `CREATE EXTERNAL TABLE foo (id INT) COMMENT 'test comment'`
  - expected: `CREATE TABLE foo (id INTEGER)`
  - actual: `CREATE EXTERNAL TABLE foo (id INT) COMMENT 'test comment'`
- `tests/dialects/test_athena.py`:44 `test_ddl` via `validate_identity`: `CREATE EXTERNAL TABLE george.t (id INT COMMENT 'foo \\ bar') LOCATION 's3://my-bucket/'`
  - expected: `CREATE TABLE george.t (id INTEGER COMMENT 'foo \\ bar')`
  - actual: `CREATE EXTERNAL TABLE george.t (id INT COMMENT 'foo \\ bar') LOCATION 's3://my-bucket/'`

### `mismatch` `REGEXP_EXTRACT()`

- `tests/dialects/test_bigquery.py`:2898 `test_regexp_extract` via `validate_identity`: `REGEXP_EXTRACT(x, '(?<)')`
  - expected: `REGEXP_EXTRACT(x, '(?<)')`
  - actual: `REGEXP_SUBSTR(x, '(?<)')`
- `tests/dialects/test_bigquery.py`:2899 `test_regexp_extract` via `validate_identity`: `` REGEXP_EXTRACT(`foo`, 'bar: (.+?)', 1, 1) ``
  - expected: `REGEXP_EXTRACT("foo", 'bar: (.+?)', 1, 1)`
  - actual: `REGEXP_SUBSTR("foo", 'bar: (.+?)', 1)`
- `tests/dialects/test_hive.py`:900 `test_hive` via `validate_all`: `REGEXP_EXTRACT('abc', '(a)(b)(c)')`
  - expected: `REGEXP_EXTRACT('abc', '(a)(b)(c)')`
  - actual: `REGEXP_SUBSTR('abc', '(a)(b)(c)')`

### `mismatch` `REPLACE()`

- `tests/dialects/test_presto.py`:160 `test_replace` via `validate_all`: `REPLACE(subject, pattern)`
  - expected: `REPLACE (subject, pattern)`
  - actual: `REPLACE(subject, pattern)`
- `tests/dialects/test_presto.py`:160 `test_replace` via `validate_all`: `REPLACE(subject, pattern)`
  - expected: `REPLACE (subject, pattern)`
  - actual: `REPLACE(subject, pattern)`
- `tests/dialects/test_presto.py`:160 `test_replace` via `validate_all`: `REPLACE(subject, pattern)`
  - expected: `REPLACE (subject, pattern)`
  - actual: `REPLACE(subject, pattern)`

### `mismatch` `SELECT`

- `tests/test_transpile.py`:123 `test_comments` via `validate`: `SELECT c /* foo */ AS alias`
  - expected: `SELECT c AS alias /* foo */`
  - actual: `SELECT c AS alias`
- `tests/test_transpile.py`:143 `test_comments` via `validate`: `SELECT foo /* comments */ ;`
  - expected: `SELECT foo /* comments */`
  - actual: `SELECT foo`
- `tests/test_transpile.py`:155 `test_comments` via `validate`: `SELECT CASE /* test */ WHEN a THEN b ELSE c END`
  - expected: `SELECT CASE WHEN a THEN b ELSE c END /* test */`
  - actual: `SELECT CASE WHEN a THEN b ELSE c END`

### `mismatch` `SELECT operator multiply`

- `tests/test_transpile.py`:131 `test_comments` via `validate`: `SELECT * FROM t1 /*x*/ UNION ALL SELECT * FROM t2`
  - expected: `SELECT * FROM t1 /* x */ UNION ALL SELECT * FROM t2`
  - actual: `SELECT * FROM t1 UNION ALL SELECT * FROM t2`
- `tests/test_transpile.py`:139 `test_comments` via `validate`: `SELECT * FROM t1 /*x*/ INTERSECT ALL SELECT * FROM t2`
  - expected: `SELECT * FROM t1 /* x */ INTERSECT ALL SELECT * FROM t2`
  - actual: `SELECT * FROM t1 INTERSECT ALL SELECT * FROM t2`
- `tests/test_transpile.py`:147 `test_comments` via `validate`: `SELECT * FROM a INNER /* comments */ JOIN b`
  - expected: `SELECT * FROM a /* comments */ INNER JOIN b ON TRUE`
  - actual: `SELECT * FROM a INNER JOIN b`

### `mismatch` `case-only rendering difference`

- `tests/test_transpile.py`:704 `test_extract` via `validate`: `extract(week from current_date + 2)`
  - expected: `EXTRACT(WEEK FROM CURRENT_DATE + 2)`
  - actual: `EXTRACT(WEEK FROM current_date + 2)`
- `tests/test_transpile.py`:672 `test_types` via `validate`: `interval::int`
  - expected: `CAST(interval AS INTEGER)`
  - actual: `CAST(INTERVAL AS INTEGER)`
- `tests/test_transpile.py`:673 `test_types` via `validate`: `x::user_defined_type`
  - expected: `CAST(x AS user_defined_type)`
  - actual: `CAST(x AS USER_DEFINED_TYPE)`

### `mismatch` `cast/type rendering: CAST()`

- `tests/dialects/test_bigquery.py`:224 `test_bigquery` via `validate_identity`: `CAST(x AS BIGNUMERIC)`
  - expected: `CAST(x AS BIGDECIMAL)`
  - actual: `CAST(x AS BIGNUMERIC)`
- `tests/dialects/test_bigquery.py`:1162 `test_bigquery` via `validate_all`: `cast(x as time format 'YYYY.MM.DD HH:MI:SSTZH')`
  - expected: `STR_TO_TIME(x, 'YYYY.MM.DD HH:MI:SSTZH')`
  - actual: `CAST(x AS TIME)`
- `tests/dialects/test_clickhouse.py`:29 `test_clickhouse` via `validate_identity`: `CAST(x AS TINYBLOB)`
  - expected: `CAST(x AS BLOB)`
  - actual: `CAST(x AS TINYBLOB)`

### `mismatch` `cast/type rendering: SELECT CAST()`

- `tests/dialects/test_bigquery.py`:3844 `test_bignumeric` via `validate_all`: `SELECT CAST(1 AS BIGNUMERIC)`
  - expected: `SELECT CAST(1 AS BIGDECIMAL)`
  - actual: `SELECT CAST(1 AS BIGNUMERIC)`
- `tests/dialects/test_bigquery.py`:3844 `test_bignumeric` via `validate_all`: `SELECT CAST(1 AS BIGNUMERIC)`
  - expected: `SELECT CAST(1 AS BIGDECIMAL)`
  - actual: `SELECT CAST(1 AS BIGNUMERIC)`
- `tests/dialects/test_bigquery.py`:3042 `test_cast_format_with_parentheses` via `validate_identity`: `SELECT CAST('2026-03-24' AS STRING FORMAT ('YYYY'))`
  - expected: `SELECT CAST('2026-03-24' AS TEXT FORMAT 'YYYY')`
  - actual: `SELECT CAST('2026-03-24' AS TEXT)`

### `mismatch` `cast/type rendering: SELECT TO_CHAR()`

- `tests/dialects/test_dremio.py`:101 `test_time_mapping` via `validate_all`: `SELECT TO_CHAR(CAST('2025-06-24 12:34:56' AS TIMESTAMP), 'yyyy-mm-dd hh24:mi:ss')`
  - expected: `SELECT CAST(CAST('2025-06-24 12:34:56' AS TIMESTAMP) AS TEXT)`
  - actual: `SELECT STRFTIME('yyyy-mm-dd hh24:mi:ss', CAST('2025-06-24 12:34:56' AS TIMESTAMP))`
- `tests/dialects/test_dremio.py`:101 `test_time_mapping` via `validate_all`: `SELECT TO_CHAR(CAST('2025-06-24 12:34:56' AS TIMESTAMP), 'YYYY-MM-DD HH24:MI:SS')`
  - expected: `SELECT CAST(CAST('2025-06-24 12:34:56' AS TIMESTAMP) AS TEXT)`
  - actual: `SELECT STRFTIME('YYYY-MM-DD HH24:MI:SS', CAST('2025-06-24 12:34:56' AS TIMESTAMP))`
- `tests/dialects/test_dremio.py`:101 `test_time_mapping` via `validate_all`: `SELECT TO_CHAR(CAST('2025-06-24 12:34:56' AS TIMESTAMP), 'YYYY-MM-DD HH24:MI:SS')`
  - expected: `SELECT CAST(CAST('2025-06-24 12:34:56' AS TIMESTAMP) AS TEXT)`
  - actual: `SELECT STRFTIME('YYYY-MM-DD HH24:MI:SS', CAST('2025-06-24 12:34:56' AS TIMESTAMP))`

### `mismatch` `date/time rendering: DATE_ADD()`

- `tests/dialects/test_bigquery.py`:1511 `test_bigquery` via `validate_all`: `DATE_ADD(CURRENT_DATE(), INTERVAL -1 DAY)`
  - expected: `DATE(CURRENT_DATE, 'INTERVAL '-1' DAY')`
  - actual: `DATE_ADD(CURRENT_DATE, INTERVAL -1 DAY)`
- `tests/dialects/test_bigquery.py`:1511 `test_bigquery` via `validate_all`: `DATE_ADD(CURRENT_DATE(), INTERVAL -1 DAY)`
  - expected: `DATE(CURRENT_DATE, 'INTERVAL '-1' DAY')`
  - actual: `DATE_ADD(CURRENT_DATE, INTERVAL -1 DAY)`
- `tests/dialects/test_bigquery.py`:1511 `test_bigquery` via `validate_all`: `DATE_ADD(CURRENT_DATE(), INTERVAL -1 DAY)`
  - expected: `DATE(CURRENT_DATE, 'INTERVAL '-1' DAY')`
  - actual: `DATE_ADD(CURRENT_DATE, INTERVAL -1 DAY)`

### `mismatch` `date/time rendering: SELECT DATEADD()`

- `tests/dialects/test_clickhouse.py`:1558 `test_datetime_funcs` via `validate_identity`: `SELECT DATEADD(date, INTERVAL '3' YEAR)`
  - expected: `SELECT DATEADD(date, INTERVAL '3' YEAR)`
  - actual: `SELECT DATE_ADD(date, INTERVAL '3' YEAR)`
- `tests/dialects/test_clickhouse.py`:1568 `test_datetime_funcs` via `validate_identity`: `SELECT DATEADD(SECOND, 1, bar)`
  - expected: `SELECT DATEADD(SECOND, 1, bar)`
  - actual: `SELECT DATE_ADD(bar, 1, SECOND)`
- `tests/dialects/test_databricks.py`:388 `test_add_date` via `validate_all`: `SELECT DATEADD(year, 1, '2020-01-01')`
  - expected: `SELECT DATEADD(year, 1, '2020-01-01')`
  - actual: `SELECT DATE_ADD('2020-01-01', 1, YEAR)`

### `mismatch` `date/time rendering: SELECT DATE_ADD()`

- `tests/dialects/test_clickhouse.py`:1558 `test_datetime_funcs` via `validate_identity`: `SELECT DATE_ADD(date, INTERVAL '3' YEAR)`
  - expected: `SELECT DATE(date, 'INTERVAL '3' YEAR')`
  - actual: `SELECT DATE_ADD(date, INTERVAL '3' YEAR)`
- `tests/dialects/test_clickhouse.py`:1568 `test_datetime_funcs` via `validate_identity`: `SELECT DATE_ADD(SECOND, 1, bar)`
  - expected: `SELECT DATE(SECOND, '1 BAR')`
  - actual: `SELECT DATE_ADD(SECOND, 1)`
- `tests/dialects/test_databricks.py`:399 `test_add_date` via `validate_all`: `SELECT DATE_ADD('2020-01-01', 1)`
  - expected: `SELECT DATE('2020-01-01', '1')`
  - actual: `SELECT DATE_ADD('2020-01-01', 1)`

### `mismatch` `date/time rendering: SELECT DATE_SUB()`

- `tests/dialects/test_bigquery.py`:479 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL '2' DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL 2 DAY)`
- `tests/dialects/test_bigquery.py`:479 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL '2' DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL 2 DAY)`
- `tests/dialects/test_bigquery.py`:486 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(DATE '2008-12-25', INTERVAL 5 DAY)`
  - expected: `SELECT DATE_SUB(DATE('2008-12-25'), INTERVAL '5' DAY)`
  - actual: `SELECT DATE_SUB(DATE('2008-12-25'), INTERVAL 5 DAY)`

### `mismatch` `date/time rendering: STR_TO_TIME()`

- `tests/test_transpile.py`:791 `test_time` via `validate`: `STR_TO_TIME('x', 'y')`
  - expected: `STR_TO_TIME('x', 'y')`
  - actual: `TO_TIMESTAMP('x', 'y')`
- `tests/test_transpile.py`:806 `test_time` via `validate`: `STR_TO_TIME(x, 'y')`
  - expected: `STR_TO_TIME(x, 'y')`
  - actual: `TO_TIMESTAMP(x, 'y')`
- `tests/test_transpile.py`:811 `test_time` via `validate`: `STR_TO_TIME(x, 'yyyy-MM-dd HH:mm:ss')`
  - expected: `STR_TO_TIME(x, 'yyyy-MM-dd HH:mm:ss')`
  - actual: `TO_TIMESTAMP(x, 'yyyy-MM-dd HH:mm:ss')`

### `mismatch` `date/time rendering: TIME_STR_TO_TIME()`

- `tests/test_transpile.py`:829 `test_time` via `validate`: `TIME_STR_TO_TIME(x)`
  - expected: `x`
  - actual: `TIME_STR_TO_TIME(x)`
- `tests/test_transpile.py`:830 `test_time` via `validate`: `TIME_STR_TO_TIME(x, 'America/Los_Angeles')`
  - expected: `x`
  - actual: `TIME_STR_TO_TIME(x, 'America/Los_Angeles')`
- `tests/dialects/test_dialect.py`:817 `test_time` via `validate_all`: `TIME_STR_TO_TIME('2020-01-01')`
  - expected: `'2020-01-01'`
  - actual: `TIME_STR_TO_TIME('2020-01-01')`

### `mismatch` `missing AS or alias rendering`

- `tests/test_transpile.py`:901 `test_index_offset` via `validate`: `x[0]`
  - expected: `x AS "0"`
  - actual: `x[0]`
- `tests/test_transpile.py`:902 `test_index_offset` via `validate`: `x[1]`
  - expected: `x AS "1"`
  - actual: `x[1]`
- `tests/test_transpile.py`:904 `test_index_offset` via `validate`: `x[x - 1]`
  - expected: `x AS "x - 1"`
  - actual: `x`

### `mismatch` `missing quoted identifier`

- `tests/test_transpile.py`:176 `test_comments` via `validate`: `SELECT FUN(x) /*x*/, [1,2,3] /*y*/`
  - expected: `SELECT FUN(x) /* x */, "1,2,3" /* y */`
  - actual: `SELECT FUN(x), ARRAY[1, 2, 3]`
- `tests/dialects/test_athena.py`:62 `test_ddl` via `validate_identity`: `` CREATE EXTERNAL TABLE `my_table` (`a7` ARRAY<DATE>) ROW FORMAT SERDE 'a' STORED AS INPUTFORMAT 'b' OUTPUTFORMAT 'c' LOCATION 'd' TBLPROPERTIES ('e'='f') ``
  - expected: `CREATE TABLE "my_table" ("a7" ARRAY<DATE>)`
  - actual: `` CREATE EXTERNAL TABLE `my_table` (`a7` ARRAY<DATE>) ROW FORMAT SERDE 'a' STORED AS INPUTFORMAT 'b' OUTPUTFORMAT 'c' LOCATION 'd' TBLPROPERTIES ('e'='f') ``
- `tests/dialects/test_athena.py`:109 `test_ddl_quoting` via `validate_identity`: `` CREATE EXTERNAL TABLE `foo` (`id` INT) LOCATION 's3://foo/' ``
  - expected: `CREATE TABLE "foo" ("id" INTEGER)`
  - actual: `` CREATE EXTERNAL TABLE `foo` (`id` INT) LOCATION 's3://foo/' ``

### `mismatch` `quote-style difference`

- `tests/dialects/test_athena.py`:88 `test_ddl` via `validate_identity`: `` ALTER TABLE `foo`.`bar` ADD COLUMN `end_ts` BIGINT ``
  - expected: `ALTER TABLE "foo"."bar" ADD COLUMN "end_ts" INTEGER`
  - actual: `ALTER TABLE foo."bar" ADD COLUMN "end_ts" INTEGER`
- `tests/dialects/test_athena.py`:92 `test_ddl` via `validate_identity`: `` ALTER TABLE `foo` DROP COLUMN `id` ``
  - expected: `ALTER TABLE "foo" DROP COLUMN "id"`
  - actual: `ALTER TABLE "foo" DROP COLUMN id`
- `tests/dialects/test_athena.py`:106 `test_ddl_quoting` via `validate_identity`: `` CREATE SCHEMA `foo` ``
  - expected: `CREATE SCHEMA "foo"`
  - actual: `` CREATE SCHEMA `foo` ``

### `oracle-error` `oracle parse: Expected AS after CAST`

- `tests/dialects/test_clickhouse.py`:20 `test_clickhouse` via `validate_identity`: `cast(notEmpty(report_task_id)?report_task_id:'-1' AS text)`
  - error: `ParseError: Expected AS after CAST. Line 1, Col: 45. cast(notEmpty(report_task_id)?report_task_id:'-1' AS text)`
- `tests/dialects/test_databricks.py`:274 `test_json` via `validate_identity`: `SELECT TRY_CAST(c1:price AS ARRAY<VARIANT>)`
  - error: `ParseError: Expected AS after CAST. Line 1, Col: 19. SELECT TRY_CAST(c1:price AS ARRAY<VARIANT>)`
- `tests/dialects/test_databricks.py`:275 `test_json` via `validate_identity`: `SELECT TRY_CAST(c1:["foo bar"]["baz qux"] AS ARRAY<VARIANT>)`
  - error: `ParseError: Expected AS after CAST. Line 1, Col: 19. SELECT TRY_CAST(c1:["foo bar"]["baz qux"] AS ARRAY<VARIANT>)`

### `oracle-error` `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 14, start: 13, end: 13, comments: []>`

- `tests/dialects/test_tsql.py`:1309 `test_ddl` via `validate_all`: `CREATE TABLE #mytemp (a INTEGER, b CHAR(2), c TIME(4), d FLOAT(24))`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 14, start: 13, end: 13, comments: []>. Line 1, Col: 14. CREATE TABLE #mytemp (a INTEGER, b CHAR(2), c TIME(4), d FLOAT(24))`
- `tests/dialects/test_tsql.py`:1309 `test_ddl` via `validate_all`: `CREATE TABLE #mytemp (a INTEGER, b CHAR(2), c TIME(4), d FLOAT(24))`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 14, start: 13, end: 13, comments: []>. Line 1, Col: 14. CREATE TABLE #mytemp (a INTEGER, b CHAR(2), c TIME(4), d FLOAT(24))`
- `tests/dialects/test_tsql.py`:238 `test_tsql` via `validate_all`: `CREATE TABLE #mytemptable (a INTEGER)`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 14, start: 13, end: 13, comments: []>. Line 1, Col: 14. CREATE TABLE #mytemptable (a INTEGER)`

### `oracle-error` `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 15, start: 14, end: 14, comments: []>`

- `tests/dialects/test_redshift.py`:345 `test_identity` via `validate_identity`: `SELECT * FROM #x`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM #x`
- `tests/dialects/test_tsql.py`:2146 `test_identifier_prefixes` via `validate_all`: `SELECT * FROM #mytemptable`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM #mytemptable`
- `tests/dialects/test_tsql.py`:2146 `test_identifier_prefixes` via `validate_all`: `SELECT * FROM #mytemptable`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM #mytemptable`

### `oracle-error` `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 39, start: 38, end: 38, comments: []>`

- `tests/dialects/test_tsql.py`:182 `test_tsql` via `validate_all`: `WITH t(c) AS (SELECT 1) SELECT c INTO #foo FROM t`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 39, start: 38, end: 38, comments: []>. Line 1, Col: 39. WITH t(c) AS (SELECT 1) SELECT c INTO #foo FROM t`
- `tests/dialects/test_tsql.py`:182 `test_tsql` via `validate_all`: `WITH t(c) AS (SELECT 1) SELECT c INTO #foo FROM t`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 39, start: 38, end: 38, comments: []>. Line 1, Col: 39. WITH t(c) AS (SELECT 1) SELECT c INTO #foo FROM t`
- `tests/dialects/test_tsql.py`:182 `test_tsql` via `validate_all`: `WITH t(c) AS (SELECT 1) SELECT c INTO #foo FROM t`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 39, start: 38, end: 38, comments: []>. Line 1, Col: 39. WITH t(c) AS (SELECT 1) SELECT c INTO #foo FROM t`

### `oracle-error` `oracle parse: Expecting (`

- `tests/dialects/test_clickhouse.py`:54 `test_clickhouse` via `validate_identity`: `WITH arrayJoin([(1, [2, 3])]) AS arr SELECT arr`
  - error: `ParseError: Expecting (. Line 1, Col: 28. WITH arrayJoin([(1, [2, 3])]) AS arr SELECT arr`
- `tests/dialects/test_clickhouse.py`:629 `test_clickhouse` via `validate_identity`: `ALTER TABLE visits DROP PARTITION 201901`
  - error: `ParseError: Expecting (. Line 1, Col: 40. ALTER TABLE visits DROP PARTITION 201901`
- `tests/dialects/test_clickhouse.py`:630 `test_clickhouse` via `validate_identity`: `ALTER TABLE visits DROP PARTITION ALL`
  - error: `ParseError: Expecting (. Line 1, Col: 37. ALTER TABLE visits DROP PARTITION ALL`

### `oracle-error` `oracle parse: Expecting )`

- `tests/test_transpile.py`:734 `test_with` via `validate`: `WITH A(filter) AS (VALUES 1, 2, 3) SELECT * FROM A WHERE filter >= 2`
  - error: `ParseError: Expecting ). Line 1, Col: 27. WITH A(filter) AS (VALUES 1, 2, 3) SELECT * FROM A WHERE filter >= 2`
- `tests/dialects/test_athena.py`:77 `test_ddl` via `validate_identity`: `CREATE TABLE foo WITH (table_type='ICEBERG', location='s3://foo/', format='orc', partitioning=ARRAY['bucket(id, 5)']) AS SELECT * FROM a`
  - error: `ParseError: Expecting ). Line 1, Col: 116. CREATE TABLE foo WITH (table_type='ICEBERG', location='s3://foo/', format='orc', partitioning=ARRAY['bucket(id, 5)']) AS SELECT * FROM a`
- `tests/dialects/test_athena.py`:80 `test_ddl` via `validate_identity`: `CREATE TABLE foo WITH (table_type='HIVE', external_location='s3://foo/', format='parquet', partitioned_by=ARRAY['ds']) AS SELECT * FROM a`
  - error: `ParseError: Expecting ). Line 1, Col: 117. E foo WITH (table_type='HIVE', external_location='s3://foo/', format='parquet', partitioned_by=ARRAY['ds']) AS SELECT * FROM a`

### `oracle-error` `oracle parse: Invalid expression / Unexpected token`

- `tests/test_transpile.py`:654 `test_comment_single_line_with_block_close` via `validate`: `SELECT c /* c1 /* c2 */ c3 */`
  - error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 28. SELECT c /* c1 /* c2 */ c3 */`
- `tests/test_transpile.py`:248 `test_comments` via `validate`: `/* multi line comment */ SELECT tbl.cola /* comment 1 */ + tbl.colb /* comment 2 */, CAST(x AS CHAR), # comment 3 y -- comment 4 FROM bar /* comment 5 */, tbl # comment 6`
  - error: `ParseError: Invalid expression / Unexpected token. Line 8, Col: 32. T tbl.cola /* comment 1 */ + tbl.colb /* comment 2 */, CAST(x AS CHAR), # comment 3 y -- comment 4 FROM bar /* comment`
- `tests/dialects/test_athena.py`:15 `test_athena` via `validate_identity`: `UNLOAD (SELECT name1, address1, comment1, key1 FROM table1) TO 's3://amzn-s3-demo-bucket/ partitioned/' WITH (format = 'TEXTFILE', partitioned_by = ARRAY['key1'])`
  - error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 103. UNLOAD (SELECT name1, address1, comment1, key1 FROM table1) TO 's3://amzn-s3-demo-bucket/ partitioned/' WITH (format = 'TEXTFILE', partitioned_by = ARRAY['key1'])`

### `oracle-error` `oracle parse: Required keyword missing`

- `tests/test_transpile.py`:55 `test_alias` via `validate`: `SELECT x union`
  - error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14. SELECT x union`
- `tests/test_transpile.py`:658 `test_comment_single_line_with_block_close` via `validate`: `SELECT c /* c1 /* c2 /* c3 */ */ */`
  - error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 32. SELECT c /* c1 /* c2 /* c3 */ */ */`
- `tests/test_transpile.py`:119 `test_comments` via `validate`: `select /* asfd /* asdf */ asdf */ 1`
  - error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Mul'>. Line 1, Col: 33. select /* asfd /* asdf */ asdf */ 1`

### `oracle-error` `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)`

- `tests/dialects/test_clickhouse.py`:66 `test_clickhouse` via `validate_identity`: `countIf(x, y)`
  - error: `ParseError: The number of provided arguments (2) is greater than the maximum number of supported arguments (1). Line 1, Col: 13. countIf(x, y)`
- `tests/dialects/test_hive.py`:968 `test_hive` via `validate_all`: `SELECT FIRST_VALUE(sample_col, TRUE)`
  - error: `ParseError: The number of provided arguments (2) is greater than the maximum number of supported arguments (1). Line 1, Col: 36. SELECT FIRST_VALUE(sample_col, TRUE)`
- `tests/dialects/test_hive.py`:968 `test_hive` via `validate_all`: `SELECT FIRST_VALUE(sample_col, TRUE)`
  - error: `ParseError: The number of provided arguments (2) is greater than the maximum number of supported arguments (1). Line 1, Col: 36. SELECT FIRST_VALUE(sample_col, TRUE)`

### `oracle-error` `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)`

- `tests/dialects/test_clickhouse.py`:1764 `test_functions` via `validate_identity`: `SELECT TRANSFORM(foo, [1, 2], ['first', 'second']) FROM table`
  - error: `ParseError: The number of provided arguments (3) is greater than the maximum number of supported arguments (2). Line 1, Col: 50. SELECT TRANSFORM(foo, [1, 2], ['first', 'second']) FROM table`
- `tests/dialects/test_snowflake.py`:798 `test_snowflake` via `validate_identity`: `SELECT ARRAY_SORT(x, TRUE, FALSE)`
  - error: `ParseError: The number of provided arguments (3) is greater than the maximum number of supported arguments (2). Line 1, Col: 33. SELECT ARRAY_SORT(x, TRUE, FALSE)`
- `tests/dialects/test_snowflake.py`:815 `test_snowflake` via `validate_all`: `SELECT ARRAY_SORT(x, foo, TRUE)`
  - error: `ParseError: The number of provided arguments (3) is greater than the maximum number of supported arguments (2). Line 1, Col: 31. SELECT ARRAY_SORT(x, foo, TRUE)`

### `oracle-error` `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)`

- `tests/dialects/test_clickhouse.py`:90 `test_clickhouse` via `validate_identity`: `'a' IN mapKeys(map('a', 1, 'b', 2))`
  - error: `ParseError: The number of provided arguments (4) is greater than the maximum number of supported arguments (2). Line 1, Col: 34. 'a' IN mapKeys(map('a', 1, 'b', 2))`
- `tests/dialects/test_clickhouse.py`:1765 `test_functions` via `validate_identity`: `SELECT TRANSFORM(foo, [1, 2], ['first', 'second'], 'default') FROM table`
  - error: `ParseError: The number of provided arguments (4) is greater than the maximum number of supported arguments (2). Line 1, Col: 61. SELECT TRANSFORM(foo, [1, 2], ['first', 'second'], 'default') FROM table`
- `tests/dialects/test_hive.py`:751 `test_hive` via `validate_all`: `map(a, b, c, d)`
  - error: `ParseError: The number of provided arguments (4) is greater than the maximum number of supported arguments (2). Line 1, Col: 15. map(a, b, c, d)`

### `rust-error` `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }`

- `tests/dialects/test_snowflake.py`:1367 `test_snowflake` via `validate_all`: `SELECT PARSE_JSON('{"fruit":"banana"}'):fruit`
  - expected: `SELECT '{"fruit":"banana"}' AS :fruit`
  - error: `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }`
- `tests/dialects/test_snowflake.py`:1367 `test_snowflake` via `validate_all`: `SELECT PARSE_JSON('{"fruit":"banana"}'):fruit`
  - expected: `SELECT '{"fruit":"banana"}' AS :fruit`
  - error: `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }`
- `tests/dialects/test_snowflake.py`:1367 `test_snowflake` via `validate_all`: `SELECT PARSE_JSON('{"fruit":"banana"}'):fruit`
  - expected: `SELECT '{"fruit":"banana"}' AS :fruit`
  - error: `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }`

### `rust-error` `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }`

- `tests/dialects/test_bigquery.py`:2491 `test_rename_table` via `validate_all`: `ALTER TABLE db.t1 RENAME TO db.t2`
  - expected: `ALTER TABLE db.t1 RENAME TO db.t2`
  - error: `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }`
- `tests/dialects/test_bigquery.py`:2491 `test_rename_table` via `validate_all`: `ALTER TABLE db.t1 RENAME TO db.t2`
  - expected: `ALTER TABLE db.t1 RENAME TO db.t2`
  - error: `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }`
- `tests/dialects/test_doris.py`:243 `test_rename_table` via `validate_all`: `ALTER TABLE db.t1 RENAME TO db.t2`
  - expected: `ALTER TABLE db.t1 RENAME TO db.t2`
  - error: `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }`

### `rust-error` `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }`

- `tests/dialects/test_clickhouse.py`:835 `test_parameterization` via `validate_all`: `SELECT {abc: UInt32}, {b: String}, {c: DateTime},{d: Map(String, Array(UInt8))}, {e: Tuple(UInt8, String)}`
  - expected: `SELECT STRUCT(UInt32 AS abc), STRUCT(String AS b), STRUCT(DateTime AS c), STRUCT(MAP(String, ARRAY(UInt8)) AS d), STRUCT(TUPLE(UInt8, String) AS e)`
  - error: `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }`
- `tests/dialects/test_clickhouse.py`:835 `test_parameterization` via `validate_all`: `SELECT {abc: UInt32}, {b: String}, {c: DateTime},{d: Map(String, Array(UInt8))}, {e: Tuple(UInt8, String)}`
  - expected: `SELECT STRUCT(UInt32 AS abc), STRUCT(String AS b), STRUCT(DateTime AS c), STRUCT(MAP(String, ARRAY(UInt8)) AS d), STRUCT(TUPLE(UInt8, String) AS e)`
  - error: `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }`
- `tests/dialects/test_duckdb.py`:345 `test_duckdb` via `validate_all`: `SELECT {'bla': column1, 'foo': column2, 'bar': column3} AS data FROM source_table`
  - expected: `SELECT STRUCT(column1 AS bla, column2 AS foo, column3 AS bar) AS data FROM source_table`
  - error: `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }`

### `rust-error` `parser: Expected RParen, got Comma (',')`

- `tests/dialects/test_hive.py`:919 `test_hive` via `validate_identity`: `EXISTS(col, x -> x % 2 = 0)`
  - expected: `EXISTS(col)`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 11`
- `tests/dialects/test_hive.py`:921 `test_hive` via `validate_all`: `SELECT EXISTS(ARRAY(2, 3), x -> x % 2 = 0)`
  - expected: `SELECT EXISTS(ARRAY(2, 3))`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 26`
- `tests/dialects/test_hive.py`:921 `test_hive` via `validate_all`: `SELECT EXISTS(ARRAY(2, 3), x -> x % 2 = 0)`
  - expected: `SELECT EXISTS(ARRAY(2, 3))`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 26`

### `rust-error` `parser: Expected RParen, got Dot ('.')`

- `tests/dialects/test_bigquery.py`:1488 `test_bigquery` via `validate_all`: `DELETE FROM db.t1 AS t1 WHERE NOT t1.c IN (SELECT db.t2.c FROM db.t2)`
  - expected: `DELETE FROM db.t1 AS t1 WHERE NOT t1.c IN (SELECT db.t2.c FROM db.t2)`
  - error: `ValueError: Parser error: Expected RParen, got Dot ('.') at line 1 col 56`
- `tests/dialects/test_bigquery.py`:1488 `test_bigquery` via `validate_all`: `DELETE FROM db.t1 AS t1 WHERE NOT t1.c IN (SELECT db.t2.c FROM db.t2)`
  - expected: `DELETE FROM db.t1 AS t1 WHERE NOT t1.c IN (SELECT db.t2.c FROM db.t2)`
  - error: `ValueError: Parser error: Expected RParen, got Dot ('.') at line 1 col 56`
- `tests/dialects/test_dialect.py`:3090 `test_merge` via `validate_all`: `MERGE INTO foo AS target USING (SELECT a, b FROM tbl) AS src ON src.a = target.a WHEN MATCHED AND target.a <> src.a THEN UPDATE SET target.b = 'FOO' WHEN NOT MATCHED THEN INSERT (target.a, target.b) VALUES (src.a, src.b)`
  - expected: `MERGE INTO foo AS target USING (SELECT a, b FROM tbl) AS src ON src.a = target.a WHEN MATCHED AND target.a <> src.a THEN UPDATE SET target.b = 'FOO' WHEN NOT MATCHED THEN INSERT (target.a, target.b) VALUES (src.a, src.b)`
  - error: `ValueError: Parser error: Expected RParen, got Dot ('.') at line 3 col 49`

### `rust-error` `parser: Expected RParen, got Union ('UNION')`

- `tests/dialects/test_presto.py`:1054 `test_presto` via `validate_all`: `WITH RECURSIVE t(n) AS (VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < 100 ) SELECT SUM(n) FROM t`
  - expected: `WITH RECURSIVE t(n) AS (VALUES (1) UNION ALL SELECT n + 1 FROM t WHERE n < 100) SELECT SUM(n) FROM t`
  - error: `ValueError: Parser error: Expected RParen, got Union ('UNION') at line 1 col 36`
- `tests/dialects/test_presto.py`:1054 `test_presto` via `validate_all`: `WITH RECURSIVE t(n) AS (VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < 100 ) SELECT SUM(n) FROM t`
  - expected: `WITH RECURSIVE t(n) AS (VALUES (1) UNION ALL SELECT n + 1 FROM t WHERE n < 100) SELECT SUM(n) FROM t`
  - error: `ValueError: Parser error: Expected RParen, got Union ('UNION') at line 1 col 36`
- `tests/dialects/test_tsql.py`:641 `test_option` via `validate_identity`: `SELECT * FROM Table1 OPTION(CONCAT UNION)`
  - expected: `SELECT * FROM Table1 AS OPTION`
  - error: `ValueError: Parser error: Expected RParen, got Union ('UNION') at line 1 col 36`

### `rust-error` `parser: Expected identifier`

- `tests/test_transpile.py`:127 `test_comments` via `validate`: `SELECT c AS /* foo */ (a, b, c) FROM t`
  - expected: `SELECT c AS (a, b, c) /* foo */ FROM t`
  - error: `ValueError: Parser error: Expected identifier, got LParen ('(') at line 1 col 23`
- `tests/dialects/test_dialect.py`:3972 `test_escaped_identifier_delimiter` via `validate_all`: `SELECT 1 AS [[x]]]`
  - expected: `SELECT 1 AS "[x]"`
  - error: `ValueError: Parser error: Expected identifier, got LBracket ('[') at line 1 col 13`
- `tests/dialects/test_dialect.py`:5263 `test_operator` via `validate_identity`: `SELECT 1 OPERATOR(pg_catalog.+) 2`
  - expected: `SELECT 1 OPERATOR(pg_catalog.+) 2`
  - error: `ValueError: Parser error: Expected identifier, got Plus ('+') at line 1 col 30`

### `unsupported-harness-shape` `SQLGlot expects UnsupportedError`

- `tests/dialects/test_bigquery.py`:494 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
  - error: `SQLGlot expects UnsupportedError`
- `tests/dialects/test_bigquery.py`:494 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
  - error: `SQLGlot expects UnsupportedError`
- `tests/dialects/test_bigquery.py`:494 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
  - error: `SQLGlot expects UnsupportedError`

### `unsupported-harness-shape` `identify helper option is not supported yet`

- `tests/test_transpile.py`:925 `test_identify_lambda` via `validate`: `x(y -> y)`
  - expected: `X("y" -> "y")`
  - error: `identify helper option is not supported yet`
- `tests/dialects/test_athena.py`:30 `test_athena` via `validate_identity`: `/* leading comment */CREATE SCHEMA foo`
  - expected: `` /* leading comment */ CREATE SCHEMA `foo` ``
  - error: `identify helper option is not supported yet`
- `tests/dialects/test_athena.py`:35 `test_athena` via `validate_identity`: `/* leading comment */SELECT * FROM foo`
  - expected: `/* leading comment */ SELECT * FROM "foo"`
  - error: `identify helper option is not supported yet`

