# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl`

Total rows: `15156`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 10869 |
| `mismatch` | 1835 |
| `oracle-error` | 1739 |
| `rust-error` | 576 |
| `unsupported-harness-shape` | 137 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `mysql` | `sqlite` | 10869 |
| `mismatch` | `mysql` | `sqlite` | 1835 |
| `oracle-error` | `mysql` | `sqlite` | 1739 |
| `rust-error` | `mysql` | `sqlite` | 576 |
| `unsupported-harness-shape` | `mysql` | `sqlite` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 7992 |
| `match` | `validate_identity` | 2770 |
| `oracle-error` | `validate_identity` | 1135 |
| `mismatch` | `validate_all` | 936 |
| `mismatch` | `validate_identity` | 836 |
| `oracle-error` | `validate_all` | 595 |
| `rust-error` | `validate_identity` | 318 |
| `rust-error` | `validate_all` | 254 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 107 |
| `mismatch` | `validate` | 63 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 4 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 701 |
| `match` | `CAST()` | 489 |
| `match` | `SELECT operator multiply` | 349 |
| `match` | `CREATE TABLE` | 302 |
| `match` | `SHOW` | 195 |
| `mismatch` | `CREATE TABLE` | 195 |
| `match` | `CREATE` | 193 |
| `mismatch` | `SELECT` | 180 |
| `oracle-error` | `SELECT` | 179 |
| `match` | `TRUNC()` | 164 |
| `mismatch` | `CREATE` | 149 |
| `oracle-error` | `CREATE TABLE` | 148 |
| `oracle-error` | `SELECT operator multiply` | 135 |
| `match` | `X` | 104 |
| `oracle-error` | `CAST()` | 96 |
| `mismatch` | `SELECT UNNEST()` | 95 |
| `match` | `WITH` | 94 |
| `match` | `SELECT CAST()` | 92 |
| `mismatch` | `SELECT operator multiply` | 85 |
| `match` | `ALTER TABLE` | 83 |
| `mismatch` | `ALTER TABLE` | 83 |
| `match` | `SELECT UNNEST()` | 80 |
| `match` | `SELECT DATEDIFF()` | 79 |
| `match` | `SET` | 78 |
| `match` | `DATE_TRUNC()` | 77 |
| `match` | `LOG()` | 67 |
| `mismatch` | `WITH` | 66 |
| `match` | `JSON_EXTRACT()` | 64 |
| `rust-error` | `SELECT` | 64 |
| `rust-error` | `SELECT operator multiply` | 64 |
| `match` | `GRANT` | 62 |
| `match` | `ANALYZE` | 60 |
| `oracle-error` | `SELECT OPTION()` | 60 |
| `match` | `A` | 58 |
| `match` | `REGEXP_REPLACE()` | 57 |
| `match` | `TIME_STR_TO_TIME()` | 57 |
| `match` | `REGEXP_INSTR()` | 56 |
| `match` | `REVOKE` | 56 |
| `match` | `SELECT TO_TIMESTAMP()` | 55 |
| `oracle-error` | `DATE_ADD()` | 54 |

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
| `mismatch` | `missing AS or alias rendering` | 211 |
| `mismatch` | `DDL/create-table rendering` | 181 |
| `mismatch` | `SELECT` | 129 |
| `mismatch` | `CREATE` | 100 |
| `mismatch` | `ALTER TABLE` | 74 |
| `mismatch` | `SELECT operator multiply` | 72 |
| `mismatch` | `SELECT UNNEST()` | 64 |
| `mismatch` | `missing quoted identifier` | 53 |
| `mismatch` | `quote-style difference` | 37 |
| `mismatch` | `WITH` | 33 |
| `mismatch` | `A` | 25 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 24 |
| `mismatch` | `date/time rendering: CREATE` | 23 |
| `mismatch` | `cast/type rendering: CAST()` | 21 |
| `mismatch` | `X` | 20 |
| `mismatch` | `case-only rendering difference` | 20 |
| `mismatch` | `date/time rendering: SELECT UNNEST()` | 17 |
| `mismatch` | `SHOW` | 13 |
| `mismatch` | `cast/type rendering: WITH` | 13 |
| `mismatch` | `json rendering: SELECT JSON_VALUE()` | 12 |
| `mismatch` | `json rendering: WITH` | 12 |
| `mismatch` | `date/time rendering: DATE_ADD()` | 11 |
| `mismatch` | `'FOO'` | 10 |
| `mismatch` | `COPY` | 9 |
| `mismatch` | `--` | 8 |
| `mismatch` | `DS` | 8 |
| `mismatch` | `PIVOT` | 8 |
| `mismatch` | `SELECT COUNT()` | 8 |
| `mismatch` | `U&'HELLO` | 8 |
| `mismatch` | `empty actual output` | 8 |
| `mismatch` | `whitespace-only difference` | 8 |
| `mismatch` | `DESCRIBE` | 7 |
| `mismatch` | `SELECT FLOOR()` | 7 |
| `mismatch` | `SELECT TO_ARRAY()` | 7 |
| `mismatch` | `cast/type rendering: SELECT operator cast` | 7 |
| `mismatch` | `DELETE` | 6 |
| `mismatch` | `FROM` | 6 |
| `mismatch` | `INSERT` | 6 |
| `mismatch` | `POSITION()` | 6 |
| `mismatch` | `SELECT CEIL()` | 6 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 1030 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 605 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 401 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 301 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 273 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 252 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 250 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 228 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 166 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 157 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 141 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 139 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 128 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 127 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 108 |
| `match` | `tests/dialects/test_oracle.py` | `test_oracle` | 103 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 102 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 101 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 98 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 98 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 89 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_tsql.py` | `test_option` | 86 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 85 |
| `match` | `tests/dialects/test_dialect.py` | `test_trim` | 80 |
| `oracle-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 80 |
| `match` | `tests/dialects/test_databricks.py` | `test_databricks` | 77 |
| `match` | `tests/dialects/test_mysql.py` | `test_identity` | 76 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `match` | `tests/dialects/test_exasol.py` | `test_scalar` | 73 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_ddl` | 69 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 69 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 67 |
| `match` | `tests/dialects/test_dialect.py` | `test_set_operators` | 66 |
| `match` | `tests/dialects/test_hive.py` | `test_joins_without_on` | 66 |
| `match` | `tests/dialects/test_duckdb.py` | `test_time` | 65 |
| `match` | `tests/dialects/test_presto.py` | `test_time` | 65 |

## Bucket Examples

### `mismatch` `A`

- `tests/test_transpile.py`:683 `test_not_range` via `validate`: `a NOT IN (1, 2)`
  - expected: `NOT a IN (1, 2)`
  - actual: `a NOT IN (1, 2)`
- `tests/test_transpile.py`:684 `test_not_range` via `validate`: `a IS NOT NULL`
  - expected: `NOT a IS NULL`
  - actual: `a IS NOT NULL`
- `tests/dialects/test_duckdb.py`:888 `test_duckdb` via `validate_all`: `a # b`
  - expected: `a /* b */`
  - actual: `a`

