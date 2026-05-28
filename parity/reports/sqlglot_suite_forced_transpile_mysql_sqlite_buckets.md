# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl`

Total rows: `15156`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 9315 |
| `mismatch` | 3354 |
| `oracle-error` | 1739 |
| `rust-error` | 611 |
| `unsupported-harness-shape` | 137 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `mysql` | `sqlite` | 9315 |
| `mismatch` | `mysql` | `sqlite` | 3354 |
| `oracle-error` | `mysql` | `sqlite` | 1739 |
| `rust-error` | `mysql` | `sqlite` | 611 |
| `unsupported-harness-shape` | `mysql` | `sqlite` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 6801 |
| `match` | `validate_identity` | 2417 |
| `mismatch` | `validate_all` | 2120 |
| `mismatch` | `validate_identity` | 1161 |
| `oracle-error` | `validate_identity` | 1135 |
| `oracle-error` | `validate_all` | 595 |
| `rust-error` | `validate_identity` | 346 |
| `rust-error` | `validate_all` | 261 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 97 |
| `mismatch` | `validate` | 73 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 4 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 685 |
| `match` | `CAST()` | 471 |
| `match` | `SELECT operator multiply` | 346 |
| `mismatch` | `CREATE TABLE` | 266 |
| `mismatch` | `SELECT` | 196 |
| `match` | `CREATE TABLE` | 192 |
| `mismatch` | `CREATE` | 182 |
| `oracle-error` | `SELECT` | 179 |
| `match` | `TRUNC()` | 164 |
| `match` | `CREATE` | 161 |
| `oracle-error` | `CREATE TABLE` | 148 |
| `oracle-error` | `SELECT operator multiply` | 135 |
| `mismatch` | `SHOW` | 108 |
| `match` | `SHOW` | 105 |
| `mismatch` | `SELECT UNNEST()` | 105 |
| `match` | `X` | 100 |
| `oracle-error` | `CAST()` | 96 |
| `match` | `WITH` | 94 |
| `mismatch` | `SELECT operator multiply` | 88 |
| `match` | `SELECT CAST()` | 85 |
| `mismatch` | `ALTER TABLE` | 85 |
| `match` | `ALTER TABLE` | 81 |
| `match` | `SELECT DATEDIFF()` | 79 |
| `match` | `SET` | 78 |
| `match` | `DATE_TRUNC()` | 73 |
| `match` | `SELECT UNNEST()` | 70 |
| `match` | `LOG()` | 67 |
| `mismatch` | `WITH` | 66 |
| `rust-error` | `SELECT` | 64 |
| `rust-error` | `SELECT operator multiply` | 64 |
| `match` | `GRANT` | 62 |
| `match` | `ANALYZE` | 60 |
| `oracle-error` | `SELECT OPTION()` | 60 |
| `match` | `A` | 58 |
| `match` | `TIME_STR_TO_TIME()` | 57 |
| `match` | `REGEXP_INSTR()` | 56 |
| `match` | `REVOKE` | 56 |
| `oracle-error` | `DATE_ADD()` | 54 |
| `mismatch` | `SELECT DATE_SUB()` | 52 |
| `match` | `FROM` | 51 |

## Rust/Oracle/Unsupported Error Buckets

