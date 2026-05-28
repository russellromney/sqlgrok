# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_postgres_sqlite.jsonl`

Total rows: `15156`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 9923 |
| `mismatch` | 2989 |
| `oracle-error` | 1456 |
| `rust-error` | 651 |
| `unsupported-harness-shape` | 137 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `postgres` | `sqlite` | 9923 |
| `mismatch` | `postgres` | `sqlite` | 2989 |
| `oracle-error` | `postgres` | `sqlite` | 1456 |
| `rust-error` | `postgres` | `sqlite` | 651 |
| `unsupported-harness-shape` | `postgres` | `sqlite` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 7057 |
| `match` | `validate_identity` | 2765 |
| `mismatch` | `validate_all` | 1966 |
| `mismatch` | `validate_identity` | 950 |
| `oracle-error` | `validate_identity` | 949 |
| `oracle-error` | `validate_all` | 501 |
| `rust-error` | `validate_identity` | 395 |
| `rust-error` | `validate_all` | 253 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 101 |
| `mismatch` | `validate` | 73 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 6 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 726 |
| `match` | `CAST()` | 547 |
| `match` | `SELECT operator multiply` | 331 |
| `match` | `CREATE TABLE` | 246 |
| `mismatch` | `CREATE TABLE` | 246 |
| `match` | `SHOW` | 215 |
| `oracle-error` | `SELECT` | 211 |
| `mismatch` | `CREATE` | 195 |
| `match` | `TRUNC()` | 164 |
| `match` | `CREATE` | 162 |
| `oracle-error` | `SELECT operator multiply` | 150 |
| `mismatch` | `SELECT` | 134 |
| `match` | `ALTER TABLE` | 115 |
| `mismatch` | `SELECT UNNEST()` | 113 |
| `match` | `X` | 109 |
| `oracle-error` | `CREATE TABLE` | 106 |
| `match` | `WITH` | 95 |
| `mismatch` | `SELECT operator multiply` | 92 |
| `match` | `SELECT CAST()` | 84 |
| `match` | `SELECT DATEDIFF()` | 83 |
| `match` | `SET` | 78 |
| `mismatch` | `DATE_ADD()` | 78 |
| `match` | `DATE_TRUNC()` | 73 |
| `match` | `LOG()` | 67 |
| `match` | `GRANT` | 65 |
| `mismatch` | `WITH` | 65 |
| `match` | `A` | 60 |
| `match` | `ANALYZE` | 60 |
| `rust-error` | `SELECT operator multiply` | 60 |
| `match` | `REVOKE` | 59 |
| `match` | `TIME_STR_TO_TIME()` | 57 |
| `match` | `REGEXP_INSTR()` | 56 |
| `match` | `SELECT UNNEST()` | 56 |
| `match` | `SELECT SUM()` | 55 |
| `match` | `INSERT` | 54 |
| `mismatch` | `CAST()` | 53 |
| `rust-error` | `SELECT` | 53 |
| `match` | `FROM` | 50 |
| `mismatch` | `ALTER TABLE` | 50 |
| `mismatch` | `SELECT DATE_SUB()` | 49 |

## Rust/Oracle/Unsupported Error Buckets

