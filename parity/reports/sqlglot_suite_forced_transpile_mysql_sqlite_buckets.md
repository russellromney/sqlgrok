# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl`

Total rows: `15170`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 8682 |
| `mismatch` | 3863 |
| `oracle-error` | 1741 |
| `rust-error` | 745 |
| `unsupported-harness-shape` | 139 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `mysql` | `sqlite` | 8682 |
| `mismatch` | `mysql` | `sqlite` | 3863 |
| `oracle-error` | `mysql` | `sqlite` | 1741 |
| `rust-error` | `mysql` | `sqlite` | 745 |
| `unsupported-harness-shape` | `mysql` | `sqlite` | 139 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 6265 |
| `mismatch` | `validate_all` | 2576 |
| `match` | `validate_identity` | 2323 |
| `mismatch` | `validate_identity` | 1211 |
| `oracle-error` | `validate_identity` | 1135 |
| `oracle-error` | `validate_all` | 597 |
| `rust-error` | `validate_identity` | 391 |
| `rust-error` | `validate_all` | 350 |
| `unsupported-harness-shape` | `validate_all` | 124 |
| `match` | `validate` | 94 |
| `mismatch` | `validate` | 76 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 4 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 670 |
| `match` | `CAST()` | 464 |
| `match` | `SELECT operator multiply` | 337 |
| `mismatch` | `CREATE TABLE` | 250 |
| `match` | `CREATE TABLE` | 201 |
| `mismatch` | `SELECT` | 190 |
| `mismatch` | `CREATE` | 182 |
| `oracle-error` | `SELECT` | 181 |
| `match` | `TRUNC()` | 162 |
| `match` | `CREATE` | 161 |
| `oracle-error` | `CREATE TABLE` | 148 |
| `oracle-error` | `SELECT operator multiply` | 135 |
| `mismatch` | `SHOW` | 108 |
| `match` | `SHOW` | 105 |
| `mismatch` | `SELECT UNNEST()` | 105 |
| `match` | `X` | 104 |
| `oracle-error` | `CAST()` | 96 |
| `match` | `WITH` | 93 |
| `mismatch` | `SELECT operator multiply` | 89 |
| `rust-error` | `SELECT` | 87 |
| `mismatch` | `ALTER TABLE` | 85 |
| `match` | `ALTER TABLE` | 81 |
| `match` | `SELECT DATEDIFF()` | 79 |
| `match` | `SET` | 78 |
| `match` | `SELECT CAST()` | 73 |
| `rust-error` | `SELECT operator multiply` | 72 |
| `match` | `DATE_TRUNC()` | 71 |
| `match` | `SELECT UNNEST()` | 70 |
| `match` | `LOG()` | 67 |
| `mismatch` | `WITH` | 65 |
| `match` | `GRANT` | 62 |
| `match` | `ANALYZE` | 60 |
| `oracle-error` | `SELECT OPTION()` | 60 |
| `match` | `A` | 58 |
| `match` | `REGEXP_INSTR()` | 56 |
| `match` | `REVOKE` | 56 |
| `oracle-error` | `DATE_ADD()` | 54 |
| `rust-error` | `CREATE TABLE` | 53 |
| `mismatch` | `SELECT DATE_SUB()` | 52 |
| `match` | `INSERT` | 50 |

## Rust/Oracle/Unsupported Error Buckets