| Status | Error Bucket | Count |
| --- | --- | ---: |
| `oracle-error` | `oracle parse: Invalid expression / Unexpected token` | 660 |
| `oracle-error` | `oracle parse: Expecting )` | 411 |
| `oracle-error` | `oracle parse: Required keyword missing` | 193 |
| `unsupported-harness-shape` | `SQLGlot expects UnsupportedError` | 119 |
| `oracle-error` | `oracle parse: Expected TYPE after CAST` | 108 |
| `oracle-error` | `oracle parse: INTERVAL expression expected but got '1'` | 72 |
| `rust-error` | `parser: Expected identifier` | 49 |
| `oracle-error` | `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)` | 46 |
| `oracle-error` | `oracle parse: Expected type` | 39 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>` | 33 |
| `oracle-error` | `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)` | 19 |
| `oracle-error` | `oracle parse: Expecting (` | 16 |
| `oracle-error` | `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)` | 16 |
| `unsupported-harness-shape` | `identify helper option is not supported yet` | 14 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }` | 10 |
| `oracle-error` | `KeyError: <class 'sqlglot.expressions.properties.PartitionByRangeProperty'>` | 8 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }` | 8 |
| `rust-error` | `parser: Expected RParen, got Comma (',')` | 8 |
| `rust-error` | `parser: Expected RParen, got Union ('UNION')` | 8 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>` | 7 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Into, value: "INTO", line: 1, col: 34, position: 33, quote_char: '\0' }` | 7 |
| `rust-error` | `parser: Expected RParen, got Dot ('.')` | 7 |
| `oracle-error` | `KeyError: <class 'sqlglot.expressions.properties.PartitionByListProperty'>` | 6 |
| `oracle-error` | `oracle parse: Expected AS after CAST` | 6 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 17, start: 16, end: 16, comments: []>` | 6 |
| `oracle-error` | `oracle token: Error tokenizing 'SELECT b'a'` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Transaction, value: "TRANSACTION", line: 1, col: 7, position: 6, quote_char: '\0' }` | 6 |
| `rust-error` | `parser: Expected LParen, got Unnest ('UNNEST')` | 6 |
| `rust-error` | `parser: Expected RParen, got Identifier ('ARRAY[1')` | 6 |
| `rust-error` | `parser: Expected RParen, got With ('WITH')` | 6 |
| `oracle-error` | `oracle parse: Expected ]` | 5 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 14, start: 13, end: 13, comments: []>` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Group, value: "group", line: 1, col: 51, position: 50, quote_char: '\0' }` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Using, value: "USING", line: 1, col: 19, position: 18, quote_char: '\0' }` | 5 |
| `rust-error` | `parser: Expected RBracket, got Colon (':')` | 5 |
| `rust-error` | `parser: Expected RParen, got Day ('DAY')` | 5 |
| `rust-error` | `parser: Expected RParen, got Identifier ('device_data')` | 5 |
| `rust-error` | `parser: Expected RParen, got Order ('ORDER')` | 5 |
| `oracle-error` | `oracle parse: INTERVAL expression expected but got '20'` | 4 |
| `oracle-error` | `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (1)` | 4 |

## Mismatch Signature Buckets

| Status | Signature | Count |
| --- | --- | ---: |
| `mismatch` | `missing AS or alias rendering` | 288 |
| `mismatch` | `DDL/create-table rendering` | 263 |
| `mismatch` | `SELECT` | 145 |
| `mismatch` | `SHOW` | 107 |
| `mismatch` | `CREATE` | 105 |
| `mismatch` | `ALTER TABLE` | 76 |
| `mismatch` | `SELECT UNNEST()` | 75 |
| `mismatch` | `SELECT operator multiply` | 75 |
| `mismatch` | `missing quoted identifier` | 55 |
| `mismatch` | `date/time rendering: SELECT DATE_SUB()` | 52 |
| `mismatch` | `cast/type rendering: CAST()` | 42 |
| `mismatch` | `cast/type rendering: SELECT TO_CHAR()` | 41 |
| `mismatch` | `date/time rendering: SELECT TO_TIMESTAMP()` | 39 |
| `mismatch` | `quote-style difference` | 36 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 34 |
| `mismatch` | `WITH` | 33 |
| `mismatch` | `date/time rendering: SELECT DATEADD()` | 32 |
| `mismatch` | `REPLACE()` | 31 |
| `mismatch` | `date/time rendering: STR_TO_TIME()` | 30 |
| `mismatch` | `REGEXP_EXTRACT()` | 29 |
| `mismatch` | `REGEXP_REPLACE()` | 27 |
| `mismatch` | `SELECT FORMAT()` | 27 |
| `mismatch` | `date/time rendering: DATE_ADD()` | 27 |
| `mismatch` | `json rendering: JSON_EXTRACT()` | 27 |
| `mismatch` | `A` | 25 |
| `mismatch` | `case-only rendering difference` | 24 |
| `mismatch` | `date/time rendering: CREATE` | 23 |
| `mismatch` | `SELECT REGEXP_EXTRACT()` | 22 |
| `mismatch` | `cast/type rendering: SELECT EXTRACT()` | 22 |
| `mismatch` | `SELECT operator index` | 21 |
| `mismatch` | `X` | 20 |
| `mismatch` | `date/time rendering: EOMONTH()` | 20 |
| `mismatch` | `date/time rendering: SELECT UNNEST()` | 17 |
| `mismatch` | `json rendering: SELECT JSON_VALUE()` | 17 |
| `mismatch` | `cast/type rendering: TS_OR_DS_TO_DATE()` | 15 |
| `mismatch` | `json rendering: SELECT JSON_EXTRACT_PATH_TEXT()` | 15 |
| `mismatch` | `date/time rendering: SELECT DATE_TRUNC()` | 14 |
| `mismatch` | `ARRAY_LENGTH()` | 13 |
| `mismatch` | `cast/type rendering: WITH` | 13 |
| `mismatch` | `date/time rendering: DATEADD()` | 13 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 847 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 514 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 334 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 322 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 242 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 230 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 218 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 217 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 198 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 184 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 173 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 145 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 120 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 120 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 118 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 113 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 100 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_oracle` | 91 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 89 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_tsql.py` | `test_option` | 86 |
| `match` | `tests/dialects/test_dialect.py` | `test_trim` | 80 |
| `oracle-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 80 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 79 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 78 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_ddl` | 77 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 73 |
| `match` | `tests/dialects/test_mysql.py` | `test_identity` | 72 |
| `match` | `tests/dialects/test_databricks.py` | `test_databricks` | 70 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 69 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 69 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 68 |
| `match` | `tests/dialects/test_exasol.py` | `test_scalar` | 68 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 67 |
| `match` | `tests/dialects/test_dialect.py` | `test_set_operators` | 66 |