| Status | Error Bucket | Count |
| --- | --- | ---: |
| `oracle-error` | `oracle parse: Invalid expression / Unexpected token` | 712 |
| `oracle-error` | `oracle parse: Expecting )` | 372 |
| `oracle-error` | `oracle parse: Required keyword missing` | 145 |
| `unsupported-harness-shape` | `SQLGlot expects UnsupportedError` | 119 |
| `oracle-error` | `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)` | 46 |
| `rust-error` | `parser: Expected identifier` | 45 |
| `oracle-error` | `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)` | 19 |
| `oracle-error` | `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)` | 16 |
| `oracle-error` | `oracle parse: Expecting (` | 14 |
| `unsupported-harness-shape` | `identify helper option is not supported yet` | 14 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 14, start: 13, end: 13, comments: []>` | 12 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }` | 10 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 15, start: 14, end: 14, comments: []>` | 9 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 14, start: 13, end: 13, comments: []>` | 8 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }` | 8 |
| `rust-error` | `parser: Expected RParen, got Comma (',')` | 8 |
| `rust-error` | `parser: Expected RParen, got Dot ('.')` | 8 |
| `rust-error` | `parser: Expected RParen, got Union ('UNION')` | 8 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 39, start: 38, end: 38, comments: []>` | 7 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>` | 7 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Into, value: "INTO", line: 1, col: 34, position: 33, quote_char: '\0' }` | 7 |
| `rust-error` | `parser: Expected RParen, got Order ('ORDER')` | 7 |
| `oracle-error` | `oracle parse: Expected AS after CAST` | 6 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 17, start: 16, end: 16, comments: []>` | 6 |
| `oracle-error` | `oracle token: Error tokenizing 'SELECT b'a'` | 6 |
| `rust-error` | `parser: Expected As, got Eof ('')` | 6 |
| `rust-error` | `parser: Expected LParen, got Unnest ('UNNEST')` | 6 |
| `rust-error` | `parser: Expected RParen, got Hour ('HOUR')` | 6 |
| `rust-error` | `parser: Expected RParen, got Identifier ('ARRAY[1')` | 6 |
| `rust-error` | `parser: Expected RParen, got Identifier ('PLAN')` | 6 |
| `rust-error` | `parser: Expected RParen, got With ('WITH')` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Comma, value: ",", line: 1, col: 10, position: 9, quote_char: '\0' }` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Group, value: "group", line: 1, col: 51, position: 50, quote_char: '\0' }` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Using, value: "USING", line: 1, col: 19, position: 18, quote_char: '\0' }` | 5 |
| `rust-error` | `parser: Expected RBracket, got Colon (':')` | 5 |
| `rust-error` | `parser: Expected RParen, got Day ('DAY')` | 5 |
| `rust-error` | `parser: Expected RParen, got Group ('GROUP')` | 5 |
| `rust-error` | `parser: Expected RParen, got Identifier ('device_data')` | 5 |
| `rust-error` | `parser: Expected RParen, got LParen ('(')` | 5 |
| `oracle-error` | `oracle parse: Expected ]` | 4 |

## Mismatch Signature Buckets

| Status | Signature | Count |
| --- | --- | ---: |
| `mismatch` | `DDL/create-table rendering` | 248 |
| `mismatch` | `missing AS or alias rendering` | 248 |
| `mismatch` | `CREATE` | 121 |
| `mismatch` | `SELECT` | 119 |
| `mismatch` | `SELECT operator multiply` | 80 |
| `mismatch` | `SELECT UNNEST()` | 74 |
| `mismatch` | `date/time rendering: DATE_ADD()` | 69 |
| `mismatch` | `case-only rendering difference` | 62 |
| `mismatch` | `date/time rendering: SELECT DATE_SUB()` | 49 |
| `mismatch` | `ALTER TABLE` | 46 |
| `mismatch` | `cast/type rendering: CAST()` | 42 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 34 |
| `mismatch` | `WITH` | 32 |
| `mismatch` | `date/time rendering: SELECT DATEADD()` | 32 |
| `mismatch` | `DECLARE` | 30 |
| `mismatch` | `date/time rendering: SELECT DATE_ADD()` | 29 |
| `mismatch` | `date/time rendering: SELECT DATE_FORMAT()` | 29 |
| `mismatch` | `REGEXP_EXTRACT()` | 28 |
| `mismatch` | `REGEXP_REPLACE()` | 27 |
| `mismatch` | `date/time rendering: CREATE` | 27 |
| `mismatch` | `json rendering: JSON_EXTRACT()` | 27 |
| `mismatch` | `date/time rendering: SELECT UNNEST()` | 26 |
| `mismatch` | `SELECT REGEXP_EXTRACT()` | 22 |
| `mismatch` | `cast/type rendering: SELECT EXTRACT()` | 22 |
| `mismatch` | `date/time rendering: EOMONTH()` | 20 |
| `mismatch` | `quote-style difference` | 20 |
| `mismatch` | `SELECT operator index` | 19 |
| `mismatch` | `cast/type rendering: SELECT TO_CHAR()` | 19 |
| `mismatch` | `A` | 18 |
| `mismatch` | `SELECT ARRAY_AGG()` | 18 |
| `mismatch` | `date/time rendering: SELECT DATE_TRUNC()` | 17 |
| `mismatch` | `date/time rendering: STR_TO_TIME()` | 17 |
| `mismatch` | `SELECT LAST_VALUE()` | 16 |
| `mismatch` | `date/time rendering: STR_TO_DATE()` | 16 |
| `mismatch` | `cast/type rendering: TS_OR_DS_TO_DATE()` | 15 |
| `mismatch` | `date/time rendering: TIME_TO_STR()` | 14 |
| `mismatch` | `ARRAY_LENGTH()` | 13 |
| `mismatch` | `cast/type rendering: WITH` | 13 |
| `mismatch` | `date/time rendering: DATEADD()` | 13 |
| `mismatch` | `REGEXP_SUBSTR()` | 12 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 925 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 541 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 392 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 297 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 258 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 227 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 226 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 195 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 193 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 193 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 161 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 129 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 125 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 125 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 120 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 119 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 103 |
| `match` | `tests/dialects/test_oracle.py` | `test_oracle` | 97 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 93 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 90 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 83 |
| `oracle-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 82 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 80 |
| `match` | `tests/dialects/test_dialect.py` | `test_trim` | 80 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 73 |
| `match` | `tests/dialects/test_databricks.py` | `test_databricks` | 71 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 71 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 70 |
| `mismatch` | `tests/dialects/test_spark.py` | `test_spark` | 70 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 69 |
| `match` | `tests/dialects/test_duckdb.py` | `test_time` | 67 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 67 |
| `match` | `tests/dialects/test_hive.py` | `test_joins_without_on` | 66 |
| `match` | `tests/dialects/test_exasol.py` | `test_scalar` | 65 |
| `match` | `tests/dialects/test_dialect.py` | `test_string_functions` | 64 |