| Status | Error Bucket | Count |
| --- | --- | ---: |
| `oracle-error` | `oracle parse: Invalid expression / Unexpected token` | 660 |
| `oracle-error` | `oracle parse: Expecting )` | 411 |
| `oracle-error` | `oracle parse: Required keyword missing` | 195 |
| `unsupported-harness-shape` | `SQLGlot expects UnsupportedError` | 121 |
| `oracle-error` | `oracle parse: Expected TYPE after CAST` | 108 |
| `oracle-error` | `oracle parse: INTERVAL expression expected but got '1'` | 72 |
| `rust-error` | `parser: Expected identifier` | 47 |
| `oracle-error` | `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)` | 46 |
| `oracle-error` | `oracle parse: Expected type` | 39 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>` | 33 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }` | 21 |
| `oracle-error` | `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)` | 19 |
| `oracle-error` | `oracle parse: Expecting (` | 16 |
| `oracle-error` | `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)` | 16 |
| `unsupported-harness-shape` | `identify helper option is not supported yet` | 14 |
| `rust-error` | `parser: Expected RParen, got Identifier ('2008-12-25 15:30:00+00')` | 13 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }` | 10 |
| `oracle-error` | `KeyError: <class 'sqlglot.expressions.properties.PartitionByRangeProperty'>` | 8 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }` | 8 |
| `rust-error` | `parser: Expected RParen, got Comma (',')` | 8 |
| `rust-error` | `parser: Expected RParen, got Union ('UNION')` | 8 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>` | 7 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Into, value: "INTO", line: 1, col: 34, position: 33, quote_char: '\0' }` | 7 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Where, value: "WHERE", line: 1, col: 34, position: 33, quote_char: '\0' }` | 7 |
| `rust-error` | `parser: Expected RParen, got Dot ('.')` | 7 |
| `oracle-error` | `KeyError: <class 'sqlglot.expressions.properties.PartitionByListProperty'>` | 6 |
| `oracle-error` | `oracle parse: Expected AS after CAST` | 6 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 17, start: 16, end: 16, comments: []>` | 6 |
| `oracle-error` | `oracle token: Error tokenizing 'SELECT b'a'` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: As, value: "AS", line: 1, col: 33, position: 32, quote_char: '\0' }` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 24, position: 23, quote_char: '\0' }` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Outer, value: "OUTER", line: 1, col: 24, position: 23, quote_char: '\0' }` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Transaction, value: "TRANSACTION", line: 1, col: 7, position: 6, quote_char: '\0' }` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Xor, value: "xor", line: 1, col: 8, position: 7, quote_char: '\0' }` | 6 |
| `rust-error` | `parser: Expected Join, got Identifier ('APPLY')` | 6 |
| `rust-error` | `parser: Expected LParen, got Unnest ('UNNEST')` | 6 |
| `rust-error` | `parser: Expected RParen, got Identifier ('ARRAY[1')` | 6 |
| `rust-error` | `parser: Expected RParen, got LParen ('(')` | 6 |
| `rust-error` | `parser: Expected RParen, got With ('WITH')` | 6 |
| `oracle-error` | `oracle parse: Expected ]` | 5 |

## Mismatch Signature Buckets

| Status | Signature | Count |
| --- | --- | ---: |
| `mismatch` | `missing AS or alias rendering` | 347 |
| `mismatch` | `DDL/create-table rendering` | 247 |
| `mismatch` | `case-only rendering difference` | 165 |
| `mismatch` | `SELECT` | 139 |
| `mismatch` | `SHOW` | 107 |
| `mismatch` | `CREATE` | 105 |
| `mismatch` | `ALTER TABLE` | 76 |
| `mismatch` | `SELECT operator multiply` | 76 |
| `mismatch` | `SELECT UNNEST()` | 75 |
| `mismatch` | `missing quoted identifier` | 59 |
| `mismatch` | `date/time rendering: SELECT DATE_SUB()` | 52 |
| `mismatch` | `cast/type rendering: CAST()` | 49 |
| `mismatch` | `cast/type rendering: SELECT TO_CHAR()` | 41 |
| `mismatch` | `date/time rendering: SELECT TO_TIMESTAMP()` | 39 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 37 |
| `mismatch` | `date/time rendering: TIME_STR_TO_TIME()` | 36 |
| `mismatch` | `quote-style difference` | 34 |
| `mismatch` | `WITH` | 32 |
| `mismatch` | `date/time rendering: SELECT DATEADD()` | 32 |
| `mismatch` | `REPLACE()` | 31 |
| `mismatch` | `date/time rendering: STR_TO_TIME()` | 30 |
| `mismatch` | `REGEXP_EXTRACT()` | 29 |
| `mismatch` | `LEVENSHTEIN()` | 28 |
| `mismatch` | `MEDIAN()` | 28 |
| `mismatch` | `REGEXP_REPLACE()` | 27 |
| `mismatch` | `SELECT FORMAT()` | 27 |
| `mismatch` | `date/time rendering: DATE_ADD()` | 27 |
| `mismatch` | `json rendering: JSON_EXTRACT()` | 27 |
| `mismatch` | `date/time rendering: MONTH()` | 26 |
| `mismatch` | `A` | 25 |
| `mismatch` | `date/time rendering: YEAR()` | 24 |
| `mismatch` | `date/time rendering: CREATE` | 23 |
| `mismatch` | `SELECT REGEXP_EXTRACT()` | 22 |
| `mismatch` | `cast/type rendering: SELECT EXTRACT()` | 22 |
| `mismatch` | `SELECT operator index` | 21 |
| `mismatch` | `SHA256()` | 21 |
| `mismatch` | `date/time rendering: DAY()` | 21 |
| `mismatch` | `SELECT COUNT_IF()` | 20 |
| `mismatch` | `X` | 20 |
| `mismatch` | `date/time rendering: EOMONTH()` | 20 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 815 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 429 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 350 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 301 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 275 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 222 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 196 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 194 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 185 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 182 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 161 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 159 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 118 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 117 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 109 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 100 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 100 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 95 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 86 |
| `oracle-error` | `tests/dialects/test_tsql.py` | `test_option` | 86 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 82 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 81 |
| `oracle-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 80 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_ddl` | 74 |
| `mismatch` | `tests/dialects/test_presto.py` | `test_presto` | 73 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 73 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 73 |
| `match` | `tests/dialects/test_mysql.py` | `test_identity` | 72 |
| `match` | `tests/dialects/test_databricks.py` | `test_databricks` | 68 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 68 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 67 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 66 |
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

- `tests/dialects/test_bigquery.py`:1323 `test_bigquery` via `validate_all`: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS (x)`
  - expected: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS _t0`
  - actual: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS`
