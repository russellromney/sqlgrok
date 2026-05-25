# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_postgres_sqlite.jsonl`

Total rows: `15164`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 8816 |
| `mismatch` | 3474 |
| `oracle-error` | 1457 |
| `rust-error` | 1280 |
| `unsupported-harness-shape` | 137 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `postgres` | `sqlite` | 8816 |
| `mismatch` | `postgres` | `sqlite` | 3474 |
| `oracle-error` | `postgres` | `sqlite` | 1457 |
| `rust-error` | `postgres` | `sqlite` | 1280 |
| `unsupported-harness-shape` | `postgres` | `sqlite` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 6212 |
| `match` | `validate_identity` | 2511 |
| `mismatch` | `validate_all` | 2466 |
| `oracle-error` | `validate_identity` | 949 |
| `mismatch` | `validate_identity` | 934 |
| `rust-error` | `validate_identity` | 665 |
| `rust-error` | `validate_all` | 605 |
| `oracle-error` | `validate_all` | 502 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `match` | `validate` | 93 |
| `mismatch` | `validate` | 74 |
| `rust-error` | `validate` | 10 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 6 |
| `unsupported-harness-shape` | `validate` | 5 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 641 |
| `match` | `CAST()` | 500 |
| `mismatch` | `CREATE TABLE` | 269 |
| `match` | `SELECT operator multiply` | 245 |
| `match` | `SHOW` | 215 |
| `oracle-error` | `SELECT` | 213 |
| `mismatch` | `CREATE` | 195 |
| `match` | `CREATE TABLE` | 167 |
| `match` | `CREATE` | 162 |
| `match` | `TRUNC()` | 162 |
| `oracle-error` | `SELECT operator multiply` | 150 |
| `rust-error` | `SELECT` | 144 |
| `mismatch` | `SELECT` | 134 |
| `mismatch` | `SELECT operator multiply` | 120 |
| `rust-error` | `SELECT operator multiply` | 118 |
| `mismatch` | `SELECT UNNEST()` | 116 |
| `match` | `ALTER TABLE` | 115 |
| `match` | `X` | 113 |
| `oracle-error` | `CREATE TABLE` | 106 |
| `rust-error` | `CREATE TABLE` | 104 |
| `match` | `WITH` | 87 |
| `match` | `SELECT DATEDIFF()` | 83 |
| `match` | `SET` | 78 |
| `mismatch` | `DATE_ADD()` | 78 |
| `match` | `DATE_TRUNC()` | 71 |
| `match` | `LOG()` | 67 |
| `match` | `GRANT` | 65 |
| `rust-error` | `CAST()` | 65 |
| `match` | `SELECT CAST()` | 63 |
| `mismatch` | `WITH` | 62 |
| `match` | `ANALYZE` | 60 |
| `match` | `A` | 59 |
| `match` | `REVOKE` | 59 |
| `mismatch` | `TIME_STR_TO_TIME()` | 57 |
| `match` | `REGEXP_INSTR()` | 56 |
| `mismatch` | `CAST()` | 56 |
| `match` | `SELECT SUM()` | 55 |
| `rust-error` | `SELECT CAST()` | 54 |
| `match` | `SELECT UNNEST()` | 53 |
| `rust-error` | `FROM` | 52 |

## Rust/Oracle/Unsupported Error Buckets