## Bucket Examples

### `mismatch` `ALTER TABLE`

- `tests/test_transpile.py`:750 `test_alter` via `validate`: `ALTER TABLE integers ALTER i TYPE VARCHAR`
  - expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT`
  - actual: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- `tests/test_transpile.py`:754 `test_alter` via `validate`: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
  - expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
  - actual: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- `tests/dialects/test_athena.py`:88 `test_ddl` via `validate_identity`: `` ALTER TABLE `foo`.`bar` ADD COLUMN `end_ts` BIGINT ``
  - expected: `` ALTER TABLE `foo`.`bar` ADD COLUMN `end_ts` BIGINT ``
  - actual: `ALTER TABLE foo."bar" ADD COLUMN "end_ts" INTEGER`

### `mismatch` `CREATE`

- `tests/dialects/test_bigquery.py`:104 `test_bigquery` via `validate_identity`: `CREATE SCHEMA x DEFAULT COLLATE 'en'`
  - expected: `CREATE SCHEMA x`
  - actual: `CREATE SCHEMA x DEFAULT COLLATE 'en'`
- `tests/dialects/test_bigquery.py`:380 `test_bigquery` via `validate_identity`: `CREATE TEMPORARY FUNCTION FOO() RETURNS STRING LANGUAGE js AS 'return "Hello world!"'`
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

### `mismatch` `DECLARE`

- `tests/dialects/test_bigquery.py`:3827 `test_bignumeric` via `validate_all`: `DECLARE x BIGNUMERIC(20, 4)`
  - expected: `DECLARE x BIGNUMERIC(20, 4)`
  - actual: `DECLARE`
- `tests/dialects/test_bigquery.py`:3827 `test_bignumeric` via `validate_all`: `DECLARE x BIGNUMERIC(20, 4)`
  - expected: `DECLARE x BIGNUMERIC(20, 4)`
  - actual: `DECLARE`
- `tests/dialects/test_bigquery.py`:3835 `test_bignumeric` via `validate_all`: `DECLARE x BIGNUMERIC(76, 38)`
  - expected: `DECLARE x BIGNUMERIC(76, 38)`
  - actual: `DECLARE`

### `mismatch` `REGEXP_EXTRACT()`

- `tests/dialects/test_bigquery.py`:2897 `test_regexp_extract` via `validate_identity`: `REGEXP_EXTRACT(x, '(?<)')`
  - expected: `REGEXP_EXTRACT(x, '(?<)')`
  - actual: `REGEXP_SUBSTR(x, '(?<)')`
- `tests/dialects/test_hive.py`:900 `test_hive` via `validate_all`: `REGEXP_EXTRACT('abc', '(a)(b)(c)')`
  - expected: `REGEXP_EXTRACT('abc', '(a)(b)(c)')`
  - actual: `REGEXP_SUBSTR('abc', '(a)(b)(c)')`
- `tests/dialects/test_hive.py`:900 `test_hive` via `validate_all`: `REGEXP_EXTRACT('abc', '(a)(b)(c)')`
  - expected: `REGEXP_EXTRACT('abc', '(a)(b)(c)')`
  - actual: `REGEXP_SUBSTR('abc', '(a)(b)(c)')`

### `mismatch` `REGEXP_REPLACE()`

- `tests/dialects/test_exasol.py`:357 `test_stringFunctions` via `validate_all`: `REGEXP_REPLACE(subject, pattern, replacement, position, occurrence)`
  - expected: `REGEXP_REPLACE(subject, pattern, replacement, position, occurrence)`
  - actual: `REGEXP_REPLACE(subject, pattern, replacement, position)`
- `tests/dialects/test_exasol.py`:357 `test_stringFunctions` via `validate_all`: `REGEXP_REPLACE(subject, pattern, replacement, position, occurrence)`
  - expected: `REGEXP_REPLACE(subject, pattern, replacement, position, occurrence)`
  - actual: `REGEXP_REPLACE(subject, pattern, replacement, position)`
- `tests/dialects/test_exasol.py`:357 `test_stringFunctions` via `validate_all`: `REGEXP_REPLACE(subject, pattern, replacement, position, occurrence)`
  - expected: `REGEXP_REPLACE(subject, pattern, replacement, position, occurrence)`
  - actual: `REGEXP_REPLACE(subject, pattern, replacement, position)`

### `mismatch` `SELECT`

- `tests/test_transpile.py`:654 `test_comment_single_line_with_block_close` via `validate`: `SELECT c /* c1 /* c2 */ c3 */`
  - expected: `SELECT c /* c1 / * c2 * / c3 */`
  - actual: `SELECT c`
- `tests/test_transpile.py`:658 `test_comment_single_line_with_block_close` via `validate`: `SELECT c /* c1 /* c2 /* c3 */ */ */`
  - expected: `SELECT c /* c1 / * c2 / * c3 * / * / */`
  - actual: `SELECT c`
- `tests/test_transpile.py`:119 `test_comments` via `validate`: `select /* asfd /* asdf */ asdf */ 1`
  - expected: `/* asfd / * asdf * / asdf */ SELECT 1`
  - actual: `SELECT 1`

### `mismatch` `SELECT UNNEST()`

- `tests/dialects/test_bigquery.py`:1322 `test_bigquery` via `validate_all`: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS (x)`
  - expected: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS _t0`
  - actual: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS`
