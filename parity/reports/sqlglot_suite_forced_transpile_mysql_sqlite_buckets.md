# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_mysql_sqlite.jsonl`

Total rows: `15164`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 7725 |
| `mismatch` | 3892 |
| `oracle-error` | 1743 |
| `rust-error` | 1667 |
| `unsupported-harness-shape` | 137 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `mysql` | `sqlite` | 7725 |
| `mismatch` | `mysql` | `sqlite` | 3892 |
| `oracle-error` | `mysql` | `sqlite` | 1743 |
| `rust-error` | `mysql` | `sqlite` | 1667 |
| `unsupported-harness-shape` | `mysql` | `sqlite` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5636 |
| `mismatch` | `validate_all` | 2676 |
| `match` | `validate_identity` | 2005 |
| `mismatch` | `validate_identity` | 1142 |
| `oracle-error` | `validate_identity` | 1135 |
| `rust-error` | `validate_all` | 874 |
| `rust-error` | `validate_identity` | 777 |
| `oracle-error` | `validate_all` | 599 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 84 |
| `mismatch` | `validate` | 74 |
| `rust-error` | `validate` | 16 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 578 |
| `match` | `CAST()` | 379 |
| `mismatch` | `CREATE TABLE` | 278 |
| `match` | `SELECT operator multiply` | 232 |
| `mismatch` | `SELECT` | 206 |
| `mismatch` | `CREATE` | 181 |
| `oracle-error` | `SELECT` | 181 |
| `rust-error` | `SELECT` | 167 |
| `match` | `TRUNC()` | 162 |
| `match` | `CREATE` | 161 |
| `rust-error` | `SELECT operator multiply` | 149 |
| `oracle-error` | `CREATE TABLE` | 148 |
| `oracle-error` | `SELECT operator multiply` | 135 |
| `match` | `CREATE TABLE` | 132 |
| `mismatch` | `SELECT operator multiply` | 117 |
| `mismatch` | `SELECT UNNEST()` | 108 |
| `mismatch` | `SHOW` | 108 |
| `match` | `SHOW` | 105 |
| `match` | `X` | 104 |
| `rust-error` | `FROM` | 104 |
| `oracle-error` | `CAST()` | 96 |
| `rust-error` | `CAST()` | 95 |
| `rust-error` | `CREATE TABLE` | 88 |
| `mismatch` | `ALTER TABLE` | 84 |
| `match` | `ALTER TABLE` | 81 |
| `match` | `SELECT DATEDIFF()` | 79 |
| `match` | `SET` | 78 |
| `match` | `WITH` | 77 |
| `match` | `DATE_TRUNC()` | 71 |
| `match` | `LOG()` | 67 |
| `match` | `SELECT UNNEST()` | 67 |
| `match` | `GRANT` | 62 |
| `mismatch` | `WITH` | 61 |
| `match` | `ANALYZE` | 60 |
| `oracle-error` | `SELECT OPTION()` | 60 |
| `mismatch` | `CAST()` | 58 |
| `match` | `A` | 57 |
| `match` | `REGEXP_INSTR()` | 56 |
| `match` | `REVOKE` | 56 |
| `rust-error` | `SELECT CAST()` | 55 |

## Rust/Oracle/Unsupported Error Buckets