| Status | Error Bucket | Count |
| --- | --- | ---: |
| `oracle-error` | `oracle parse: Invalid expression / Unexpected token` | 711 |
| `oracle-error` | `oracle parse: Expecting )` | 372 |
| `oracle-error` | `oracle parse: Required keyword missing` | 147 |
| `unsupported-harness-shape` | `SQLGlot expects UnsupportedError` | 119 |
| `rust-error` | `parser: Expected identifier` | 92 |
| `rust-error` | `parser: Expected statement` | 76 |
| `oracle-error` | `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)` | 46 |
| `rust-error` | `parser: Expected RParen, got LParen ('(')` | 43 |
| `rust-error` | `parser: Expected RParen, got As ('AS')` | 36 |
| `rust-error` | `parser: Expected RParen, got Identifier ('TO')` | 34 |
| `rust-error` | `parser: Expected RParen, got Comma (',')` | 28 |
| `rust-error` | `parser: Expected data type, got Map` | 24 |
| `rust-error` | `parser: Expected And, got Number ('10')` | 22 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }` | 21 |
| `oracle-error` | `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)` | 19 |
| `rust-error` | `parser: Expected Join, got Union ('UNION')` | 18 |
| `rust-error` | `parser: Expected RParen, got Identifier ('FORMAT')` | 18 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Lateral, value: "LATERAL", line: 1, col: 17, position: 16, quote_char: '\0' }` | 17 |
| `oracle-error` | `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)` | 16 |
| `rust-error` | `parser: Expected VALUES, SELECT, or DEFAULT VALUES after INSERT` | 16 |
| `rust-error` | `parser: Expected RParen, got Order ('ORDER')` | 15 |
| `oracle-error` | `oracle parse: Expecting (` | 14 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: As, value: "AS", line: 1, col: 29, position: 28, quote_char: '\0' }` | 14 |
| `rust-error` | `parser: Expected RParen, got FatArrow ('=>')` | 14 |
| `unsupported-harness-shape` | `identify helper option is not supported yet` | 14 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: By, value: "BY", line: 1, col: 57, position: 56, quote_char: '\0' }` | 13 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: By, value: "BY", line: 1, col: 69, position: 68, quote_char: '\0' }` | 13 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 14, start: 13, end: 13, comments: []>` | 12 |
| `rust-error` | `parser: Expected data type, got Struct` | 12 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }` | 10 |
| `rust-error` | `parser: Expected RParen, got Having ('HAVING')` | 10 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 15, start: 14, end: 14, comments: []>` | 9 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.L_BRACKET, text: [, line: 1, col: 14, start: 13, end: 13, comments: []>` | 8 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: All, value: "ALL", line: 1, col: 19, position: 18, quote_char: '\0' }` | 8 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: As, value: "AS", line: 1, col: 33, position: 32, quote_char: '\0' }` | 8 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }` | 8 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: On, value: "ON", line: 1, col: 14, position: 13, quote_char: '\0' }` | 8 |
| `rust-error` | `parser: Expected Join, got Identifier ('DIRECTED')` | 8 |
| `rust-error` | `parser: Expected RParen, got As ('as')` | 8 |
| `rust-error` | `parser: Expected RParen, got Dot ('.')` | 8 |

## Mismatch Signature Buckets

| Status | Signature | Count |
| --- | --- | ---: |
| `mismatch` | `DDL/create-table rendering` | 272 |
| `mismatch` | `missing AS or alias rendering` | 271 |
| `mismatch` | `case-only rendering difference` | 219 |
| `mismatch` | `SELECT` | 124 |
| `mismatch` | `CREATE` | 121 |
| `mismatch` | `SELECT operator multiply` | 118 |
| `mismatch` | `SELECT UNNEST()` | 77 |
| `mismatch` | `date/time rendering: DATE_ADD()` | 69 |
| `mismatch` | `date/time rendering: TIME_STR_TO_TIME()` | 57 |
| `mismatch` | `date/time rendering: SELECT DATE_SUB()` | 49 |
| `mismatch` | `cast/type rendering: CAST()` | 46 |
| `mismatch` | `ALTER TABLE` | 45 |
| `mismatch` | `date/time rendering: SELECT DATEADD()` | 32 |
| `mismatch` | `DECLARE` | 30 |
| `mismatch` | `WITH` | 29 |
| `mismatch` | `date/time rendering: SELECT DATE_ADD()` | 29 |
| `mismatch` | `date/time rendering: SELECT DATE_FORMAT()` | 29 |
| `mismatch` | `LEVENSHTEIN()` | 28 |
| `mismatch` | `MEDIAN()` | 28 |
| `mismatch` | `REGEXP_EXTRACT()` | 28 |
| `mismatch` | `REGEXP_REPLACE()` | 27 |
| `mismatch` | `date/time rendering: CREATE` | 27 |
| `mismatch` | `json rendering: JSON_EXTRACT()` | 27 |
| `mismatch` | `MONTH()` | 26 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 26 |
| `mismatch` | `date/time rendering: SELECT UNNEST()` | 26 |
| `mismatch` | `YEAR()` | 24 |
| `mismatch` | `SELECT REGEXP_EXTRACT()` | 22 |
| `mismatch` | `cast/type rendering: SELECT EXTRACT()` | 22 |
| `mismatch` | `A` | 21 |
| `mismatch` | `DAY()` | 20 |
| `mismatch` | `SELECT ARRAY_AGG()` | 20 |
| `mismatch` | `SELECT COUNT_IF()` | 20 |
| `mismatch` | `date/time rendering: EOMONTH()` | 20 |
| `mismatch` | `cast/type rendering: SELECT TO_CHAR()` | 19 |
| `mismatch` | `SELECT operator index` | 18 |
| `mismatch` | `date/time rendering: SELECT DATE_TRUNC()` | 17 |
| `mismatch` | `date/time rendering: STR_TO_TIME()` | 17 |
| `mismatch` | `LTRIM()` | 16 |
| `mismatch` | `MOD()` | 16 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 859 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 443 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 349 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 309 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 285 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 228 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 227 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 200 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 191 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 180 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 170 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 135 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 125 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 110 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 108 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 101 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 98 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 97 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 97 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 88 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 85 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 84 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 84 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 84 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 83 |
| `oracle-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 82 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 78 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 74 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 72 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 72 |
| `mismatch` | `tests/dialects/test_presto.py` | `test_presto` | 71 |
| `match` | `tests/dialects/test_duckdb.py` | `test_time` | 67 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 67 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 67 |
| `match` | `tests/dialects/test_databricks.py` | `test_databricks` | 65 |
| `match` | `tests/dialects/test_exasol.py` | `test_scalar` | 65 |

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

