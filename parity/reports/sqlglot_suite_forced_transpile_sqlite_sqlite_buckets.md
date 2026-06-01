# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_sqlite_sqlite.jsonl`

Total rows: `15156`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 11400 |
| `mismatch` | 1487 |
| `oracle-error` | 1545 |
| `rust-error` | 587 |
| `unsupported-harness-shape` | 137 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `sqlite` | `sqlite` | 11400 |
| `oracle-error` | `sqlite` | `sqlite` | 1545 |
| `mismatch` | `sqlite` | `sqlite` | 1487 |
| `rust-error` | `sqlite` | `sqlite` | 587 |
| `unsupported-harness-shape` | `sqlite` | `sqlite` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 8292 |
| `match` | `validate_identity` | 3004 |
| `oracle-error` | `validate_identity` | 993 |
| `mismatch` | `validate_identity` | 711 |
| `mismatch` | `validate_all` | 709 |
| `oracle-error` | `validate_all` | 543 |
| `rust-error` | `validate_identity` | 351 |
| `rust-error` | `validate_all` | 233 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 104 |
| `mismatch` | `validate` | 67 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |
| `rust-error` | `validate` | 3 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 707 |
| `match` | `CAST()` | 562 |
| `match` | `SELECT operator multiply` | 388 |
| `match` | `CREATE TABLE` | 363 |
| `match` | `SHOW` | 215 |
| `match` | `CREATE` | 196 |
| `oracle-error` | `SELECT` | 195 |
| `mismatch` | `CREATE TABLE` | 169 |
| `mismatch` | `SELECT` | 165 |
| `match` | `TRUNC()` | 164 |
| `mismatch` | `CREATE` | 147 |
| `oracle-error` | `SELECT operator multiply` | 143 |
| `match` | `WITH` | 127 |
| `match` | `ALTER TABLE` | 119 |
| `oracle-error` | `CREATE TABLE` | 113 |
| `match` | `SELECT UNNEST()` | 112 |
| `match` | `SELECT CAST()` | 110 |
| `match` | `X` | 107 |
| `match` | `SELECT DATEDIFF()` | 83 |
| `match` | `SET` | 78 |
| `match` | `DATE_TRUNC()` | 77 |
| `match` | `DATE_ADD()` | 74 |
| `match` | `LOG()` | 67 |
| `match` | `JSON_EXTRACT()` | 64 |
| `match` | `GRANT` | 62 |
| `rust-error` | `SELECT operator multiply` | 61 |
| `match` | `ANALYZE` | 60 |
| `match` | `SELECT SUM()` | 58 |
| `match` | `A` | 57 |
| `match` | `REGEXP_REPLACE()` | 57 |
| `match` | `SELECT DATE_SUB()` | 57 |
| `match` | `TIME_STR_TO_TIME()` | 57 |
| `rust-error` | `SELECT` | 57 |
| `match` | `REGEXP_INSTR()` | 56 |
| `match` | `REVOKE` | 56 |
| `match` | `SELECT TO_TIMESTAMP()` | 55 |
| `oracle-error` | `WITH` | 52 |
| `match` | `FROM` | 51 |
| `match` | `INSERT` | 48 |
| `match` | `SELECT DATE_TRUNC()` | 48 |

## Rust/Oracle/Unsupported Error Buckets