- `tests/dialects/test_bigquery.py`:1322 `test_bigquery` via `validate_all`: `SELECT * FROM UNNEST(['7', '14']) AS x`
  - expected: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS x`
  - actual: `SELECT * FROM UNNEST(['7', '14']) AS x`
- `tests/dialects/test_bigquery.py`:1322 `test_bigquery` via `validate_all`: `SELECT * FROM UNNEST(['7', '14']) AS x`
  - expected: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS x`
  - actual: `SELECT * FROM UNNEST(['7', '14']) AS x`

### `mismatch` `SELECT operator multiply`

- `tests/test_transpile.py`:131 `test_comments` via `validate`: `SELECT * FROM t1 /*x*/ UNION ALL SELECT * FROM t2`
  - expected: `SELECT * FROM t1 /* x */ UNION ALL SELECT * FROM t2`
  - actual: `SELECT * FROM t1 UNION ALL SELECT * FROM t2`
- `tests/test_transpile.py`:139 `test_comments` via `validate`: `SELECT * FROM t1 /*x*/ INTERSECT ALL SELECT * FROM t2`
  - expected: `SELECT * FROM t1 /* x */ INTERSECT ALL SELECT * FROM t2`
  - actual: `SELECT * FROM t1 INTERSECT ALL SELECT * FROM t2`
- `tests/test_transpile.py`:147 `test_comments` via `validate`: `SELECT * FROM a INNER /* comments */ JOIN b`
  - expected: `SELECT * FROM a /* comments */ INNER JOIN b`
  - actual: `SELECT * FROM a INNER JOIN b`

### `mismatch` `WITH`

- `tests/test_transpile.py`:544 `test_comments` via `validate`: `with x as ( SELECT * /* NOTE: LEFT JOIN because blah blah blah */ FROM a ) select * from x`
  - expected: `WITH x AS ( SELECT * /* NOTE: LEFT JOIN because blah blah blah */ FROM a ) SELECT * FROM x`
  - actual: `WITH x AS ( SELECT * FROM a ) SELECT * FROM x`
- `tests/test_transpile.py`:573 `test_comments` via `validate`: `with a as /* comment */ ( select * from b) select * from a`
  - expected: `WITH a /* comment */ AS (SELECT * FROM b) SELECT * FROM a`
  - actual: `WITH a AS (SELECT * FROM b) SELECT * FROM a`
- `tests/test_transpile.py`:607 `test_comments` via `validate`: `WITH x /* a */ AS ( SELECT 2 AS n /* b */ FROM (/* c */ SELECT /* c2 */ a /* d */ FROM t) AS x ) SELECT * FROM x /* e */ WHERE n >= (/* f */ SELECT MAX(x) FROM t) ORDER BY n /* g */ -- h`
  - expected: `WITH x /* a */ AS ( SELECT 2 AS n /* b */ FROM ( /* c */ /* c2 */ SELECT a /* d */ FROM t ) AS x ) SELECT * FROM x /* e */ WHERE n >= ( SELECT MAX(x) FROM t ) /* f */ ORDER BY n /* g */ /* h */ NULLS LAST`
  - actual: `WITH x AS ( SELECT 2 AS n FROM (SELECT a FROM t) AS x ) SELECT * FROM x WHERE n >= (SELECT MAX(x) FROM t) ORDER BY n NULLS LAST`

### `mismatch` `case-only rendering difference`

- `tests/test_transpile.py`:672 `test_types` via `validate`: `interval::int`
  - expected: `CAST(interval AS INTEGER)`
  - actual: `CAST(INTERVAL AS INTEGER)`
- `tests/test_transpile.py`:673 `test_types` via `validate`: `x::user_defined_type`
  - expected: `CAST(x AS user_defined_type)`
  - actual: `CAST(x AS USER_DEFINED_TYPE)`