### `mismatch` `LEVENSHTEIN()`

- `tests/dialects/test_dialect.py`:2549 `test_operators` via `validate_all`: `LEVENSHTEIN(col1, col2)`
  - expected: `EDITDIST3(col1, col2)`
  - actual: `LEVENSHTEIN(col1, col2)`
- `tests/dialects/test_dialect.py`:2549 `test_operators` via `validate_all`: `LEVENSHTEIN(col1, col2)`
  - expected: `EDITDIST3(col1, col2)`
  - actual: `LEVENSHTEIN(col1, col2)`
- `tests/dialects/test_dialect.py`:2549 `test_operators` via `validate_all`: `LEVENSHTEIN(col1, col2)`
  - expected: `EDITDIST3(col1, col2)`
  - actual: `LEVENSHTEIN(col1, col2)`

### `mismatch` `MEDIAN()`

- `tests/dialects/test_dialect.py`:3998 `test_median` via `validate_all`: `MEDIAN(x)`
  - expected: `PERCENTILE_CONT(x, 0.5)`
  - actual: `MEDIAN(x)`
- `tests/dialects/test_dialect.py`:3998 `test_median` via `validate_all`: `MEDIAN(x)`
  - expected: `PERCENTILE_CONT(x, 0.5)`
  - actual: `MEDIAN(x)`
- `tests/dialects/test_dialect.py`:3998 `test_median` via `validate_all`: `MEDIAN(x)`
  - expected: `PERCENTILE_CONT(x, 0.5)`
  - actual: `MEDIAN(x)`

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

- `tests/test_transpile.py`:673 `test_types` via `validate`: `x::user_defined_type`
  - expected: `CAST(x AS user_defined_type)`
  - actual: `CAST(x AS USER_DEFINED_TYPE)`
- `tests/dialects/test_bigquery.py`:759 `test_bigquery` via `validate_all`: `TIMESTAMPDIFF(month, b, a)`
  - expected: `TIMESTAMPDIFF(month, b, A)`
  - actual: `TIMESTAMPDIFF(month, b, a)`
- `tests/dialects/test_bigquery.py`:759 `test_bigquery` via `validate_all`: `TIMESTAMPDIFF(month, b, a)`
  - expected: `TIMESTAMPDIFF(month, b, A)`
  - actual: `TIMESTAMPDIFF(month, b, a)`

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

- `tests/test_transpile.py`:836 `test_time` via `validate`: `TIME_TO_TIME_STR(x)`
  - expected: `CAST(x AS TEXT)`
  - actual: `TIME_TO_TIME_STR(x)`
- `tests/dialects/test_bigquery.py`:3819 `test_bignumeric` via `validate_all`: `SELECT BIGNUMERIC '1'`
  - expected: `SELECT CAST('1' AS BIGDECIMAL)`
  - actual: `SELECT BIGNUMERIC`