| Status | Error Bucket | Count |
| --- | --- | ---: |
| `oracle-error` | `oracle parse: Invalid expression / Unexpected token` | 753 |
| `oracle-error` | `oracle parse: Expecting )` | 472 |
| `oracle-error` | `oracle parse: Required keyword missing` | 130 |
| `unsupported-harness-shape` | `SQLGlot expects UnsupportedError` | 119 |
| `rust-error` | `parser: Expected identifier` | 46 |
| `oracle-error` | `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)` | 24 |
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
| `rust-error` | `parser: Expected As, got Eof ('')` | 6 |
| `rust-error` | `parser: Expected LParen, got Unnest ('UNNEST')` | 6 |
| `rust-error` | `parser: Expected RParen, got Hour ('HOUR')` | 6 |
| `rust-error` | `parser: Expected RParen, got Identifier ('PLAN')` | 6 |
| `rust-error` | `parser: Expected RParen, got With ('WITH')` | 6 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Group, value: "group", line: 1, col: 51, position: 50, quote_char: '\0' }` | 5 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Using, value: "USING", line: 1, col: 19, position: 18, quote_char: '\0' }` | 5 |
| `rust-error` | `parser: Expected RParen, got Day ('DAY')` | 5 |
| `rust-error` | `parser: Expected RParen, got Group ('GROUP')` | 5 |
| `rust-error` | `parser: Expected RParen, got Identifier ('device_data')` | 5 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.NUMBER, text: 25, line: 1, col: 24, start: 22, end: 23, comments: []>` | 4 |
| `oracle-error` | `oracle parse: Expected }` | 4 |
| `oracle-error` | `oracle token: Error tokenizing ''\''` | 4 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Asc, value: "ASC", line: 1, col: 36, position: 35, quote_char: '\0' }` | 4 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Desc, value: "DESC", line: 1, col: 1, position: 0, quote_char: '\0' }` | 4 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Desc, value: "DESC", line: 1, col: 36, position: 35, quote_char: '\0' }` | 4 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 14, position: 13, quote_char: '\0' }` | 4 |

## Mismatch Signature Buckets

| Status | Signature | Count |
| --- | --- | ---: |
| `mismatch` | `missing AS or alias rendering` | 313 |
| `mismatch` | `DDL/create-table rendering` | 149 |
| `mismatch` | `SELECT` | 97 |
| `mismatch` | `CREATE` | 96 |
| `mismatch` | `case-only rendering difference` | 44 |
| `mismatch` | `ALTER TABLE` | 41 |
| `mismatch` | `missing quoted identifier` | 36 |
| `mismatch` | `quote-style difference` | 34 |
| `mismatch` | `SELECT operator multiply` | 28 |
| `mismatch` | `date/time rendering: CREATE` | 24 |
| `mismatch` | `cast/type rendering: CAST()` | 17 |
| `mismatch` | `date/time rendering: SELECT UNNEST()` | 17 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 16 |
| `mismatch` | `A` | 15 |
| `mismatch` | `X` | 13 |
| `mismatch` | `WITH` | 12 |
| `mismatch` | `'FOO'` | 10 |
| `mismatch` | `--` | 9 |
| `mismatch` | `COPY` | 9 |
| `mismatch` | `PIVOT` | 9 |
| `mismatch` | `SELECT POSEXPLODE()` | 9 |
| `mismatch` | `whitespace-only difference` | 8 |
| `mismatch` | `DESCRIBE` | 7 |
| `mismatch` | `INSERT` | 7 |
| `mismatch` | `SELECT COUNT()` | 7 |
| `mismatch` | `SELECT FLOOR()` | 7 |
| `mismatch` | `SELECT TO_ARRAY()` | 7 |
| `mismatch` | `FROM` | 6 |
| `mismatch` | `POSITION()` | 6 |
| `mismatch` | `SELECT CEIL()` | 6 |
| `mismatch` | `STRPOS()` | 6 |
| `mismatch` | `STR_POSITION()` | 6 |
| `mismatch` | `date/time rendering: SELECT DATETRUNC()` | 6 |
| `mismatch` | `date/time rendering: WITH` | 6 |
| `mismatch` | `INTERVAL` | 5 |
| `mismatch` | `SELECT LEADING()` | 5 |
| `mismatch` | `SELECT operator json` | 5 |
| `mismatch` | `SELECT operator json-text` | 5 |
| `mismatch` | `date/time rendering: DATE_TRUNC()` | 5 |
| `mismatch` | `date/time rendering: PARSE_TIMESTAMP()` | 5 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 1075 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 678 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 458 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 344 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 289 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 263 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 251 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 241 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 182 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 173 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 152 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 148 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 125 |
| `match` | `tests/dialects/test_oracle.py` | `test_oracle` | 109 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 109 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 105 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 103 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 99 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 89 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 85 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 84 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 83 |
| `match` | `tests/dialects/test_dialect.py` | `test_trim` | 80 |
| `match` | `tests/dialects/test_databricks.py` | `test_databricks` | 78 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `match` | `tests/dialects/test_presto.py` | `test_time` | 74 |
| `match` | `tests/dialects/test_exasol.py` | `test_scalar` | 73 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 70 |
| `match` | `tests/dialects/test_duckdb.py` | `test_time` | 68 |
| `match` | `tests/dialects/test_dialect.py` | `test_set_operators` | 66 |
| `match` | `tests/dialects/test_hive.py` | `test_joins_without_on` | 66 |
| `match` | `tests/dialects/test_mysql.py` | `test_identity` | 66 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 66 |
| `match` | `tests/dialects/test_dialect.py` | `test_string_functions` | 64 |
| `match` | `tests/dialects/test_snowflake.py` | `test_regexp_functions` | 62 |
| `match` | `tests/dialects/test_hive.py` | `test_time` | 61 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 60 |

## Bucket Examples

### `mismatch` `'FOO'`

- `tests/dialects/test_snowflake.py`:2065 `test_snowflake` via `validate_all`: `'foo' REGEXP 'bar'`
  - expected: `REGEXP_LIKE('foo', 'bar')`
  - actual: `'foo'`
- `tests/dialects/test_snowflake.py`:2065 `test_snowflake` via `validate_all`: `'foo' REGEXP 'bar'`
  - expected: `REGEXP_LIKE('foo', 'bar')`
  - actual: `'foo'`
- `tests/dialects/test_snowflake.py`:2065 `test_snowflake` via `validate_all`: `'foo' REGEXP 'bar'`
  - expected: `REGEXP_LIKE('foo', 'bar')`
  - actual: `'foo'`

### `mismatch` `--`

- `tests/test_transpile.py`:645 `test_comment_single_line_with_block_close` via `validate`: `-- aa */ SELECT * FROM secret_table -- SELECT 1`
  - expected: `/* aa * / SELECT * FROM secret_table -- */ SELECT 1`
  - actual: `SELECT 1`
- `tests/test_transpile.py`:649 `test_comment_single_line_with_block_close` via `validate`: `-- comment */ DROP TABLE users -- SELECT 1`
  - expected: `/* comment * / DROP TABLE users -- */ SELECT 1`
  - actual: `SELECT 1`
- `tests/test_transpile.py`:204 `test_comments` via `validate`: `-- comment 1 -- comment 2 -- comment 3 SELECT * FROM foo`
  - expected: `/* comment 1 */ /* comment 2 */ /* comment 3 */ SELECT * FROM foo`
  - actual: `SELECT * FROM foo`

### `mismatch` `A`

- `tests/test_transpile.py`:683 `test_not_range` via `validate`: `a NOT IN (1, 2)`
  - expected: `NOT a IN (1, 2)`
  - actual: `a NOT IN (1, 2)`
- `tests/test_transpile.py`:684 `test_not_range` via `validate`: `a IS NOT NULL`
  - expected: `NOT a IS NULL`
  - actual: `a IS NOT NULL`
- `tests/dialects/test_duckdb.py`:1346 `test_duckdb` via `validate_identity`: `a ~~~ b`
  - expected: `a GLOB b`
  - actual: `a LIKE ~b`

### `mismatch` `ALTER TABLE`

- `tests/test_transpile.py`:754 `test_alter` via `validate`: `ALTER TABLE integers ALTER i TYPE VARCHAR COLLATE foo USING bar`
  - expected: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo USING bar`
  - actual: `ALTER TABLE integers ALTER COLUMN i SET DATA TYPE TEXT COLLATE foo`