- `tests/dialects/test_bigquery.py`:759 `test_bigquery` via `validate_all`: `TIMESTAMPDIFF(month, b, a)`
  - expected: `TIMESTAMPDIFF(month, b, A)`
  - actual: `TIMESTAMPDIFF(month, b, a)`

### `mismatch` `cast/type rendering: CAST()`

- `tests/dialects/test_bigquery.py`:223 `test_bigquery` via `validate_identity`: `CAST(x AS BIGNUMERIC)`
  - expected: `CAST(x AS BIGDECIMAL)`
  - actual: `CAST(x AS BIGNUMERIC)`
- `tests/dialects/test_bigquery.py`:1161 `test_bigquery` via `validate_all`: `cast(x as time format 'YYYY.MM.DD HH:MI:SSTZH')`
  - expected: `STR_TO_TIME(x, '%Y.%m.%d HH:%M:%S%ZH')`
  - actual: `CAST(x AS TIME)`
- `tests/dialects/test_clickhouse.py`:29 `test_clickhouse` via `validate_identity`: `CAST(x AS TINYBLOB)`
  - expected: `CAST(x AS BLOB)`
  - actual: `CAST(x AS TINYBLOB)`

### `mismatch` `cast/type rendering: SELECT CAST()`

- `tests/dialects/test_bigquery.py`:3843 `test_bignumeric` via `validate_all`: `SELECT CAST(1 AS BIGNUMERIC)`
  - expected: `SELECT CAST(1 AS BIGDECIMAL)`
  - actual: `SELECT CAST(1 AS BIGNUMERIC)`
- `tests/dialects/test_bigquery.py`:3843 `test_bignumeric` via `validate_all`: `SELECT CAST(1 AS BIGNUMERIC)`
  - expected: `SELECT CAST(1 AS BIGDECIMAL)`
  - actual: `SELECT CAST(1 AS BIGNUMERIC)`
- `tests/dialects/test_bigquery.py`:1120 `test_bigquery` via `validate_all`: `SELECT CAST(TIMESTAMP '2008-12-25 00:00:00+00:00' AS STRING FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM' AT TIME ZONE 'Asia/Kolkata') AS date_time_to_string`
  - expected: `SELECT CAST(CAST('2008-12-25 00:00:00+00:00' AS TIMESTAMP) AS TEXT FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM' AT TIME ZONE 'Asia/Kolkata') AS date_time_to_string`
  - actual: `SELECT CAST(CAST('2008-12-25 00:00:00+00:00' AS TIMESTAMP) AS TEXT FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM') AS date_time_to_string`

### `mismatch` `date/time rendering: CREATE`

- `tests/dialects/test_postgres.py`:1277 `test_ddl` via `validate_identity`: `CREATE CONSTRAINT TRIGGER my_trigger AFTER INSERT OR DELETE OR UPDATE OF col_a, col_b ON public.my_table DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION DO_STH()`
  - expected: `CREATE CONSTRAINT TRIGGER my_trigger`
  - actual: `CREATE CONSTRAINT TRIGGER my_trigger AFTER INSERT OR DELETE OR UPDATE OF col_a, col_b ON public.my_table DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION DO_STH()`
- `tests/dialects/test_postgres.py`:1320 `test_ddl` via `validate_identity`: `CREATE OR REPLACE FUNCTION foo(id UUID, OUT created_at TIMESTAMPTZ)`
  - expected: `CREATE OR REPLACE FUNCTION foo(id UUID, created_at TIMESTAMPTZ OUT)`
  - actual: `CREATE OR REPLACE FUNCTION foo(id UUID, OUT created_at TIMESTAMPTZ)`
- `tests/dialects/test_postgres.py`:1921 `test_postgres_create_trigger` via `validate_identity`: `CREATE TRIGGER check_update BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE FUNCTION CHECK_ACCOUNT_UPDATE()`
  - expected: `CREATE TRIGGER check_update`
  - actual: `CREATE TRIGGER check_update BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE FUNCTION CHECK_ACCOUNT_UPDATE()`

### `mismatch` `date/time rendering: DATE_ADD()`

- `tests/dialects/test_bigquery.py`:1510 `test_bigquery` via `validate_all`: `DATE_ADD(CURRENT_DATE(), INTERVAL -1 DAY)`
  - expected: `DATE(CURRENT_DATE, 'INTERVAL '-1' DAY')`
  - actual: `DATE_ADD(CURRENT_DATE, INTERVAL -1 DAY)`
- `tests/dialects/test_bigquery.py`:1510 `test_bigquery` via `validate_all`: `DATE_ADD(CURRENT_DATE(), INTERVAL -1 DAY)`
  - expected: `DATE(CURRENT_DATE, 'INTERVAL '-1' DAY')`
  - actual: `DATE_ADD(CURRENT_DATE, INTERVAL -1 DAY)`
- `tests/dialects/test_bigquery.py`:1510 `test_bigquery` via `validate_all`: `DATE_ADD(CURRENT_DATE(), INTERVAL -1 DAY)`
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