| Status | Error Bucket | Count |
| --- | --- | ---: |
| `oracle-error` | `oracle parse: Invalid expression / Unexpected token` | 660 |
| `oracle-error` | `oracle parse: Expecting )` | 413 |
| `oracle-error` | `oracle parse: Required keyword missing` | 195 |
| `rust-error` | `parser: Expected identifier` | 126 |
| `unsupported-harness-shape` | `SQLGlot expects UnsupportedError` | 119 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: From, value: "FROM", line: 1, col: 1, position: 0, quote_char: '\0' }` | 113 |
| `oracle-error` | `oracle parse: Expected TYPE after CAST` | 108 |
| `rust-error` | `parser: Expected RParen, got Comma (',')` | 102 |
| `oracle-error` | `oracle parse: INTERVAL expression expected but got '1'` | 72 |
| `rust-error` | `parser: Expected RParen, got FatArrow ('=>')` | 72 |
| `rust-error` | `parser: Expected statement` | 66 |
| `oracle-error` | `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)` | 46 |
| `oracle-error` | `oracle parse: Expected type` | 39 |
| `rust-error` | `parser: Expected RParen, got LParen ('(')` | 38 |
| `rust-error` | `parser: Expected RParen, got Identifier ('TO')` | 34 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.SENTINEL, text: SENTINEL, line: 1, col: 1, start: 0, end: 0, comments: []>` | 33 |
| `rust-error` | `parser: Expected RParen, got Identifier ('VARYING')` | 32 |
| `rust-error` | `parser: Expected RParen, got As ('AS')` | 30 |
| `rust-error` | `parser: Expected data type, got Map` | 24 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Range, value: "RANGE", line: 1, col: 8, position: 7, quote_char: '\0' }` | 22 |
| `rust-error` | `parser: Expected And, got Number ('10')` | 22 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }` | 21 |
| `rust-error` | `parser: Expected VALUES, SELECT, or DEFAULT VALUES after INSERT` | 20 |
| `oracle-error` | `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)` | 19 |
| `rust-error` | `parser: Expected RParen, got Respect ('RESPECT')` | 19 |
| `rust-error` | `parser: Expected RParen, got Identifier ('FORMAT')` | 18 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Lateral, value: "LATERAL", line: 1, col: 17, position: 16, quote_char: '\0' }` | 17 |
| `oracle-error` | `oracle parse: Expecting (` | 16 |
| `oracle-error` | `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)` | 16 |
| `rust-error` | `parser: Expected Join, got Union ('UNION')` | 14 |
| `unsupported-harness-shape` | `identify helper option is not supported yet` | 14 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: By, value: "BY", line: 1, col: 57, position: 56, quote_char: '\0' }` | 13 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: By, value: "BY", line: 1, col: 69, position: 68, quote_char: '\0' }` | 13 |
| `rust-error` | `parser: Expected RParen, got Identifier ('2008-12-25 15:30:00+00')` | 13 |
| `rust-error` | `parser: Expected RParen, got Order ('ORDER')` | 12 |
| `rust-error` | `parser: Expected data type, got Struct` | 12 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }` | 10 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: As, value: "AS", line: 1, col: 33, position: 32, quote_char: '\0' }` | 9 |
| `rust-error` | `parser: Expected RParen, got Having ('HAVING')` | 9 |
| `rust-error` | `parser: Expected SELECT or INSERT after WITH clause` | 9 |

## Mismatch Signature Buckets

| Status | Signature | Count |
| --- | --- | ---: |
| `mismatch` | `missing AS or alias rendering` | 316 |
| `mismatch` | `DDL/create-table rendering` | 274 |
| `mismatch` | `case-only rendering difference` | 159 |
| `mismatch` | `SELECT` | 155 |
| `mismatch` | `SELECT operator multiply` | 114 |
| `mismatch` | `SHOW` | 107 |
| `mismatch` | `CREATE` | 105 |
| `mismatch` | `SELECT UNNEST()` | 78 |
| `mismatch` | `ALTER TABLE` | 75 |
| `mismatch` | `cast/type rendering: CAST()` | 58 |
| `mismatch` | `missing quoted identifier` | 57 |
| `mismatch` | `date/time rendering: SELECT DATE_SUB()` | 52 |
| `mismatch` | `cast/type rendering: SELECT TO_CHAR()` | 41 |
| `mismatch` | `date/time rendering: SELECT TO_TIMESTAMP()` | 39 |
| `mismatch` | `date/time rendering: TIME_STR_TO_TIME()` | 36 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 34 |
| `mismatch` | `date/time rendering: SELECT DATEADD()` | 32 |
| `mismatch` | `date/time rendering: STR_TO_TIME()` | 30 |
| `mismatch` | `REGEXP_EXTRACT()` | 29 |
| `mismatch` | `A` | 28 |
| `mismatch` | `LEVENSHTEIN()` | 28 |
| `mismatch` | `MEDIAN()` | 28 |
| `mismatch` | `WITH` | 28 |
| `mismatch` | `REGEXP_REPLACE()` | 27 |
| `mismatch` | `SELECT FORMAT()` | 27 |
| `mismatch` | `date/time rendering: DATE_ADD()` | 27 |
| `mismatch` | `json rendering: JSON_EXTRACT()` | 27 |
| `mismatch` | `quote-style difference` | 27 |
| `mismatch` | `date/time rendering: MONTH()` | 26 |
| `mismatch` | `date/time rendering: YEAR()` | 24 |
| `mismatch` | `date/time rendering: CREATE` | 23 |
| `mismatch` | `SELECT REGEXP_EXTRACT()` | 22 |
| `mismatch` | `cast/type rendering: SELECT EXTRACT()` | 22 |
| `mismatch` | `SHA256()` | 21 |
| `mismatch` | `date/time rendering: DAY()` | 21 |
| `mismatch` | `SELECT COUNT_IF()` | 20 |
| `mismatch` | `SELECT operator index` | 20 |
| `mismatch` | `X` | 20 |
| `mismatch` | `date/time rendering: EOMONTH()` | 20 |
| `mismatch` | `STRPOS()` | 19 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 761 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 350 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 334 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 286 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 246 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 212 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 196 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 191 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 182 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 164 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 161 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 159 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 140 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 137 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 127 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 124 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 122 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 105 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 96 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 96 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `mismatch` | `tests/dialects/test_presto.py` | `test_presto` | 86 |
| `oracle-error` | `tests/dialects/test_tsql.py` | `test_option` | 86 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 85 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 80 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 80 |
| `oracle-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 80 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 77 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 76 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 75 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_ddl` | 74 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 67 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 66 |
| `match` | `tests/dialects/test_dialect.py` | `test_set_operators` | 66 |
| `mismatch` | `tests/dialects/test_oracle.py` | `test_oracle` | 66 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 65 |

## Bucket Examples

### `mismatch` `A`

- `tests/test_transpile.py`:682 `test_not_range` via `validate`: `a NOT BETWEEN b AND c`
  - expected: `NOT a BETWEEN b AND c`
  - actual: `a NOT BETWEEN b AND c`
- `tests/test_transpile.py`:683 `test_not_range` via `validate`: `a NOT IN (1, 2)`
  - expected: `NOT a IN (1, 2)`
  - actual: `a NOT IN (1, 2)`
- `tests/test_transpile.py`:684 `test_not_range` via `validate`: `a IS NOT NULL`
  - expected: `NOT a IS NULL`
  - actual: `a IS NOT NULL`

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

### `mismatch` `case-only rendering difference`

- `tests/test_transpile.py`:704 `test_extract` via `validate`: `extract(week from current_date + 2)`
  - expected: `EXTRACT(WEEK FROM CURRENT_DATE + 2)`
  - actual: `EXTRACT(WEEK FROM current_date + 2)`
- `tests/dialects/test_bigquery.py`:1365 `test_bigquery` via `validate_all`: `current_time`
  - expected: `CURRENT_TIME`
  - actual: `current_time`
- `tests/dialects/test_bigquery.py`:1365 `test_bigquery` via `validate_all`: `current_time`
  - expected: `CURRENT_TIME`
  - actual: `current_time`

### `mismatch` `cast/type rendering: CAST()`

- `tests/dialects/test_bigquery.py`:223 `test_bigquery` via `validate_identity`: `CAST(x AS BIGNUMERIC)`
  - expected: `CAST(x AS BIGDECIMAL)`
  - actual: `CAST(x AS BIGNUMERIC)`
- `tests/dialects/test_bigquery.py`:1261 `test_bigquery` via `validate_all`: `CAST(a AS INT64)`
  - expected: `CAST(a AS INTEGER)`
  - actual: `CAST(a AS INT64)`
- `tests/dialects/test_bigquery.py`:1261 `test_bigquery` via `validate_all`: `CAST(a AS INT64)`
  - expected: `CAST(a AS INTEGER)`
  - actual: `CAST(a AS INT64)`

### `mismatch` `cast/type rendering: SELECT CAST()`

- `tests/dialects/test_bigquery.py`:3843 `test_bignumeric` via `validate_all`: `SELECT CAST(1 AS BIGNUMERIC)`
  - expected: `SELECT CAST(1 AS BIGDECIMAL)`
  - actual: `SELECT CAST(1 AS BIGNUMERIC)`
- `tests/dialects/test_bigquery.py`:3843 `test_bignumeric` via `validate_all`: `SELECT CAST(1 AS BIGNUMERIC)`
  - expected: `SELECT CAST(1 AS BIGDECIMAL)`
  - actual: `SELECT CAST(1 AS BIGNUMERIC)`
- `tests/dialects/test_clickhouse.py`:280 `test_clickhouse` via `validate_all`: `SELECT CAST(STR_TO_DATE(SUBSTRING(a.eta, 1, 10), '%Y-%m-%d') AS Nullable(DATE))`
  - expected: `SELECT DATE(STR_TO_DATE(SUBSTRING(a.eta, 1, 10), '%Y-%m-%d'))`
  - actual: `SELECT CAST(STR_TO_DATE(SUBSTR(a.eta, 1, 10), '%Y-%m-%d') AS NULLABLE)`

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
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL 2 DAY)`
- `tests/dialects/test_bigquery.py`:478 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(CURRENT_DATE(), INTERVAL 2 DAY)`
  - expected: `SELECT DATE_SUB(CURRENT_DATE, '2', DAY)`
  - actual: `SELECT DATE_SUB(CURRENT_DATE, INTERVAL 2 DAY)`
- `tests/dialects/test_bigquery.py`:485 `test_bigquery` via `validate_all`: `SELECT DATE_SUB(DATE '2008-12-25', INTERVAL 5 DAY)`
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

- `tests/test_transpile.py`:771 `test_time` via `validate`: `TIMESTAMP WITHOUT TIME ZONE '2020-01-01'`
  - expected: `CAST('2020-01-01' AS TIMESTAMPTZ)`
  - actual: `TIMESTAMP`
- `tests/test_transpile.py`:836 `test_time` via `validate`: `TIME_TO_TIME_STR(x)`
  - expected: `CAST(x AS TEXT)`
  - actual: `TIME_TO_TIME_STR(x)`
- `tests/dialects/test_bigquery.py`:3819 `test_bignumeric` via `validate_all`: `SELECT BIGNUMERIC '1'`
  - expected: `SELECT CAST('1' AS BIGDECIMAL)`
  - actual: `SELECT BIGNUMERIC`

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

### `oracle-error` `oracle parse: Expected TYPE after CAST`

- `tests/dialects/test_bigquery.py`:237 `test_bigquery` via `validate_identity`: `CAST(x AS RECORD)`
  - error: `ParseError: Expected TYPE after CAST. Line 1, Col: 16. CAST(x AS RECORD)`
- `tests/dialects/test_bigquery.py`:376 `test_bigquery` via `validate_identity`: `SELECT CAST(1 AS BYTEINT)`
  - error: `ParseError: Expected TYPE after CAST. Line 1, Col: 24. SELECT CAST(1 AS BYTEINT)`
- `tests/dialects/test_bigquery.py`:1271 `test_bigquery` via `validate_all`: `CAST(a AS BYTES)`
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

### `rust-error` `ValueError: Unexpected token: Token { token_type: From, value: "FROM", line: 1, col: 1, position: 0, quote_char: '\0' }`

- `tests/dialects/test_duckdb.py`:485 `test_duckdb` via `validate_identity`: `FROM x SELECT x UNION SELECT 1`
  - expected: `SELECT x FROM x UNION SELECT 1`
  - error: `ValueError: Unexpected token: Token { token_type: From, value: "FROM", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `tests/dialects/test_duckdb.py`:486 `test_duckdb` via `validate_identity`: `FROM (FROM tbl)`
  - expected: `SELECT * FROM (SELECT * FROM tbl)`
  - error: `ValueError: Unexpected token: Token { token_type: From, value: "FROM", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `tests/dialects/test_duckdb.py`:487 `test_duckdb` via `validate_identity`: `FROM tbl`
  - expected: `SELECT * FROM tbl`
  - error: `ValueError: Unexpected token: Token { token_type: From, value: "FROM", line: 1, col: 1, position: 0, quote_char: '\0' }`

### `rust-error` `ValueError: Unexpected token: Token { token_type: Range, value: "RANGE", line: 1, col: 8, position: 7, quote_char: '\0' }`

- `tests/dialects/test_bigquery.py`:2709 `test_range_type` via `validate_identity`: `SELECT RANGE<DATE> '[2020-01-01, 2020-12-31)'`
  - expected: `SELECT CAST('[2020-01-01, 2020-12-31)' AS RANGE<DATE>)`
  - error: `ValueError: Unexpected token: Token { token_type: Range, value: "RANGE", line: 1, col: 8, position: 7, quote_char: '\0' }`
- `tests/dialects/test_bigquery.py`:2709 `test_range_type` via `validate_identity`: `SELECT RANGE<DATE> '[UNBOUNDED, 2020-12-31)'`
  - expected: `SELECT CAST('[UNBOUNDED, 2020-12-31)' AS RANGE<DATE>)`
  - error: `ValueError: Unexpected token: Token { token_type: Range, value: "RANGE", line: 1, col: 8, position: 7, quote_char: '\0' }`
- `tests/dialects/test_bigquery.py`:2709 `test_range_type` via `validate_identity`: `SELECT RANGE<DATETIME> '[2020-01-01 12:00:00, 2020-12-31 12:00:00)'`
  - expected: `SELECT CAST('[2020-01-01 12:00:00, 2020-12-31 12:00:00)' AS RANGE<DATETIME>)`
  - error: `ValueError: Unexpected token: Token { token_type: Range, value: "RANGE", line: 1, col: 8, position: 7, quote_char: '\0' }`

### `rust-error` `parser: Expected RParen, got As ('AS')`

- `tests/dialects/test_bigquery.py`:1963 `test_bigquery` via `validate_identity`: `TO_JSON(STRUCT(1 AS id, [10, 20] AS cords))`
  - expected: `TO_JSON(STRUCT(1 AS id, ARRAY(10, 20) AS cords))`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 18`
- `tests/dialects/test_bigquery.py`:3345 `test_json_array` via `validate_identity`: `JSON_ARRAY(STRUCT(10 AS a, 'foo' AS b))`
  - expected: `JSON_ARRAY(STRUCT(10 AS a, 'foo' AS b))`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 22`
- `tests/dialects/test_bigquery.py`:2542 `test_json_object` via `validate_identity`: `SELECT JSON_OBJECT(['a', 'b'], [STRUCT(10 AS id, 'Red' AS color), STRUCT(20 AS id, 'Blue' AS color)]) AS json_data`
  - expected: `SELECT JSON_OBJECT(ARRAY('a', 'b'), ARRAY(STRUCT(10 AS id, 'Red' AS color), STRUCT(20 AS id, 'Blue' AS color))) AS json_data`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 43`

### `rust-error` `parser: Expected RParen, got Comma (',')`

- `tests/dialects/test_bigquery.py`:1126 `test_bigquery` via `validate_all`: `WITH cte AS (SELECT ARRAY(1, 2, 3) AS arr) SELECT EXPLODE(arr) FROM cte`
  - expected: `WITH cte AS (SELECT ARRAY(1, 2, 3) AS arr) SELECT EXPLODE(arr) FROM cte`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 28`
- `tests/dialects/test_bigquery.py`:1306 `test_bigquery` via `validate_all`: `ARRAY(1, 2, 3)`
  - expected: `ARRAY(1, 2, 3)`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 8`
- `tests/dialects/test_bigquery.py`:1306 `test_bigquery` via `validate_all`: `ARRAY(1, 2, 3)`
  - expected: `ARRAY(1, 2, 3)`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 8`

### `rust-error` `parser: Expected RParen, got FatArrow ('=>')`

- `tests/dialects/test_bigquery.py`:106 `test_bigquery` via `validate_identity`: `PARSE_JSON('{}', wide_number_mode => 'exact')`
  - expected: `'{}'`
  - error: `ValueError: Parser error: Expected RParen, got FatArrow ('=>') at line 1 col 35`
- `tests/dialects/test_bigquery.py`:111 `test_bigquery` via `validate_identity`: `SELECT SEARCH(data_to_search, 'search_query', json_scope => 'JSON_KEYS_AND_VALUES')`
  - expected: `SELECT SEARCH(data_to_search, 'search_query', json_scope => 'JSON_KEYS_AND_VALUES')`
  - error: `ValueError: Parser error: Expected RParen, got FatArrow ('=>') at line 1 col 58`
- `tests/dialects/test_bigquery.py`:114 `test_bigquery` via `validate_identity`: `SELECT SEARCH(data_to_search, 'search_query', analyzer => 'PATTERN_ANALYZER')`
  - expected: `SELECT SEARCH(data_to_search, 'search_query', analyzer => 'PATTERN_ANALYZER')`
  - error: `ValueError: Parser error: Expected RParen, got FatArrow ('=>') at line 1 col 56`

### `rust-error` `parser: Expected RParen, got Identifier ('TO')`

- `tests/dialects/test_druid.py`:10 `test_druid` via `validate_identity`: `SELECT CEIL(__time TO WEEK) FROM t`
  - expected: `SELECT CEIL(__time TO WEEK) FROM t`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('TO') at line 1 col 20`
- `tests/dialects/test_druid.py`:13 `test_druid` via `validate_identity`: `SELECT FLOOR(__time TO WEEK) FROM t`
  - expected: `SELECT FLOOR(__time TO WEEK) FROM t`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('TO') at line 1 col 21`
- `tests/dialects/test_druid.py`:21 `test_druid` via `validate_all`: `FLOOR(__time TO WEEK)`
  - expected: `FLOOR(__time TO WEEK)`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('TO') at line 1 col 14`

### `rust-error` `parser: Expected RParen, got Identifier ('VARYING')`

- `tests/dialects/test_dialect.py`:378 `test_cast` via `validate_all`: `CAST(a AS CHARACTER VARYING)`
  - expected: `CAST(a AS TEXT)`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('VARYING') at line 1 col 21`
- `tests/dialects/test_dialect.py`:378 `test_cast` via `validate_all`: `CAST(a AS CHARACTER VARYING)`
  - expected: `CAST(a AS TEXT)`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('VARYING') at line 1 col 21`
- `tests/dialects/test_dialect.py`:378 `test_cast` via `validate_all`: `CAST(a AS CHARACTER VARYING)`
  - expected: `CAST(a AS TEXT)`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('VARYING') at line 1 col 21`

### `rust-error` `parser: Expected RParen, got LParen ('(')`

- `tests/dialects/test_bigquery.py`:2295 `test_remove_precision_parameterized_types` via `validate_identity`: `INSERT INTO test (cola, colb) VALUES (CAST(7 AS STRING(10)), CAST(14 AS STRING(10)))`
  - expected: `INSERT INTO test (cola, colb) VALUES (CAST(7 AS TEXT(10)), CAST(14 AS TEXT(10)))`
  - error: `ValueError: Parser error: Expected RParen, got LParen ('(') at line 1 col 55`
- `tests/dialects/test_bigquery.py`:2303 `test_remove_precision_parameterized_types` via `validate_identity`: `SELECT CAST('1' AS STRING(10)) UNION ALL SELECT CAST('2' AS STRING(10))`
  - expected: `SELECT CAST('1' AS TEXT(10)) UNION ALL SELECT CAST('2' AS TEXT(10))`
  - error: `ValueError: Parser error: Expected RParen, got LParen ('(') at line 1 col 26`
- `tests/dialects/test_bigquery.py`:2307 `test_remove_precision_parameterized_types` via `validate_identity`: `SELECT cola FROM (SELECT CAST('1' AS STRING(10)) AS cola UNION ALL SELECT CAST('2' AS STRING(10)) AS cola)`
  - expected: `SELECT cola FROM (SELECT CAST('1' AS TEXT(10)) AS cola UNION ALL SELECT CAST('2' AS TEXT(10)) AS cola)`
  - error: `ValueError: Parser error: Expected RParen, got LParen ('(') at line 1 col 44`

### `rust-error` `parser: Expected data type, got Map`

- `tests/dialects/test_dialect.py`:287 `test_cast` via `validate_all`: `CAST(MAP('a', '1') AS MAP(TEXT, TEXT))`
  - expected: `CAST(MAP('a', '1') AS MAP<TEXT, TEXT>)`
  - error: `ValueError: Parser error: Expected data type, got Map`
- `tests/dialects/test_materialize.py`:75 `test_materialize` via `validate_identity`: `SELECT CAST(NULL AS MAP[TEXT => INT])`
  - expected: `SELECT CAST(NULL AS MAP<TEXT, INTEGER>)`
  - error: `ValueError: Parser error: Expected data type, got Map`
- `tests/dialects/test_materialize.py`:76 `test_materialize` via `validate_identity`: `SELECT CAST(NULL AS MAP[TEXT => MAP[TEXT => INT]])`
  - expected: `SELECT CAST(NULL AS MAP<TEXT, MAP<TEXT, INTEGER>>)`
  - error: `ValueError: Parser error: Expected data type, got Map`

### `rust-error` `parser: Expected identifier`

- `tests/test_transpile.py`:127 `test_comments` via `validate`: `SELECT c AS /* foo */ (a, b, c) FROM t`
  - expected: `SELECT c AS (a, b, c) /* foo */ FROM t`
  - error: `ValueError: Parser error: Expected identifier, got LParen ('(') at line 1 col 23`
- `tests/test_transpile.py`:730 `test_with` via `validate`: `WITH a AS (SELECT 1), WITH b AS (SELECT 2) SELECT *`
  - expected: `WITH a AS (SELECT 1), b AS (SELECT 2) SELECT *`
  - error: `ValueError: Parser error: Expected identifier, got With ('WITH') at line 1 col 23`
- `tests/dialects/test_bigquery.py`:95 `test_bigquery` via `validate_identity`: `assert.true(1 = 1)`
  - expected: `assert.true(1 = 1)`
  - error: `ValueError: Parser error: Expected identifier, got True ('true') at line 1 col 8`

### `rust-error` `parser: Expected statement`

- `tests/test_transpile.py`:337 `test_comments` via `validate`: `(/* 1 */ 1 ) /* 2 */`
  - expected: `(1) /* 1 */ /* 2 */`
  - error: `ValueError: Parser error: Expected statement`
- `tests/dialects/test_bigquery.py`:45 `test_bigquery` via `validate_identity`: `SAFE.SOME_RANDOM_FUNC(a, b, c)`
  - expected: `SAFE.SOME_RANDOM_FUNC(a, b, c)`
  - error: `ValueError: Parser error: Expected statement`
- `tests/dialects/test_bigquery.py`:46 `test_bigquery` via `validate_identity`: `SAFE.SUBSTR('foo', 0, -2)`
  - expected: `SAFE.SUBSTR('foo', 0, -2)`
  - error: `ValueError: Parser error: Expected statement`

### `unsupported-harness-shape` `SQLGlot expects UnsupportedError`

- `tests/dialects/test_bigquery.py`:493 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
  - error: `SQLGlot expects UnsupportedError`
- `tests/dialects/test_bigquery.py`:493 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
  - error: `SQLGlot expects UnsupportedError`
- `tests/dialects/test_bigquery.py`:493 `test_bigquery` via `validate_all`: `EDIT_DISTANCE(col1, col2, max_distance => 3)`
  - error: `SQLGlot expects UnsupportedError`