- `tests/dialects/test_hive.py`:213 `test_ddl` via `validate_identity`: `ALTER TABLE X ADD COLUMNS (y INT, z STRING)`
  - expected: `ALTER TABLE X ADD COLUMNS (y INTEGER, z TEXT)`
  - actual: `ALTER TABLE X ADD COLUMNS (y INT, z STRING)`
- `tests/dialects/test_mysql.py`:34 `test_ddl` via `validate_identity`: `ALTER TABLE t ADD COLUMN c INT INVISIBLE`
  - expected: `ALTER TABLE t ADD COLUMN c INT INVISIBLE`
  - actual: `ALTER TABLE t ADD COLUMN c INTEGER`

### `mismatch` `COPY`

- `tests/dialects/test_duckdb.py`:1287 `test_duckdb` via `validate_identity`: `COPY lineitem (l_orderkey) TO 'orderkey.tbl' WITH (DELIMITER '|')`
  - expected: `COPY INTO lineitem (l_orderkey) TO 'orderkey.tbl' WITH (DELIMITER '|')`
  - actual: `COPY lineitem (l_orderkey) TO 'orderkey.tbl' WITH (DELIMITER '|')`
- `tests/dialects/test_postgres.py`:897 `test_postgres` via `validate_identity`: `COPY tbl (col1, col2) FROM 'file' WITH (FORMAT format, HEADER MATCH, FREEZE TRUE)`
  - expected: `COPY INTO tbl (col1, col2) FROM 'file' WITH (FORMAT format, HEADER MATCH, FREEZE TRUE)`
  - actual: `COPY tbl (col1, col2) FROM 'file' WITH (FORMAT format, HEADER MATCH, FREEZE TRUE)`