- `tests/dialects/test_bigquery.py`:1323 `test_bigquery` via `validate_all`: `SELECT * FROM UNNEST(['7', '14']) AS x`
  - expected: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS x`
  - actual: `SELECT * FROM UNNEST(['7', '14']) AS x`
- `tests/dialects/test_bigquery.py`:1323 `test_bigquery` via `validate_all`: `SELECT * FROM UNNEST(['7', '14']) AS x`
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

- `tests/dialects/test_duckdb.py`:2502 `test_show_tables` via `validate_identity`: `SHOW TABLES`
  - expected: ``
  - actual: `SHOW TABLES`
- `tests/dialects/test_duckdb.py`:2503 `test_show_tables` via `validate_identity`: `SHOW TABLES FROM my_schema`
  - expected: ``
  - actual: `SHOW TABLES FROM my_schema`
- `tests/dialects/test_duckdb.py`:2504 `test_show_tables` via `validate_identity`: `SHOW TABLES FROM my_database`
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

### `mismatch` `case-only rendering difference`

- `tests/test_transpile.py`:704 `test_extract` via `validate`: `extract(week from current_date + 2)`
  - expected: `EXTRACT(WEEK FROM CURRENT_DATE + 2)`
  - actual: `EXTRACT(WEEK FROM current_date + 2)`
- `tests/test_transpile.py`:672 `test_types` via `validate`: `interval::int`
  - expected: `CAST(interval AS INTEGER)`
  - actual: `CAST(INTERVAL AS INTEGER)`
- `tests/dialects/test_bigquery.py`:1366 `test_bigquery` via `validate_all`: `current_time`
  - expected: `CURRENT_TIME`
  - actual: `current_time`

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

- `tests/dialects/test_bigquery.py`:479 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, '2', DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL 2 DAY)`
- `tests/dialects/test_bigquery.py`:479 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, '2', DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL 2 DAY)`
- `tests/dialects/test_bigquery.py`:486 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(DATE '2008-12-25', INTERVAL 5 DAY)`
  - expected: `SELECT DATE_SUB(DATE('2008-12-25'), '5', DAY)`
  - actual: `SELECT DATE_SUB(DATE('2008-12-25'), INTERVAL 5 DAY)`