- `tests/dialects/test_bigquery.py`:3819 `test_bignumeric` via `validate_all`: `SELECT BIGNUMERIC '1'`
  - expected: `SELECT CAST('1' AS BIGDECIMAL)`
  - actual: `SELECT BIGNUMERIC`

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

### `rust-error` `ValueError: Unexpected token: Token { token_type: Lateral, value: "LATERAL", line: 1, col: 17, position: 16, quote_char: '\0' }`

- `tests/dialects/test_hive.py`:281 `test_lateral_view` via `validate_all`: `SELECT a FROM x LATERAL VIEW EXPLODE(y) t AS a`
  - expected: `SELECT a FROM x LATERAL VIEW EXPLODE(y) t AS a`
  - error: `ValueError: Unexpected token: Token { token_type: Lateral, value: "LATERAL", line: 1, col: 17, position: 16, quote_char: '\0' }`
- `tests/dialects/test_hive.py`:281 `test_lateral_view` via `validate_all`: `SELECT a FROM x LATERAL VIEW EXPLODE(y) t AS a`
  - expected: `SELECT a FROM x LATERAL VIEW EXPLODE(y) t AS a`
  - error: `ValueError: Unexpected token: Token { token_type: Lateral, value: "LATERAL", line: 1, col: 17, position: 16, quote_char: '\0' }`
- `tests/dialects/test_hive.py`:281 `test_lateral_view` via `validate_all`: `SELECT a FROM x LATERAL VIEW EXPLODE(y) t AS a`
  - expected: `SELECT a FROM x LATERAL VIEW EXPLODE(y) t AS a`
  - error: `ValueError: Unexpected token: Token { token_type: Lateral, value: "LATERAL", line: 1, col: 17, position: 16, quote_char: '\0' }`

### `rust-error` `parser: Expected And, got Number ('10')`

- `tests/dialects/test_dialect.py`:4122 `test_between` via `validate_all`: `SELECT x BETWEEN SYMMETRIC 10 AND 2`
  - expected: `SELECT (x BETWEEN 10 AND 2 OR x BETWEEN 2 AND 10)`
  - error: `ValueError: Parser error: Expected And, got Number ('10') at line 1 col 28`
- `tests/dialects/test_dialect.py`:4122 `test_between` via `validate_all`: `SELECT x BETWEEN SYMMETRIC 10 AND 2`
  - expected: `SELECT (x BETWEEN 10 AND 2 OR x BETWEEN 2 AND 10)`
  - error: `ValueError: Parser error: Expected And, got Number ('10') at line 1 col 28`
- `tests/dialects/test_dialect.py`:4122 `test_between` via `validate_all`: `SELECT x BETWEEN SYMMETRIC 10 AND 2`
  - expected: `SELECT (x BETWEEN 10 AND 2 OR x BETWEEN 2 AND 10)`
  - error: `ValueError: Parser error: Expected And, got Number ('10') at line 1 col 28`

### `rust-error` `parser: Expected Join, got Union ('UNION')`

- `tests/dialects/test_bigquery.py`:3095 `test_set_operations` via `validate_identity`: `SELECT 1 AS foo INNER UNION ALL SELECT 3 AS foo, 4 AS bar`
  - expected: `SELECT 1 AS foo INNER UNION ALL SELECT 3 AS foo, 4 AS bar`
  - error: `ValueError: Parser error: Expected Join, got Union ('UNION') at line 1 col 23`
- `tests/dialects/test_bigquery.py`:3105 `test_set_operations` via `validate_identity`: `SELECT 1 AS foo LEFT UNION ALL SELECT 3 AS foo, 4 AS bar`
  - expected: `SELECT 1 AS foo LEFT UNION ALL SELECT 3 AS foo, 4 AS bar`
  - error: `ValueError: Parser error: Expected Join, got Union ('UNION') at line 1 col 22`
- `tests/dialects/test_bigquery.py`:3105 `test_set_operations` via `validate_identity`: `SELECT 1 AS foo LEFT UNION ALL BY NAME SELECT 3 AS foo, 4 AS bar`
  - expected: `SELECT 1 AS foo LEFT UNION ALL BY NAME SELECT 3 AS foo, 4 AS bar`
  - error: `ValueError: Parser error: Expected Join, got Union ('UNION') at line 1 col 22`

### `rust-error` `parser: Expected RParen, got As ('AS')`