- `tests/dialects/test_postgres.py`:900 `test_postgres` via `validate_identity`: `COPY tbl (col1, col2) TO 'file' WITH (FORMAT format, HEADER MATCH, FREEZE TRUE)`
  - expected: `COPY INTO tbl (col1, col2) TO 'file' WITH (FORMAT format, HEADER MATCH, FREEZE TRUE)`
  - actual: `COPY tbl (col1, col2) TO 'file' WITH (FORMAT format, HEADER MATCH, FREEZE TRUE)`

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
- `tests/dialects/test_athena.py`:50 `test_ddl` via `validate_identity`: `CREATE EXTERNAL TABLE foo (id INT, val STRING) CLUSTERED BY (id, val) INTO 10 BUCKETS`
  - expected: `CREATE TABLE foo (id INTEGER, val TEXT)`
  - actual: `CREATE EXTERNAL TABLE foo (id INT, val STRING) CLUSTERED BY (id, val) INTO 10 BUCKETS`
- `tests/dialects/test_bigquery.py`:199 `test_bigquery` via `validate_identity`: `CREATE TABLE x (a STRUCT<values ARRAY<INT64>>)`
  - expected: `CREATE TABLE x (a STRUCT<values ARRAY<INTEGER>>)`
  - actual: `CREATE TABLE x (a STRUCT<values ARRAY<INT64>>)`

### `mismatch` `PIVOT`

- `tests/dialects/test_duckdb.py`:27 `test_duckdb` via `validate_identity`: `PIVOT duckdb_functions() ON schema_name USING AVG(LENGTH(function_name))::INTEGER GROUP BY schema_name`
  - expected: ``
  - actual: `PIVOT duckdb_functions() ON schema_name USING AVG(LENGTH(function_name))::INTEGER GROUP BY schema_name`
- `tests/dialects/test_duckdb.py`:675 `test_duckdb` via `validate_identity`: `PIVOT Cities ON Year IN (2000, 2010) USING SUM(Population) GROUP BY Country`
  - expected: ``
  - actual: `PIVOT Cities ON Year IN (2000, 2010) USING SUM(Population) GROUP BY Country`
- `tests/dialects/test_duckdb.py`:678 `test_duckdb` via `validate_identity`: `PIVOT Cities ON Year USING SUM(Population) AS total, MAX(Population) AS max GROUP BY Country`
  - expected: ``
  - actual: `PIVOT Cities ON Year USING SUM(Population) AS total, MAX(Population) AS max GROUP BY Country`

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
  - actual: `SELECT * FROM a INNER JOIN b ON TRUE`

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

- `tests/dialects/test_dialect.py`:1837 `test_json` via `validate_all`: `x -> '$.y'`
  - expected: `x -> '$.y'`
  - actual: `x -> '$."$.y"'`
- `tests/dialects/test_dialect.py`:1837 `test_json` via `validate_all`: `x -> '$.y'`
  - expected: `x -> '$.y'`
  - actual: `x -> '$."$.y"'`
- `tests/dialects/test_dialect.py`:1866 `test_json` via `validate_all`: `x ->> '$.y'`
  - expected: `x ->> '$.y'`
  - actual: `x ->> '$."$.y"'`