### `mismatch` `ALTER TABLE`

- `tests/test_transpile.py`:754 `test_alter` via `validate`: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
  - expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
  - actual: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo`
- `tests/dialects/test_hive.py`:178 `test_ddl` via `validate_identity`: `ALTER TABLE x PARTITION(y = z) ADD COLUMN a VARCHAR(10)`
  - expected: `ALTER TABLE x PARTITION(y = z) ADD COLUMN a TEXT(10)`
  - actual: `ALTER TABLE x PARTITION(y = z) ADD COLUMN a VARCHAR(10)`
- `tests/dialects/test_hive.py`:179 `test_ddl` via `validate_identity`: `ALTER TABLE x CHANGE a a VARCHAR(10)`
  - expected: `ALTER TABLE x CHANGE COLUMN a a TEXT(10)`
  - actual: `ALTER TABLE x CHANGE a a VARCHAR(10)`

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
- `tests/dialects/test_athena.py`:50 `test_ddl` via `validate_identity`: `CREATE EXTERNAL TABLE foo (id INT, val STRING) CLUSTERED BY (id, val) INTO 10 BUCKETS`
  - expected: `CREATE TABLE foo (id INTEGER, val TEXT)`
  - actual: `CREATE EXTERNAL TABLE foo (id INT, val STRING) CLUSTERED BY (id, val) INTO 10 BUCKETS`
- `tests/dialects/test_athena.py`:152 `test_ddl_quoting` via `validate_identity`: `CREATE TABLE "foo" AS WITH "foo" AS (SELECT "a", "b" FROM "bar") SELECT * FROM "foo"`
  - expected: `CREATE TABLE "foo" AS WITH "foo" AS (SELECT 'a', 'b' FROM "bar") SELECT * FROM "foo"`
  - actual: `CREATE TABLE "foo" AS WITH "foo" AS (SELECT "a", "b" FROM "bar") SELECT * FROM "foo"`

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

- `tests/dialects/test_bigquery.py`:1322 `test_bigquery` via `validate_all`: `SELECT * FROM UNNEST(['7', '14']) AS x`
  - expected: `SELECT * FROM UNNEST(ARRAY('7', '14')) AS x`
  - actual: `SELECT * FROM UNNEST(['7', '14']) AS x`
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

- `tests/dialects/test_mysql.py`:1369 `test_show_db_like_or_where_sql` via `validate_identity`: `SHOW TABLE STATUS`
  - expected: ``
  - actual: `SHOW TABLE STATUS`
- `tests/dialects/test_mysql.py`:1373 `test_show_db_like_or_where_sql` via `validate_identity`: `SHOW TABLE STATUS FROM db_name`
  - expected: ``
  - actual: `SHOW TABLE STATUS FROM db_name`
- `tests/dialects/test_mysql.py`:1377 `test_show_db_like_or_where_sql` via `validate_identity`: `SHOW TABLE STATUS LIKE '%foo%'`
  - expected: ``
  - actual: `SHOW TABLE STATUS LIKE '%foo%'`

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

### `mismatch` `X`

- `tests/dialects/test_bigquery.py`:344 `test_bigquery` via `validate_identity`: `x <> ""`
  - expected: `x <> ''`
  - actual: `x <> ""`
- `tests/dialects/test_bigquery.py`:348 `test_bigquery` via `validate_identity`: `x <> """"""`
  - expected: `x <> '""'`
  - actual: `x <> """"""`
- `tests/dialects/test_clickhouse.py`:348 `test_clickhouse` via `validate_all`: `x = any(array[1])`
  - expected: `x = ANY(ARRAY(1))`
  - actual: `x = ANY(ARRAY[1])`

### `mismatch` `case-only rendering difference`

- `tests/test_transpile.py`:672 `test_types` via `validate`: `interval::int`
  - expected: `CAST(interval AS INTEGER)`
  - actual: `CAST(INTERVAL AS INTEGER)`
- `tests/dialects/test_clickhouse.py`:1522 `test_agg_functions_multiple_suffixes` via `validate_identity`: `SELECT sumMergeIfMerge(s) FROM (SELECT sumMergeIfState(agg, 1 = 1) AS s FROM (SELECT sumState(toFloat64(number)) AS agg FROM numbers(10)))`
  - expected: `SELECT SUMMERGEIFMERGE(s) FROM (SELECT SUMMERGEIFSTATE(agg, 1 = 1) AS s FROM (SELECT SUMSTATE(TOFLOAT64(number)) AS agg FROM NUMBERS(10)))`
  - actual: `SELECT SUMMERGEIFMERGE(s) FROM (SELECT SUMMERGEIFSTATE(agg, 1 = 1) AS s FROM (SELECT SUMSTATE(TOFLOAT64(number)) AS agg FROM numbers(10)))`
- `tests/dialects/test_clickhouse.py`:73 `test_clickhouse` via `validate_identity`: `CAST(x AS Enum('hello' = 1, 'world' = 2))`
  - expected: `CAST(x AS ENUM('hello' = 1, 'world' = 2))`
  - actual: `CAST(x AS Enum('hello' = 1, 'world' = 2))`

### `mismatch` `cast/type rendering: CAST()`

- `tests/dialects/test_bigquery.py`:1161 `test_bigquery` via `validate_all`: `cast(x as time format 'YYYY.MM.DD HH:MI:SSTZH')`
  - expected: `STR_TO_TIME(x, 'YYYY.MM.DD HH:MI:SSTZH')`
  - actual: `CAST(x AS TIME)`
- `tests/dialects/test_clickhouse.py`:503 `test_clickhouse` via `validate_all`: `CAST(1 AS NULLABLE(Int64))`
  - expected: `CAST(1 AS INTEGER)`
  - actual: `CAST(1 AS NULLABLE(Int64))`
- `tests/dialects/test_dialect.py`:491 `test_cast` via `validate_all`: `CAST(a AS NUMBER)`
  - expected: `CAST(a AS REAL)`
  - actual: `CAST(a AS NUMBER)`

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

### `mismatch` `cast/type rendering: WITH`

- `tests/dialects/test_bigquery.py`:2060 `test_bigquery` via `validate_all`: `WITH sample AS (SELECT * FROM UNNEST([TIMESTAMP '2024-03-15 14:35:46', TIMESTAMP '2024-03-16 01:12:03']) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY, 'America/New_York') AS truncated_ts FROM sample`
  - expected: `WITH sample AS (SELECT * FROM UNNEST(ARRAY(CAST('2024-03-15 14:35:46' AS TIMESTAMPTZ), CAST('2024-03-16 01:12:03' AS TIMESTAMPTZ))) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY, 'America/New_York') AS truncated_ts FROM sample`
  - actual: `WITH sample AS (SELECT * FROM UNNEST([TIMESTAMP '2024-03-15 14:35:46', TIMESTAMP '2024-03-16 01:12:03']) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY, 'America/New_York') AS truncated_ts FROM sample`
- `tests/dialects/test_bigquery.py`:2060 `test_bigquery` via `validate_all`: `WITH sample AS (SELECT * FROM UNNEST([TIMESTAMP '2024-03-15 14:35:46', TIMESTAMP '2024-03-16 01:12:03']) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY, 'America/New_York') AS truncated_ts FROM sample`
  - expected: `WITH sample AS (SELECT * FROM UNNEST(ARRAY(CAST('2024-03-15 14:35:46' AS TIMESTAMPTZ), CAST('2024-03-16 01:12:03' AS TIMESTAMPTZ))) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY, 'America/New_York') AS truncated_ts FROM sample`
  - actual: `WITH sample AS (SELECT * FROM UNNEST([TIMESTAMP '2024-03-15 14:35:46', TIMESTAMP '2024-03-16 01:12:03']) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY, 'America/New_York') AS truncated_ts FROM sample`
- `tests/dialects/test_bigquery.py`:2067 `test_bigquery` via `validate_all`: `WITH sample AS (SELECT ts FROM UNNEST([TIMESTAMP '2024-03-15 14:35:46', TIMESTAMP '2024-03-16 01:12:03']) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY) AS truncated_ts FROM sample`
  - expected: `WITH sample AS (SELECT ts FROM UNNEST(ARRAY(CAST('2024-03-15 14:35:46' AS TIMESTAMPTZ), CAST('2024-03-16 01:12:03' AS TIMESTAMPTZ))) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY) AS truncated_ts FROM sample`
  - actual: `WITH sample AS (SELECT ts FROM UNNEST([TIMESTAMP '2024-03-15 14:35:46', TIMESTAMP '2024-03-16 01:12:03']) AS ts) SELECT ts, TIMESTAMP_TRUNC(ts, DAY) AS truncated_ts FROM sample`

### `mismatch` `date/time rendering: CREATE`

- `tests/dialects/test_postgres.py`:1277 `test_ddl` via `validate_identity`: `CREATE CONSTRAINT TRIGGER my_trigger AFTER INSERT OR DELETE OR UPDATE OF col_a, col_b ON public.my_table DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION DO_STH()`
  - expected: `CREATE CONSTRAINT TRIGGER my_trigger`
  - actual: `CREATE CONSTRAINT TRIGGER my_trigger AFTER INSERT OR DELETE OR UPDATE OF col_a, col_b ON public.my_table DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION DO_STH()`
- `tests/dialects/test_postgres.py`:1921 `test_postgres_create_trigger` via `validate_identity`: `CREATE TRIGGER check_update BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE FUNCTION CHECK_ACCOUNT_UPDATE()`
  - expected: `CREATE TRIGGER check_update`
  - actual: `CREATE TRIGGER check_update BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE FUNCTION CHECK_ACCOUNT_UPDATE()`
- `tests/dialects/test_postgres.py`:1921 `test_postgres_create_trigger` via `validate_identity`: `CREATE TRIGGER audit_changes AFTER INSERT OR UPDATE OR DELETE ON products FOR EACH ROW EXECUTE FUNCTION AUDIT_LOG()`
  - expected: `CREATE TRIGGER audit_changes`
  - actual: `CREATE TRIGGER audit_changes AFTER INSERT OR UPDATE OR DELETE ON products FOR EACH ROW EXECUTE FUNCTION AUDIT_LOG()`

### `mismatch` `date/time rendering: SELECT UNNEST()`

- `tests/dialects/test_dialect.py`:3651 `test_generate_date_array` via `validate_all`: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE '2020-01-01', DATE '2020-02-01', INTERVAL 1 WEEK))`
  - expected: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE('2020-01-01'), DATE('2020-02-01'), INTERVAL '1' WEEK))`
  - actual: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE '2020-01-01', DATE '2020-02-01', INTERVAL 1 WEEK))`