## Bucket Examples

### `mismatch` `ALTER TABLE`

- `tests/test_transpile.py`:750 `test_alter` via `validate`: `ALTER TABLE integers ALTER i TYPE VARCHAR`
  - expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT`
  - actual: `ALTER TABLE integers ALTER i TYPE VARCHAR`
- `tests/test_transpile.py`:754 `test_alter` via `validate`: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
  - expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
  - actual: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
- `tests/dialects/test_hive.py`:178 `test_ddl` via `validate_identity`: `ALTER TABLE x PARTITION(y = z) ADD COLUMN a VARCHAR(10)`
  - expected: `ALTER TABLE x PARTITION(y = z) ADD COLUMN a TEXT(10)`
  - actual: `ALTER TABLE x PARTITION(y = z) ADD COLUMN a VARCHAR(10)`

### `mismatch` `CREATE`

- `tests/dialects/test_athena.py`:121 `test_ddl_quoting` via `validate_identity`: `CREATE VIEW "foo" AS SELECT "id" FROM "tbl"`
  - expected: `CREATE VIEW "foo" AS SELECT 'id' FROM "tbl"`
  - actual: `CREATE VIEW "foo" AS SELECT "id" FROM "tbl"`
- `tests/dialects/test_athena.py`:136 `test_ddl_quoting` via `validate_identity`: `CREATE VIEW "foo" AS SELECT "id" FROM "tbl"`
  - expected: `CREATE VIEW "foo" AS SELECT 'id' FROM "tbl"`
  - actual: `CREATE VIEW "foo" AS SELECT "id" FROM "tbl"`
- `tests/dialects/test_bigquery.py`:104 `test_bigquery` via `validate_identity`: `CREATE SCHEMA x DEFAULT COLLATE 'en'`
  - expected: `CREATE SCHEMA x`
  - actual: `CREATE SCHEMA x DEFAULT COLLATE 'en'`

### `mismatch` `DDL/create-table rendering`

- `tests/test_transpile.py`:374 `test_comments` via `validate`: `-- comment4 CREATE TABLE db.tba AS SELECT a, b, c FROM tb_01 WHERE -- comment5 a = 1 AND b = 2 --comment6 -- and c = 1 -- comment7 ;`
  - expected: `/* comment4 */ CREATE TABLE db.tba AS SELECT a, b, c FROM tb_01 WHERE a /* comment5 */ = 1 AND b = 2 /* comment6 */ /* and c = 1 */ /* comment7 */`
  - actual: `CREATE TABLE db.tba AS SELECT a, b, c FROM tb_01 WHERE a = 1 AND b = 2`
- `tests/dialects/test_athena.py`:43 `test_ddl` via `validate_identity`: `CREATE EXTERNAL TABLE foo (id INT) COMMENT 'test comment'`
  - expected: `CREATE TABLE foo (id INTEGER)`
  - actual: `CREATE EXTERNAL TABLE foo (id INT) COMMENT 'test comment'`
- `tests/dialects/test_athena.py`:44 `test_ddl` via `validate_identity`: `CREATE EXTERNAL TABLE george.t (id INT COMMENT 'foo \\ bar') LOCATION 's3://my-bucket/'`
  - expected: `CREATE TABLE george.t (id INTEGER COMMENT 'foo \ bar')`
  - actual: `CREATE EXTERNAL TABLE george.t (id INT COMMENT 'foo \\ bar') LOCATION 's3://my-bucket/'`

### `mismatch` `REGEXP_EXTRACT()`

- `tests/dialects/test_bigquery.py`:2897 `test_regexp_extract` via `validate_identity`: `REGEXP_EXTRACT(x, '(?<)')`
  - expected: `REGEXP_EXTRACT(x, '(?<)')`
  - actual: `REGEXP_SUBSTR(x, '(?<)')`
- `tests/dialects/test_bigquery.py`:2898 `test_regexp_extract` via `validate_identity`: `` REGEXP_EXTRACT(`foo`, 'bar: (.+?)', 1, 1) ``
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

### `mismatch` `SHOW`

- `tests/dialects/test_duckdb.py`:2494 `test_show_tables` via `validate_identity`: `SHOW TABLES`
  - expected: ``
  - actual: `SHOW TABLES`
- `tests/dialects/test_duckdb.py`:2495 `test_show_tables` via `validate_identity`: `SHOW TABLES FROM my_schema`
  - expected: ``
  - actual: `SHOW TABLES FROM my_schema`
- `tests/dialects/test_duckdb.py`:2496 `test_show_tables` via `validate_identity`: `SHOW TABLES FROM my_database`
  - expected: ``
  - actual: `SHOW TABLES FROM my_database`

### `mismatch` `WITH`

- `tests/test_transpile.py`:544 `test_comments` via `validate`: `with x as ( SELECT * /* NOTE: LEFT JOIN because blah blah blah */ FROM a ) select * from x`
  - expected: `WITH x AS ( SELECT * /* NOTE: LEFT JOIN because blah blah blah */ FROM a ) SELECT * FROM x`
  - actual: `WITH x AS ( SELECT * FROM a ) SELECT * FROM x`
- `tests/test_transpile.py`:573 `test_comments` via `validate`: `with a as /* comment */ ( select * from b) select * from a`
  - expected: `WITH a /* comment */ AS (SELECT * FROM b) SELECT * FROM a`
  - actual: `WITH a AS (SELECT * FROM b) SELECT * FROM a`
- `tests/test_transpile.py`:607 `test_comments` via `validate`: `WITH x /* a */ AS ( SELECT 2 AS n /* b */ FROM (/* c */ SELECT /* c2 */ a /* d */ FROM t) AS x ) SELECT * FROM x /* e */ WHERE n >= (/* f */ SELECT MAX(x) FROM t) ORDER BY n /* g */ -- h`
  - expected: `WITH x /* a */ AS ( SELECT 2 AS n /* b */ FROM ( /* c */ /* c2 */ SELECT a /* d */ FROM t ) AS x ) SELECT * FROM x /* e */ WHERE n >= ( SELECT MAX(x) FROM t ) /* f */ ORDER BY n /* g */ /* h */`
  - actual: `WITH x AS ( SELECT 2 AS n FROM (SELECT a FROM t) AS x ) SELECT * FROM x WHERE n >= (SELECT MAX(x) FROM t) ORDER BY n`

### `mismatch` `cast/type rendering: CAST()`

- `tests/dialects/test_bigquery.py`:1161 `test_bigquery` via `validate_all`: `cast(x as time format 'YYYY.MM.DD HH:MI:SSTZH')`
  - expected: `STR_TO_TIME(x, 'YYYY.MM.DD HH:MI:SSTZH')`
  - actual: `CAST(x AS TIME)`
- `tests/dialects/test_clickhouse.py`:73 `test_clickhouse` via `validate_identity`: `CAST(x AS Enum('hello' = 1, 'world' = 2))`
  - expected: `CAST(x AS ENUM('hello' = 1, 'world' = 2))`
  - actual: `CAST(x AS ENUM)`
- `tests/dialects/test_clickhouse.py`:74 `test_clickhouse` via `validate_identity`: `CAST(x AS Enum('hello', 'world'))`
  - expected: `CAST(x AS ENUM('hello', 'world'))`
  - actual: `CAST(x AS ENUM)`

### `mismatch` `cast/type rendering: SELECT CAST()`

- `tests/dialects/test_bigquery.py`:1120 `test_bigquery` via `validate_all`: `SELECT CAST(TIMESTAMP '2008-12-25 00:00:00+00:00' AS STRING FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM' AT TIME ZONE 'Asia/Kolkata') AS date_time_to_string`
  - expected: `SELECT CAST(CAST('2008-12-25 00:00:00+00:00' AS TIMESTAMPTZ) AS TEXT FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM' AT TIME ZONE 'Asia/Kolkata') AS date_time_to_string`
  - actual: `SELECT CAST(CAST('2008-12-25 00:00:00+00:00' AS TIMESTAMPTZ) AS TEXT FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM') AS date_time_to_string`
- `tests/dialects/test_bigquery.py`:3041 `test_cast_format_with_parentheses` via `validate_identity`: `SELECT CAST('2026-03-24' AS STRING FORMAT ('YYYY'))`
  - expected: `SELECT CAST('2026-03-24' AS TEXT FORMAT 'YYYY')`
  - actual: `SELECT CAST('2026-03-24' AS TEXT)`
- `tests/dialects/test_bigquery.py`:3046 `test_cast_format_with_parentheses` via `validate_identity`: `SELECT CAST(date AS STRING FORMAT ('YYYY')) FROM (SELECT DATE('2026-03-24') AS date)`
  - expected: `SELECT CAST(date AS TEXT FORMAT 'YYYY') FROM (SELECT DATE('2026-03-24') AS date)`
  - actual: `SELECT CAST(date AS TEXT) FROM (SELECT DATE('2026-03-24') AS date)`

### `mismatch` `cast/type rendering: SELECT TO_CHAR()`

- `tests/dialects/test_dremio.py`:101 `test_time_mapping` via `validate_all`: `SELECT TO_CHAR(CAST('2025-06-24 12:34:56' AS TIMESTAMP), 'yyyy-mm-dd hh24:mi:ss')`
  - expected: `SELECT CAST(CAST('2025-06-24 12:34:56' AS TIMESTAMPTZ) AS TEXT)`
  - actual: `SELECT STRFTIME('yyyy-mm-dd hh24:mi:ss', CAST('2025-06-24 12:34:56' AS TIMESTAMPTZ))`
- `tests/dialects/test_dremio.py`:101 `test_time_mapping` via `validate_all`: `SELECT TO_CHAR(CAST('2025-06-24 12:34:56' AS TIMESTAMP), 'YYYY-MM-DD HH24:MI:SS')`
  - expected: `SELECT CAST(CAST('2025-06-24 12:34:56' AS TIMESTAMPTZ) AS TEXT)`
  - actual: `SELECT STRFTIME('YYYY-MM-DD HH24:MI:SS', CAST('2025-06-24 12:34:56' AS TIMESTAMPTZ))`
- `tests/dialects/test_dremio.py`:101 `test_time_mapping` via `validate_all`: `SELECT TO_CHAR(CAST('2025-06-24 12:34:56' AS TIMESTAMP), 'YYYY-MM-DD HH24:MI:SS')`
  - expected: `SELECT CAST(CAST('2025-06-24 12:34:56' AS TIMESTAMPTZ) AS TEXT)`
  - actual: `SELECT STRFTIME('YYYY-MM-DD HH24:MI:SS', CAST('2025-06-24 12:34:56' AS TIMESTAMPTZ))`

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

### `mismatch` `date/time rendering: SELECT DATE_SUB()`

- `tests/dialects/test_bigquery.py`:478 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, '2', DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL '2' DAY)`
- `tests/dialects/test_bigquery.py`:478 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, '2', DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL '2' DAY)`
- `tests/dialects/test_bigquery.py`:485 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(DATE '2008-12-25', INTERVAL 5 DAY)`
  - expected: `SELECT DATE_SUB(DATE('2008-12-25'), '5', DAY)`
  - actual: `SELECT DATE_SUB(DATE('2008-12-25'), INTERVAL '5' DAY)`

### `mismatch` `date/time rendering: SELECT TO_TIMESTAMP()`

- `tests/dialects/test_oracle.py`:299 `test_oracle` via `validate_all`: `SELECT TO_TIMESTAMP('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
  - expected: `SELECT TO_TIMESTAMP('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
  - actual: `SELECT STR_TO_DATE('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
- `tests/dialects/test_oracle.py`:299 `test_oracle` via `validate_all`: `SELECT TO_TIMESTAMP('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
  - expected: `SELECT TO_TIMESTAMP('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
  - actual: `SELECT STR_TO_DATE('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
- `tests/dialects/test_oracle.py`:353 `test_oracle` via `validate_identity`: `SELECT TO_TIMESTAMP('05 Dec 2000 10:00 AM', 'DD Mon YYYY HH12:MI AM')`
  - expected: `SELECT TO_TIMESTAMP('05 Dec 2000 10:00 AM', 'DD Mon YYYY HH12:MI AM')`
  - actual: `SELECT STR_TO_DATE('05 Dec 2000 10:00 AM', 'DD Mon YYYY HH12:MI AM')`

### `mismatch` `date/time rendering: STR_TO_TIME()`

- `tests/test_transpile.py`:791 `test_time` via `validate`: `STR_TO_TIME('x', 'y')`
  - expected: `STR_TO_TIME('x', 'y')`
  - actual: `STR_TO_DATE('x', 'y')`
- `tests/test_transpile.py`:806 `test_time` via `validate`: `STR_TO_TIME(x, 'y')`
  - expected: `STR_TO_TIME(x, 'y')`
  - actual: `STR_TO_DATE(x, 'y')`
- `tests/test_transpile.py`:811 `test_time` via `validate`: `STR_TO_TIME(x, 'yyyy-MM-dd HH:mm:ss')`
  - expected: `STR_TO_TIME(x, 'yyyy-MM-dd HH:mm:ss')`
  - actual: `STR_TO_DATE(x, 'yyyy-MM-dd HH:mm:ss')`

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

### `mismatch` `missing quoted identifier`

- `tests/dialects/test_athena.py`:62 `test_ddl` via `validate_identity`: `` CREATE EXTERNAL TABLE `my_table` (`a7` ARRAY<DATE>) ROW FORMAT SERDE 'a' STORED AS INPUTFORMAT 'b' OUTPUTFORMAT 'c' LOCATION 'd' TBLPROPERTIES ('e'='f') ``
  - expected: `CREATE TABLE "my_table" ("a7" ARRAY<DATE>)`
  - actual: `` CREATE EXTERNAL TABLE `my_table` (`a7` ARRAY<DATE>) ROW FORMAT SERDE 'a' STORED AS INPUTFORMAT 'b' OUTPUTFORMAT 'c' LOCATION 'd' TBLPROPERTIES ('e'='f') ``
- `tests/dialects/test_athena.py`:109 `test_ddl_quoting` via `validate_identity`: `` CREATE EXTERNAL TABLE `foo` (`id` INT) LOCATION 's3://foo/' ``
  - expected: `CREATE TABLE "foo" ("id" INTEGER)`
  - actual: `` CREATE EXTERNAL TABLE `foo` (`id` INT) LOCATION 's3://foo/' ``
- `tests/dialects/test_bigquery.py`:2818 `test_json_extract` via `validate_identity`: `JSON_VALUE(doc, '$. a b c .d')`
  - expected: `JSON_VALUE(doc, '$." a b c ".d')`
  - actual: `doc -> '$. a b c .d'`

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

### `oracle-error` `KeyError: <class 'sqlglot.expressions.properties.PartitionByRangeProperty'>`

- `tests/dialects/test_doris.py`:149 `test_partition` via `validate_identity`: `` CREATE TABLE test_table (c1 INT, c2 DATE) PARTITION BY RANGE (`c2`) (PARTITION `p201701` VALUES LESS THAN ('2017-02-01'), PARTITION `p201702` VALUES LESS THAN ('2017-03-01')) ``
  - error: `KeyError: <class 'sqlglot.expressions.properties.PartitionByRangeProperty'>`
- `tests/dialects/test_mysql.py`:249 `test_ddl` via `validate_identity`: `CREATE TABLE t (id INT, created_at DATE) PARTITION BY RANGE (id) (PARTITION p0 VALUES LESS THAN (10), PARTITION p1 VALUES LESS THAN (20), PARTITION p2 VALUES LESS THAN (MAXVALUE))`
  - error: `KeyError: <class 'sqlglot.expressions.properties.PartitionByRangeProperty'>`
- `tests/dialects/test_mysql.py`:252 `test_ddl` via `validate_identity`: `CREATE TABLE t (id INT, name VARCHAR(50)) PARTITION BY RANGE (id) (PARTITION p0 VALUES LESS THAN (100))`
  - error: `KeyError: <class 'sqlglot.expressions.properties.PartitionByRangeProperty'>`

### `oracle-error` `oracle parse: Expected TYPE after CAST`

- `tests/dialects/test_bigquery.py`:237 `test_bigquery` via `validate_identity`: `CAST(x AS RECORD)`
  - error: `ParseError: Expected TYPE after CAST. Line 1, Col: 16. CAST(x AS RECORD)`
- `tests/dialects/test_bigquery.py`:376 `test_bigquery` via `validate_identity`: `SELECT CAST(1 AS BYTEINT)`
  - error: `ParseError: Expected TYPE after CAST. Line 1, Col: 24. SELECT CAST(1 AS BYTEINT)`
- `tests/dialects/test_bigquery.py`:1271 `test_bigquery` via `validate_all`: `CAST(a AS BYTES)`
  - error: `ParseError: Expected TYPE after CAST. Line 1, Col: 15. CAST(a AS BYTES)`

### `oracle-error` `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>`

- `tests/dialects/test_clickhouse.py`:842 `test_parameterization` via `validate_all`: `SELECT * FROM {table: Identifier}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {table: Identifier}`
- `tests/dialects/test_spark.py`:981 `test_spark` via `validate_all`: `SELECT * FROM {df}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {df}`
- `tests/dialects/test_spark.py`:981 `test_spark` via `validate_all`: `SELECT * FROM {df}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {df}`

### `oracle-error` `oracle parse: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>`

- `tests/test_transpile.py`:55 `test_alias` via `validate`: `SELECT x from`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13. SELECT x from`
- `tests/test_transpile.py`:55 `test_alias` via `validate`: `SELECT x join`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13. SELECT x join`
- `tests/dialects/test_redshift.py`:345 `test_identity` via `validate_identity`: `SELECT * FROM #x`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>. Line 1, Col: 13. SELECT * FROM #x`

### `oracle-error` `oracle parse: Expected type`

- `tests/test_transpile.py`:673 `test_types` via `validate`: `x::user_defined_type`
  - error: `ParseError: Expected type. Line 1, Col: 20. x::user_defined_type`
- `tests/dialects/test_clickhouse.py`:863 `test_signed_and_unsigned_types` via `validate_all`: `pow(2, 32)::UInt8`
  - error: `ParseError: Expected type. Line 1, Col: 17. pow(2, 32)::UInt8`
- `tests/dialects/test_clickhouse.py`:863 `test_signed_and_unsigned_types` via `validate_all`: `pow(2, 32)::UInt16`
  - error: `ParseError: Expected type. Line 1, Col: 18. pow(2, 32)::UInt16`

### `oracle-error` `oracle parse: Expecting (`

- `tests/dialects/test_clickhouse.py`:629 `test_clickhouse` via `validate_identity`: `ALTER TABLE visits DROP PARTITION 201901`
  - error: `ParseError: Expecting (. Line 1, Col: 40. ALTER TABLE visits DROP PARTITION 201901`
- `tests/dialects/test_clickhouse.py`:630 `test_clickhouse` via `validate_identity`: `ALTER TABLE visits DROP PARTITION ALL`
  - error: `ParseError: Expecting (. Line 1, Col: 37. ALTER TABLE visits DROP PARTITION ALL`
- `tests/dialects/test_clickhouse.py`:631 `test_clickhouse` via `validate_identity`: `ALTER TABLE visits DROP PARTITION tuple(toYYYYMM(toDate('2019-01-25')))`
  - error: `ParseError: Expecting (. Line 1, Col: 39. ALTER TABLE visits DROP PARTITION tuple(toYYYYMM(toDate('2019-01-25')))`

### `oracle-error` `oracle parse: Expecting )`

- `tests/test_transpile.py`:518 `test_comments` via `validate`: `-- comment SOME_FUNC(arg IGNORE NULLS) OVER (PARTITION BY foo ORDER BY bla) AS col`
  - error: `ParseError: Expecting ). Line 2, Col: 20. -- comment SOME_FUNC(arg IGNORE NULLS) OVER (PARTITION BY foo ORDER BY bla) AS col`
- `tests/dialects/test_bigquery.py`:3731 `test_approx_quantiles` via `validate_identity`: `APPROX_QUANTILES(x, 2 IGNORE NULLS)`
  - error: `ParseError: Expecting ). Line 1, Col: 28. APPROX_QUANTILES(x, 2 IGNORE NULLS)`
- `tests/dialects/test_bigquery.py`:3803 `test_approx_quantiles_to_duckdb` via `validate_all`: `APPROX_QUANTILES(x, 2 IGNORE NULLS)`
  - error: `ParseError: Expecting ). Line 1, Col: 28. APPROX_QUANTILES(x, 2 IGNORE NULLS)`

### `oracle-error` `oracle parse: INTERVAL expression expected but got '1'`

- `tests/dialects/test_clickhouse.py`:420 `test_clickhouse` via `validate_all`: `DATE_ADD('DAY', 1, x)`
  - error: `ParseError: INTERVAL expression expected but got '1'`
- `tests/dialects/test_clickhouse.py`:420 `test_clickhouse` via `validate_all`: `DATE_ADD(DAY, 1, x)`
  - error: `ParseError: INTERVAL expression expected but got '1'`
- `tests/dialects/test_clickhouse.py`:420 `test_clickhouse` via `validate_all`: `DATE_ADD(DAY, 1, x)`
  - error: `ParseError: INTERVAL expression expected but got '1'`

### `oracle-error` `oracle parse: Invalid expression / Unexpected token`

- `tests/test_transpile.py`:654 `test_comment_single_line_with_block_close` via `validate`: `SELECT c /* c1 /* c2 */ c3 */`
  - error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 28. SELECT c /* c1 /* c2 */ c3 */`
- `tests/dialects/test_athena.py`:15 `test_athena` via `validate_identity`: `UNLOAD (SELECT name1, address1, comment1, key1 FROM table1) TO 's3://amzn-s3-demo-bucket/ partitioned/' WITH (format = 'TEXTFILE', partitioned_by = ARRAY['key1'])`
  - error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 103. UNLOAD (SELECT name1, address1, comment1, key1 FROM table1) TO 's3://amzn-s3-demo-bucket/ partitioned/' WITH (format = 'TEXTFILE', partitioned_by = ARRAY['key1'])`
- `tests/dialects/test_athena.py`:21 `test_athena` via `validate_identity`: `USING EXTERNAL FUNCTION some_function(input VARBINARY) RETURNS VARCHAR LAMBDA 'some-name' SELECT some_function(1)`
  - error: `ParseError: Invalid expression / Unexpected token. Line 1, Col: 5. USING EXTERNAL FUNCTION some_function(input VARBINARY) RETURNS VARCHAR LAMBDA`

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

### `rust-error` `parser: Expected RParen, got Union ('UNION')`

- `tests/dialects/test_presto.py`:1054 `test_presto` via `validate_all`: `WITH RECURSIVE t(n) AS (VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < 100 ) SELECT SUM(n) FROM t`
  - expected: `WITH RECURSIVE t(n) AS (VALUES("1") UNION ALL SELECT n + 1 FROM t WHERE n < 100) SELECT SUM(n) FROM t`
  - error: `ValueError: Parser error: Expected RParen, got Union ('UNION') at line 1 col 36`
- `tests/dialects/test_presto.py`:1054 `test_presto` via `validate_all`: `WITH RECURSIVE t(n) AS (VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < 100 ) SELECT SUM(n) FROM t`
  - expected: `WITH RECURSIVE t(n) AS (VALUES("1") UNION ALL SELECT n + 1 FROM t WHERE n < 100) SELECT SUM(n) FROM t`
  - error: `ValueError: Parser error: Expected RParen, got Union ('UNION') at line 1 col 36`
- `tests/dialects/test_tsql.py`:641 `test_option` via `validate_identity`: `SELECT * FROM Table1 OPTION(CONCAT UNION)`
  - expected: `SELECT * FROM Table1 AS OPTION`
  - error: `ValueError: Parser error: Expected RParen, got Union ('UNION') at line 1 col 36`

### `rust-error` `parser: Expected identifier`

- `tests/test_transpile.py`:127 `test_comments` via `validate`: `SELECT c AS /* foo */ (a, b, c) FROM t`
  - expected: `SELECT c AS (a, b, c) /* foo */ FROM t`
  - error: `ValueError: Parser error: Expected identifier, got LParen ('(') at line 1 col 23`
- `tests/dialects/test_bigquery.py`:188 `test_bigquery` via `validate_identity`: `SELECT * FROM foo.bar.25_`
  - expected: `SELECT * FROM foo.bar."25_"`
  - error: `ValueError: Parser error: Expected identifier, got Number ('25') at line 1 col 23`
- `tests/dialects/test_bigquery.py`:189 `test_bigquery` via `validate_identity`: `SELECT * FROM foo.bar.25x a`
  - expected: `SELECT * FROM foo.bar."25x" AS a`
  - error: `ValueError: Parser error: Expected identifier, got Number ('25') at line 1 col 23`

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