### `mismatch` `case-only rendering difference`

- `tests/test_transpile.py`:672 `test_types` via `validate`: `interval::int`
  - expected: `CAST(interval AS INTEGER)`
  - actual: `CAST(INTERVAL AS INTEGER)`
- `tests/dialects/test_bigquery.py`:759 `test_bigquery` via `validate_all`: `TIMESTAMPDIFF(month, b, a)`
  - expected: `TIMESTAMPDIFF(month, b, A)`
  - actual: `TIMESTAMPDIFF(month, b, a)`
- `tests/dialects/test_bigquery.py`:759 `test_bigquery` via `validate_all`: `TIMESTAMPDIFF(month, b, a)`
  - expected: `TIMESTAMPDIFF(month, b, A)`
  - actual: `TIMESTAMPDIFF(month, b, a)`

### `mismatch` `cast/type rendering: CAST()`

- `tests/dialects/test_bigquery.py`:1161 `test_bigquery` via `validate_all`: `cast(x as time format 'YYYY.MM.DD HH:MI:SSTZH')`
  - expected: `STR_TO_TIME(x, 'YYYY.MM.DD HH:MI:SSTZH')`
  - actual: `CAST(x AS TIME)`
- `tests/dialects/test_clickhouse.py`:72 `test_clickhouse` via `validate_identity`: `CAST(x AS Nested(ID UInt32, Serial UInt32, EventTime DateTime))`
  - expected: `CAST(x AS Nested(ID, SERIAL, EVENTTIME))`
  - actual: `CAST(x AS Nested(ID UInt32, Serial UInt32, EventTime DateTime))`
- `tests/dialects/test_clickhouse.py`:91 `test_clickhouse` via `validate_identity`: `CAST((1, 2) AS Tuple(a Int8, b Int16))`
  - expected: `CAST((1, 2) AS Tuple(A, B))`
  - actual: `CAST((1, 2) AS Tuple(a Int8, b Int16))`

### `mismatch` `cast/type rendering: SELECT CAST()`

- `tests/dialects/test_bigquery.py`:1120 `test_bigquery` via `validate_all`: `SELECT CAST(TIMESTAMP '2008-12-25 00:00:00+00:00' AS STRING FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM' AT TIME ZONE 'Asia/Kolkata') AS date_time_to_string`
  - expected: `SELECT CAST(CAST('2008-12-25 00:00:00+00:00' AS TIMESTAMP) AS TEXT FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM' AT TIME ZONE 'Asia/Kolkata') AS date_time_to_string`
  - actual: `SELECT CAST(CAST('2008-12-25 00:00:00+00:00' AS TIMESTAMP) AS TEXT FORMAT 'YYYY-MM-DD HH24:MI:SS TZH:TZM') AS date_time_to_string`
- `tests/dialects/test_bigquery.py`:3041 `test_cast_format_with_parentheses` via `validate_identity`: `SELECT CAST('2026-03-24' AS STRING FORMAT ('YYYY'))`
  - expected: `SELECT CAST('2026-03-24' AS TEXT FORMAT 'YYYY')`
  - actual: `SELECT CAST('2026-03-24' AS TEXT)`
- `tests/dialects/test_bigquery.py`:3046 `test_cast_format_with_parentheses` via `validate_identity`: `SELECT CAST(date AS STRING FORMAT ('YYYY')) FROM (SELECT DATE('2026-03-24') AS date)`
  - expected: `SELECT CAST(date AS TEXT FORMAT 'YYYY') FROM (SELECT DATE('2026-03-24') AS date)`
  - actual: `SELECT CAST(date AS TEXT) FROM (SELECT DATE('2026-03-24') AS date)`

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

- `tests/dialects/test_athena.py`:62 `test_ddl` via `validate_identity`: `` CREATE EXTERNAL TABLE `my_table` (`a7` ARRAY<DATE>) ROW FORMAT SERDE 'a' STORED AS INPUTFORMAT 'b' OUTPUTFORMAT 'c' LOCATION 'd' TBLPROPERTIES ('e'='f') ``
  - expected: `CREATE TABLE "my_table" ("a7" ARRAY<DATE>)`
  - actual: `` CREATE EXTERNAL TABLE `my_table` (`a7` ARRAY<DATE>) ROW FORMAT SERDE 'a' STORED AS INPUTFORMAT 'b' OUTPUTFORMAT 'c' LOCATION 'd' TBLPROPERTIES ('e'='f') ``