- `tests/dialects/test_bigquery.py`:108 `test_bigquery` via `validate_identity`: `STRUCT(values AS value)`
  - expected: `STRUCT(values AS value)`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 15`
- `tests/dialects/test_bigquery.py`:1963 `test_bigquery` via `validate_identity`: `TO_JSON(STRUCT(1 AS id, [10, 20] AS cords))`
  - expected: `TO_JSON(STRUCT(1 AS id, ARRAY(10, 20) AS cords))`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 18`
- `tests/dialects/test_bigquery.py`:3345 `test_json_array` via `validate_identity`: `JSON_ARRAY(STRUCT(10 AS a, 'foo' AS b))`
  - expected: `JSON_ARRAY(STRUCT(10 AS a, 'foo' AS b))`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 22`

### `rust-error` `parser: Expected RParen, got Comma (',')`

- `tests/dialects/test_dialect.py`:4161 `test_like_quantifiers` via `validate_all`: `SELECT col LIKE ALL (x, y, z)`
  - expected: `SELECT col LIKE ALL (x, y, z)`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 23`
- `tests/dialects/test_dialect.py`:4161 `test_like_quantifiers` via `validate_all`: `SELECT col LIKE ALL (x, y, z)`
  - expected: `SELECT col LIKE ALL (x, y, z)`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 23`
- `tests/dialects/test_dialect.py`:4161 `test_like_quantifiers` via `validate_all`: `SELECT col LIKE ALL (x, y, z)`
  - expected: `SELECT col LIKE ALL (x, y, z)`
  - error: `ValueError: Parser error: Expected RParen, got Comma (',') at line 1 col 23`

### `rust-error` `parser: Expected RParen, got Identifier ('FORMAT')`

- `tests/dialects/test_bigquery.py`:172 `test_bigquery` via `validate_identity`: `SELECT CAST(CURRENT_DATE AS STRING FORMAT 'DAY') AS current_day`
  - expected: `SELECT CAST(CURRENT_DATE AS TEXT FORMAT 'DAY') AS current_day`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('FORMAT') at line 1 col 36`
- `tests/dialects/test_bigquery.py`:174 `test_bigquery` via `validate_identity`: `CAST(encrypted_value AS STRING FORMAT 'BASE64')`
  - expected: `CAST(encrypted_value AS TEXT FORMAT 'BASE64')`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('FORMAT') at line 1 col 32`
- `tests/dialects/test_bigquery.py`:1106 `test_bigquery` via `validate_all`: `SELECT CAST('20201225' AS TIMESTAMP FORMAT 'YYYYMMDD' AT TIME ZONE 'America/New_York')`
  - expected: `SELECT STR_TO_TIME('20201225', '%Y%m%d')`
  - error: `ValueError: Parser error: Expected RParen, got Identifier ('FORMAT') at line 1 col 37`

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

### `rust-error` `parser: Expected VALUES, SELECT, or DEFAULT VALUES after INSERT`

- `tests/dialects/test_clickhouse.py`:601 `test_clickhouse` via `validate_identity`: `INSERT INTO TABLE FUNCTION hdfs('hdfs://hdfs1:9000/test', 'TSV', 'name String, column2 UInt32, column3 UInt32') VALUES ('test', 1, 2)`
  - expected: `INSERT INTO FUNCTION HDFS('hdfs://hdfs1:9000/test', 'TSV', 'name String, column2 UInt32, column3 UInt32') VALUES ('test', 1, 2)`
  - error: `ValueError: Parser error: Expected VALUES, SELECT, or DEFAULT VALUES after INSERT`
- `tests/dialects/test_clickhouse.py`:758 `test_clickhouse_values` via `validate_identity`: `INSERT INTO t (col1, col2) FORMAT Values('abcd', 1234)`
  - expected: `INSERT INTO t (col1, col2) VALUES ('abcd', 1234)`
  - error: `ValueError: Parser error: Expected VALUES, SELECT, or DEFAULT VALUES after INSERT`
- `tests/dialects/test_databricks.py`:45 `test_databricks` via `validate_identity`: `INSERT INTO a REPLACE WHERE cond VALUES (1), (2)`
  - expected: `INSERT INTO a REPLACE WHERE cond VALUES (1), (2)`
  - error: `ValueError: Parser error: Expected VALUES, SELECT, or DEFAULT VALUES after INSERT`

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