- `tests/dialects/test_dialect.py`:3651 `test_generate_date_array` via `validate_all`: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE '2020-01-01', DATE '2020-02-01', INTERVAL 1 WEEK))`
  - expected: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE('2020-01-01'), DATE('2020-02-01'), INTERVAL '1' WEEK))`
  - actual: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE '2020-01-01', DATE '2020-02-01', INTERVAL 1 WEEK))`
- `tests/dialects/test_dialect.py`:3651 `test_generate_date_array` via `validate_all`: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE '2020-01-01', DATE '2020-02-01', INTERVAL 1 WEEK))`
  - expected: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE('2020-01-01'), DATE('2020-02-01'), INTERVAL '1' WEEK))`
  - actual: `SELECT * FROM UNNEST(GENERATE_DATE_ARRAY(DATE '2020-01-01', DATE '2020-02-01', INTERVAL 1 WEEK))`

### `mismatch` `json rendering: SELECT JSON_VALUE()`

- `tests/dialects/test_exasol.py`:890 `test_json` via `validate_identity`: `SELECT JSON_VALUE('{"d":"a"}', '$.d' NULL ON ERROR) AS x`
  - expected: `SELECT JSON_VALUE('{"d":"a"}', '$.d' NULL ON ERROR) AS x`
  - actual: `SELECT JSON_VALUE('{"d":"a"}', '$.d') AS x`
- `tests/dialects/test_exasol.py`:891 `test_json` via `validate_all`: `SELECT JSON_VALUE('{"d":"a"}', '$.d' NULL ON ERROR) AS x`
  - expected: `SELECT JSON_VALUE('{"d":"a"}', '$.d' NULL ON ERROR) AS x`
  - actual: `SELECT JSON_VALUE('{"d":"a"}', '$.d') AS x`
- `tests/dialects/test_exasol.py`:891 `test_json` via `validate_all`: `SELECT JSON_VALUE('{"d":"a"}', '$.d' NULL ON ERROR) AS x`
  - expected: `SELECT JSON_VALUE('{"d":"a"}', '$.d' NULL ON ERROR) AS x`
  - actual: `SELECT JSON_VALUE('{"d":"a"}', '$.d') AS x`

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
- `tests/dialects/test_clickhouse.py`:681 `test_clickhouse` via `validate_identity`: `SELECT 1_2_3_4_5`
  - expected: `SELECT "1_2_3_4_5"`
  - actual: `SELECT 1 AS _2_3_4_5`
- `tests/dialects/test_clickhouse.py`:682 `test_clickhouse` via `validate_identity`: `SELECT 1_b`
  - expected: `SELECT "1_b"`
  - actual: `SELECT 1 AS _b`

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