- `tests/dialects/test_bigquery.py`:248 `test_bigquery` via `validate_identity`: `MERGE INTO dataset.NewArrivals USING (SELECT * FROM UNNEST([('microwave', 10, 'warehouse #1'), ('dryer', 30, 'warehouse #1'), ('oven', 20, 'warehouse #2')])) ON FALSE WHEN NOT MATCHED THEN INSERT ROW WHEN NOT MATCHED BY SOURCE THEN DELETE`
  - expected: `MERGE INTO dataset.NewArrivals USING (SELECT * FROM UNNEST("('microwave', 10, 'warehouse #1'), ('dryer', 30, 'warehouse #1'), ('oven', 20, 'warehouse #2')")) ON FALSE WHEN NOT MATCHED THEN INSERT ROW WHEN NOT MATCHED BY SOURCE THEN DELETE`
  - actual: `MERGE INTO dataset.NewArrivals USING (SELECT * FROM UNNEST([('microwave', 10, 'warehouse #1'), ('dryer', 30, 'warehouse #1'), ('oven', 20, 'warehouse #2')])) ON FALSE WHEN NOT MATCHED THEN INSERT ROW WHEN NOT MATCHED BY SOURCE THEN DELETE`
- `tests/dialects/test_bigquery.py`:1333 `test_bigquery` via `validate_all`: `SELECT ARRAY(SELECT x FROM UNNEST([0, 1]) AS x)`
  - expected: `SELECT ARRAY(SELECT x FROM UNNEST("0, 1") AS x)`
  - actual: `SELECT ARRAY(SELECT x FROM UNNEST([0, 1]) AS x)`

### `mismatch` `quote-style difference`

- `tests/dialects/test_athena.py`:92 `test_ddl` via `validate_identity`: `` ALTER TABLE `foo` DROP COLUMN `id` ``
  - expected: `ALTER TABLE "foo" DROP COLUMN "id"`
  - actual: `ALTER TABLE "foo" DROP COLUMN id`
- `tests/dialects/test_athena.py`:106 `test_ddl_quoting` via `validate_identity`: `` CREATE SCHEMA `foo` ``
  - expected: `CREATE SCHEMA "foo"`
  - actual: `` CREATE SCHEMA `foo` ``
- `tests/dialects/test_athena.py`:161 `test_dml_quoting` via `validate_identity`: `INSERT INTO "foo" ("id") VALUES (1)`
  - expected: `INSERT INTO "foo" ("id") VALUES (1)`
  - actual: `INSERT INTO "foo" (id) VALUES (1)`

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

### `oracle-error` `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>`

- `tests/dialects/test_clickhouse.py`:842 `test_parameterization` via `validate_all`: `SELECT * FROM {table: Identifier}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {table: Identifier}`
- `tests/dialects/test_spark.py`:981 `test_spark` via `validate_all`: `SELECT * FROM {df}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {df}`
- `tests/dialects/test_spark.py`:981 `test_spark` via `validate_all`: `SELECT * FROM {df}`
  - error: `ParseError: Expected table name but got <Token token_type: TokenType.L_BRACE, text: {, line: 1, col: 15, start: 14, end: 14, comments: []>. Line 1, Col: 15. SELECT * FROM {df}`

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
- `tests/dialects/test_dialect.py`:3988 `test_escaped_identifier_delimiter` via `validate_all`: `SELECT 1 AS [[x]]]`
  - expected: `SELECT 1 AS "[x]"`
  - error: `ValueError: Parser error: Expected identifier, got LBracket ('[') at line 1 col 13`
- `tests/dialects/test_dialect.py`:5279 `test_operator` via `validate_identity`: `SELECT 1 OPERATOR(pg_catalog.+) 2`
  - expected: `SELECT 1 OPERATOR(pg_catalog.+) 2`
  - error: `ValueError: Parser error: Expected identifier, got Plus ('+') at line 1 col 30`

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