### `mismatch` `date/time rendering: SELECT DATE_FORMAT()`

- `tests/dialects/test_clickhouse.py`:617 `test_clickhouse` via `validate_all`: `SELECT DATE_FORMAT(NOW(), '%Y-%m-%d')`
  - expected: `SELECT DATE_FORMAT(CURRENT_TIMESTAMP, '%Y-%m-%d')`
  - actual: `SELECT STRFTIME('%Y-%m-%%w', CURRENT_TIMESTAMP)`
- `tests/dialects/test_exasol.py`:317 `test_stringFunctions` via `validate_all`: `SELECT DATE_FORMAT('2009-10-04 22:23:00', '%W %M %Y')`
  - expected: `SELECT DATE_FORMAT('2009-10-04 22:23:00', '%W %M %Y')`
  - actual: `SELECT STRFTIME('%W %M %Y', '2009-10-04 22:23:00')`
- `tests/dialects/test_mysql.py`:724 `test_date_format` via `validate_all`: `SELECT DATE_FORMAT('2017-06-15', '%Y')`
  - expected: `SELECT DATE_FORMAT('2017-06-15', '%Y')`
  - actual: `SELECT STRFTIME('%Y', '2017-06-15')`

### `mismatch` `date/time rendering: SELECT DATE_SUB()`