### `mismatch` `date/time rendering: SELECT TO_TIMESTAMP()`

- `tests/dialects/test_oracle.py`:306 `test_oracle` via `validate_all`: `SELECT TO_TIMESTAMP('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
  - expected: `SELECT TO_TIMESTAMP('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
  - actual: `SELECT STR_TO_DATE('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
- `tests/dialects/test_oracle.py`:306 `test_oracle` via `validate_all`: `SELECT TO_TIMESTAMP('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
  - expected: `SELECT TO_TIMESTAMP('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
  - actual: `SELECT STR_TO_DATE('2024-12-12 12:12:12.000000', 'YYYY-MM-DD HH24:MI:SS.FF6')`
- `tests/dialects/test_oracle.py`:360 `test_oracle` via `validate_identity`: `SELECT TO_TIMESTAMP('05 Dec 2000 10:00 AM', 'DD Mon YYYY HH12:MI AM')`
  - expected: `SELECT TO_TIMESTAMP('05 Dec 2000 10:00 AM', 'DD Mon YYYY HH12:MI AM')`
  - actual: `SELECT STR_TO_DATE('05 Dec 2000 10:00 AM', 'DD Mon YYYY HH12:MI AM')`

### `mismatch` `date/time rendering: TIME_STR_TO_TIME()`

- `tests/test_transpile.py`:830 `test_time` via `validate`: `TIME_STR_TO_TIME(x, 'America/Los_Angeles')`
  - expected: `x`
  - actual: `TIME_STR_TO_TIME(x, 'America/Los_Angeles')`
- `tests/dialects/test_dialect.py`:846 `test_time` via `validate_all`: `TIME_STR_TO_TIME('2020-01-01 12:13:14.123-08:00', 'America/Los_Angeles')`
  - expected: `'2020-01-01 12:13:14.123-08:00'`
  - actual: `TIME_STR_TO_TIME('2020-01-01 12:13:14.123-08:00', 'America/Los_Angeles')`
- `tests/dialects/test_dialect.py`:846 `test_time` via `validate_all`: `TIME_STR_TO_TIME('2020-01-01 12:13:14.123-08:00', 'America/Los_Angeles')`
  - expected: `'2020-01-01 12:13:14.123-08:00'`
  - actual: `TIME_STR_TO_TIME('2020-01-01 12:13:14.123-08:00', 'America/Los_Angeles')`

### `mismatch` `missing AS or alias rendering`

- `tests/test_transpile.py`:836 `test_time` via `validate`: `TIME_TO_TIME_STR(x)`
  - expected: `CAST(x AS TEXT)`
  - actual: `TIME_TO_TIME_STR(x)`
- `tests/dialects/test_bigquery.py`:3820 `test_bignumeric` via `validate_all`: `SELECT BIGNUMERIC '1'`
  - expected: `SELECT CAST('1' AS BIGDECIMAL)`
  - actual: `SELECT BIGNUMERIC`
- `tests/dialects/test_bigquery.py`:3820 `test_bignumeric` via `validate_all`: `SELECT BIGNUMERIC '1'`
  - expected: `SELECT CAST('1' AS BIGDECIMAL)`
  - actual: `SELECT BIGNUMERIC`

### `mismatch` `missing quoted identifier`

- `tests/dialects/test_athena.py`:62 `test_ddl` via `validate_identity`: `` CREATE EXTERNAL TABLE `my_table` (`a7` ARRAY<DATE>) ROW FORMAT SERDE 'a' STORED AS INPUTFORMAT 'b' OUTPUTFORMAT 'c' LOCATION 'd' TBLPROPERTIES ('e'='f') ``
  - expected: `CREATE TABLE "my_table" ("a7" ARRAY<DATE>)`
  - actual: `` CREATE EXTERNAL TABLE `my_table` (`a7` ARRAY<DATE>) ROW FORMAT SERDE 'a' STORED AS INPUTFORMAT 'b' OUTPUTFORMAT 'c' LOCATION 'd' TBLPROPERTIES ('e'='f') ``
- `tests/dialects/test_athena.py`:109 `test_ddl_quoting` via `validate_identity`: `` CREATE EXTERNAL TABLE `foo` (`id` INT) LOCATION 's3://foo/' ``
  - expected: `CREATE TABLE "foo" ("id" INTEGER)`
  - actual: `` CREATE EXTERNAL TABLE `foo` (`id` INT) LOCATION 's3://foo/' ``
- `tests/dialects/test_bigquery.py`:2819 `test_json_extract` via `validate_identity`: `JSON_VALUE(doc, '$. a b c .d')`
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

- `tests/dialects/test_bigquery.py`:238 `test_bigquery` via `validate_identity`: `CAST(x AS RECORD)`
  - error: `ParseError: Expected TYPE after CAST. Line 1, Col: 16. CAST(x AS RECORD)`
- `tests/dialects/test_bigquery.py`:377 `test_bigquery` via `validate_identity`: `SELECT CAST(1 AS BYTEINT)`
  - error: `ParseError: Expected TYPE after CAST. Line 1, Col: 24. SELECT CAST(1 AS BYTEINT)`
- `tests/dialects/test_bigquery.py`:1272 `test_bigquery` via `validate_all`: `CAST(a AS BYTES)`
  - error: `ParseError: Expected TYPE after CAST. Line 1, Col: 15. CAST(a AS BYTES)`

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
- `tests/dialects/test_bigquery.py`:3732 `test_approx_quantiles` via `validate_identity`: `APPROX_QUANTILES(x, 2 IGNORE NULLS)`
  - error: `ParseError: Expecting ). Line 1, Col: 28. APPROX_QUANTILES(x, 2 IGNORE NULLS)`
- `tests/dialects/test_bigquery.py`:3804 `test_approx_quantiles_to_duckdb` via `validate_all`: `APPROX_QUANTILES(x, 2 IGNORE NULLS)`
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

- `tests/dialects/test_bigquery.py`:710 `test_bigquery` via `validate_all`: `SELECT TIME(15, 30, 00)`
  - error: `ParseError: The number of provided arguments (3) is greater than the maximum number of supported arguments (2). Line 1, Col: 23. SELECT TIME(15, 30, 00)`
- `tests/dialects/test_bigquery.py`:710 `test_bigquery` via `validate_all`: `SELECT TIME(15, 30, 00)`
  - error: `ParseError: The number of provided arguments (3) is greater than the maximum number of supported arguments (2). Line 1, Col: 23. SELECT TIME(15, 30, 00)`
- `tests/dialects/test_bigquery.py`:710 `test_bigquery` via `validate_all`: `SELECT TIME(15, 30, 00)`
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

### `rust-error` `parser: Expected RParen, got Identifier ('2008-12-25 15:30:00+00')`

- `tests/dialects/test_bigquery.py`:886 `test_bigquery` via `validate_all`: `SELECT TIMESTAMP_ADD(TIMESTAMP "2008-12-25 15:30:00+00", INTERVAL 10 MINUTE)`
  - expected: `SELECT TIMESTAMP_ADD(CAST('2008-12-25 15:30:00+00' AS TIMESTAMPTZ), INTERVAL '10' MINUTE)`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('2008-12-25 15:30:00+00') at line 1 col 32`
- `tests/dialects/test_bigquery.py`:886 `test_bigquery` via `validate_all`: `SELECT TIMESTAMP_ADD(TIMESTAMP "2008-12-25 15:30:00+00", INTERVAL 10 MINUTE)`
  - expected: `SELECT TIMESTAMP_ADD(CAST('2008-12-25 15:30:00+00' AS TIMESTAMPTZ), INTERVAL '10' MINUTE)`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('2008-12-25 15:30:00+00') at line 1 col 32`
- `tests/dialects/test_bigquery.py`:886 `test_bigquery` via `validate_all`: `SELECT TIMESTAMP_ADD(TIMESTAMP "2008-12-25 15:30:00+00", INTERVAL 10 MINUTE)`
  - expected: `SELECT TIMESTAMP_ADD(CAST('2008-12-25 15:30:00+00' AS TIMESTAMPTZ), INTERVAL '10' MINUTE)`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('2008-12-25 15:30:00+00') at line 1 col 32`

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