- `tests/dialects/test_bigquery.py`:478 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL '2' DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL 2 DAY)`
- `tests/dialects/test_bigquery.py`:478 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL '2' DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL 2 DAY)`
- `tests/dialects/test_bigquery.py`:485 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(DATE '2008-12-25', INTERVAL 5 DAY)`
  - expected: `SELECT DATE_SUB(DATE('2008-12-25'), INTERVAL '5' DAY)`
  - actual: `SELECT DATE_SUB(DATE('2008-12-25'), INTERVAL 5 DAY)`

### `mismatch` `missing AS or alias rendering`

- `tests/dialects/test_bigquery.py`:3819 `test_bignumeric` via `validate_all`: `SELECT BIGNUMERIC '1'`
  - expected: `SELECT CAST('1' AS BIGDECIMAL)`
  - actual: `SELECT BIGNUMERIC`
- `tests/dialects/test_bigquery.py`:3819 `test_bignumeric` via `validate_all`: `SELECT BIGNUMERIC '1'`
  - expected: `SELECT CAST('1' AS BIGDECIMAL)`
  - actual: `SELECT BIGNUMERIC`
- `tests/dialects/test_bigquery.py`:3819 `test_bignumeric` via `validate_all`: `SELECT BIGDECIMAL '1'`
  - expected: `SELECT CAST('1' AS BIGDECIMAL)`
  - actual: `SELECT BIGDECIMAL`

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

### `oracle-error` `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>`

- `tests/dialects/test_clickhouse.py`:842 `test_parameterization` via `validate_all`: `SELECT * FROM {table: Identifier}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {table: Identifier}`
- `tests/dialects/test_spark.py`:981 `test_spark` via `validate_all`: `SELECT * FROM {df}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {df}`
- `tests/dialects/test_spark.py`:981 `test_spark` via `validate_all`: `SELECT * FROM {df}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {df}`

### `oracle-error` `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 14, start: 13, end: 13, comments: []>`

- `tests/dialects/test_tsql.py`:1208 `test_ddl` via `validate_all`: `CREATE TABLE [#temptest] (name INTEGER)`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 14, start: 13, end: 13, comments: []>. Line 1, Col: 14. CREATE TABLE [#temptest] (name INTEGER)`
- `tests/dialects/test_tsql.py`:2121 `test_identifier_prefixes` via `validate_all`: `CREATE TABLE [#temptest] (name INTEGER)`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 14, start: 13, end: 13, comments: []>. Line 1, Col: 14. CREATE TABLE [#temptest] (name INTEGER)`
- `tests/dialects/test_tsql.py`:2121 `test_identifier_prefixes` via `validate_all`: `CREATE TABLE [#temptest] (name INTEGER)`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 14, start: 13, end: 13, comments: []>. Line 1, Col: 14. CREATE TABLE [#temptest] (name INTEGER)`

### `oracle-error` `oracle parse: Expecting (`

- `tests/dialects/test_clickhouse.py`:629 `test_clickhouse` via `validate_identity`: `ALTER TABLE visits DROP PARTITION 201901`
  - error: `ParseError: Expecting (. Line 1, Col: 40. ALTER TABLE visits DROP PARTITION 201901`
- `tests/dialects/test_clickhouse.py`:630 `test_clickhouse` via `validate_identity`: `ALTER TABLE visits DROP PARTITION ALL`
  - error: `ParseError: Expecting (. Line 1, Col: 37. ALTER TABLE visits DROP PARTITION ALL`
- `tests/dialects/test_clickhouse.py`:631 `test_clickhouse` via `validate_identity`: `ALTER TABLE visits DROP PARTITION tuple(toYYYYMM(toDate('2019-01-25')))`
  - error: `ParseError: Expecting (. Line 1, Col: 39. ALTER TABLE visits DROP PARTITION tuple(toYYYYMM(toDate('2019-01-25')))`

### `oracle-error` `oracle parse: Expecting )`

- `tests/test_transpile.py`:734 `test_with` via `validate`: `WITH A(filter) AS (VALUES 1, 2, 3) SELECT * FROM A WHERE filter >= 2`
  - error: `ParseError: Expecting ). Line 1, Col: 27. WITH A(filter) AS (VALUES 1, 2, 3) SELECT * FROM A WHERE filter >= 2`
- `tests/dialects/test_athena.py`:67 `test_ddl` via `validate_identity`: `` CREATE TABLE iceberg_table (`id` BIGINT, `data` STRING, category STRING) PARTITIONED BY (category, BUCKET(16, id)) LOCATION 's3://amzn-s3-demo-bucket/your-folder/' TBLPROPERTIES ('table_type'='ICEBERG', 'write_compression'='snappy') ``
  - error: `` ParseError: Expecting ). Line 1, Col: 32. CREATE TABLE iceberg_table (`id` BIGINT, `data` STRING, category STRING) PARTITIONED BY (category, BUCKET(16, id)) LOCATION 's3://am ``
- `tests/dialects/test_athena.py`:70 `test_ddl` via `validate_identity`: `` CREATE OR REPLACE TABLE iceberg_table (`id` BIGINT, `data` STRING, category STRING) PARTITIONED BY (category, BUCKET(16, id)) LOCATION 's3://amzn-s3-demo-bucket/your-folder/' TBLPROPERTIES ('table_type'='ICEBERG', 'write_compression'='snappy') ``
  - error: `` ParseError: Expecting ). Line 1, Col: 43. CREATE OR REPLACE TABLE iceberg_table (`id` BIGINT, `data` STRING, category STRING) PARTITIONED BY (category, BUCKET(16, id)) LOCATION 's3://am ``

### `oracle-error` `oracle parse: Invalid expression / Unexpected token`

- `tests/dialects/test_athena.py`:15 `test_athena` via `validate_identity`: `UNLOAD (SELECT name1, address1, comment1, key1 FROM table1) TO 's3://amzn-s3-demo-bucket/ partitioned/' WITH (format = 'TEXTFILE', partitioned_by = ARRAY['key1'])`
  - error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 103. UNLOAD (SELECT name1, address1, comment1, key1 FROM table1) TO 's3://amzn-s3-demo-bucket/ partitioned/' WITH (format = 'TEXTFILE', partitioned_by = ARRAY['key1'])`
- `tests/dialects/test_athena.py`:21 `test_athena` via `validate_identity`: `USING EXTERNAL FUNCTION some_function(input VARBINARY) RETURNS VARCHAR LAMBDA 'some-name' SELECT some_function(1)`
  - error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 5. USING EXTERNAL FUNCTION some_function(input VARBINARY) RETURNS VARCHAR LAMBDA`
- `tests/dialects/test_athena.py`:128 `test_ddl_quoting` via `validate_identity`: `` DROP TABLE `foo` ``
  - error: `` ParseError: Invalid expression / Unexpected token. Line 1, Col: 15. DROP TABLE `foo` ``

### `oracle-error` `oracle parse: Required keyword missing`

- `tests/test_transpile.py`:55 `test_alias` via `validate`: `SELECT x union`
  - error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.query.Union'>. Line 1, Col: 14. SELECT x union`
- `tests/test_transpile.py`:248 `test_comments` via `validate`: `/* multi line comment */ SELECT tbl.cola /* comment 1 */ + tbl.colb /* comment 2 */, CAST(x AS CHAR), # comment 3 y -- comment 4 FROM bar /* comment 5 */, tbl # comment 6`
  - error: `ParseError: Required keyword: 'this' missing for <class 'sqlglot.expressions.core.BitwiseXor'>. Line 8, Col: 42. tbl.cola /* comment 1 */ + tbl.colb /* comment 2 */, CAST(x AS CHAR), # comment 3 y -- comment 4 FROM bar /* comment 5 */,`
- `tests/test_transpile.py`:353 `test_comments` via `validate`: `SELECT 1 // hi this is a comment`
  - error: `ParseError: Required keyword: 'expression' missing for <class 'sqlglot.expressions.core.Div'>. Line 1, Col: 11. SELECT 1 // hi this is a comment`

### `oracle-error` `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)`

- `tests/dialects/test_clickhouse.py`:66 `test_clickhouse` via `validate_identity`: `countIf(x, y)`
  - error: `ParseError: The number of provided arguments (2) is greater than the maximum number of supported arguments (1). Line 1, Col: 13. countIf(x, y)`
- `tests/dialects/test_exasol.py`:152 `test_bits` via `validate_all`: `SELECT BIT_AND(x, 1)`
  - error: `ParseError: The number of provided arguments (2) is greater than the maximum number of supported arguments (1). Line 1, Col: 20. SELECT BIT_AND(x, 1)`
- `tests/dialects/test_exasol.py`:152 `test_bits` via `validate_all`: `SELECT BIT_AND(x, 1)`
  - error: `ParseError: The number of provided arguments (2) is greater than the maximum number of supported arguments (1). Line 1, Col: 20. SELECT BIT_AND(x, 1)`

### `oracle-error` `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)`

- `tests/dialects/test_bigquery.py`:709 `test_bigquery` via `validate_all`: `SELECT TIME(15, 30, 00)`
  - error: `ParseError: The number of provided arguments (3) is greater than the maximum number of supported arguments (2). Line 1, Col: 23. SELECT TIME(15, 30, 00)`
- `tests/dialects/test_bigquery.py`:709 `test_bigquery` via `validate_all`: `SELECT TIME(15, 30, 00)`
  - error: `ParseError: The number of provided arguments (3) is greater than the maximum number of supported arguments (2). Line 1, Col: 23. SELECT TIME(15, 30, 00)`
- `tests/dialects/test_bigquery.py`:709 `test_bigquery` via `validate_all`: `SELECT TIME(15, 30, 00)`
  - error: `ParseError: The number of provided arguments (3) is greater than the maximum number of supported arguments (2). Line 1, Col: 23. SELECT TIME(15, 30, 00)`

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

- `tests/dialects/test_bigquery.py`:2490 `test_rename_table` via `validate_all`: `ALTER TABLE db.t1 RENAME TO db.t2`
  - expected: `ALTER TABLE db.t1 RENAME TO db.t2`
  - error: `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }`
- `tests/dialects/test_bigquery.py`:2490 `test_rename_table` via `validate_all`: `ALTER TABLE db.t1 RENAME TO db.t2`
  - expected: `ALTER TABLE db.t1 RENAME TO db.t2`
  - error: `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }`
- `tests/dialects/test_doris.py`:243 `test_rename_table` via `validate_all`: `ALTER TABLE db.t1 RENAME TO db.t2`
  - expected: `ALTER TABLE db.t1 RENAME TO db.t2`
  - error: `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }`

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

- `tests/dialects/test_bigquery.py`:1487 `test_bigquery` via `validate_all`: `DELETE FROM db.t1 AS t1 WHERE NOT t1.c IN (SELECT db.t2.c FROM db.t2)`
  - expected: `DELETE FROM db.t1 AS t1 WHERE NOT t1.c IN (SELECT db.t2.c FROM db.t2)`
  - error: `ValueError: Parser error: Expected RParen, got Dot ('.') at line 1 col 56`
- `tests/dialects/test_bigquery.py`:1487 `test_bigquery` via `validate_all`: `DELETE FROM db.t1 AS t1 WHERE NOT t1.c IN (SELECT db.t2.c FROM db.t2)`
  - expected: `DELETE FROM db.t1 AS t1 WHERE NOT t1.c IN (SELECT db.t2.c FROM db.t2)`
  - error: `ValueError: Parser error: Expected RParen, got Dot ('.') at line 1 col 56`
- `tests/dialects/test_dialect.py`:3106 `test_merge` via `validate_all`: `MERGE INTO foo AS target USING (SELECT a, b FROM tbl) AS src ON src.a = target.a WHEN MATCHED AND target.a <> src.a THEN UPDATE SET target.b = 'FOO' WHEN NOT MATCHED THEN INSERT (target.a, target.b) VALUES (src.a, src.b)`
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
- `tests/dialects/test_dialect.py`:5279 `test_operator` via `validate_identity`: `SELECT 1 OPERATOR(pg_catalog.+) 2`
  - expected: `SELECT 1 OPERATOR(pg_catalog.+) 2`
  - error: `ValueError: Parser error: Expected identifier, got Plus ('+') at line 1 col 30`
- `tests/dialects/test_dialect.py`:3568 `test_truncate` via `validate_identity`: `TRUNCATE TABLE IF EXISTS db.schema.test`
  - expected: `TRUNCATE TABLE IF EXISTS db.schema.test`
  - error: `ValueError: Parser error: Expected identifier, got If ('IF') at line 1 col 16`

### `unsupported-harness-shape` `SQLGlot expects UnsupportedError`

- `tests/dialects/test_bigquery.py`:493 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
  - error: `SQLGlot expects UnsupportedError`
- `tests/dialects/test_bigquery.py`:493 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
  - error: `SQLGlot expects UnsupportedError`
- `tests/dialects/test_bigquery.py`:493 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
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

