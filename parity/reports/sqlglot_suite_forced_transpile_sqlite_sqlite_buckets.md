# SQLGlot Suite Bucket Report

Source: `parity/reports/sqlglot_suite_forced_transpile_sqlite_sqlite.jsonl`

Total rows: `15164`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 7786 |
| `mismatch` | 3990 |
| `oracle-error` | 1549 |
| `rust-error` | 1702 |
| `unsupported-harness-shape` | 137 |

## Route Buckets

| Status | Read | Write | Count |
| --- | --- | --- | ---: |
| `match` | `sqlite` | `sqlite` | 7786 |
| `mismatch` | `sqlite` | `sqlite` | 3990 |
| `rust-error` | `sqlite` | `sqlite` | 1702 |
| `oracle-error` | `sqlite` | `sqlite` | 1549 |
| `unsupported-harness-shape` | `sqlite` | `sqlite` | 137 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 5484 |
| `mismatch` | `validate_all` | 2903 |
| `match` | `validate_identity` | 2243 |
| `oracle-error` | `validate_identity` | 993 |
| `mismatch` | `validate_identity` | 991 |
| `rust-error` | `validate_all` | 851 |
| `rust-error` | `validate_identity` | 832 |
| `oracle-error` | `validate_all` | 547 |
| `unsupported-harness-shape` | `validate_all` | 122 |
| `mismatch` | `validate` | 96 |
| `match` | `validate` | 59 |
| `rust-error` | `validate` | 19 |
| `unsupported-harness-shape` | `validate_identity` | 10 |
| `oracle-error` | `validate` | 9 |
| `unsupported-harness-shape` | `validate` | 5 |

## SQL Shape Buckets

| Status | Shape | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 564 |
| `match` | `CAST()` | 439 |
| `mismatch` | `CREATE TABLE` | 268 |
| `match` | `SELECT operator multiply` | 215 |
| `match` | `SHOW` | 215 |
| `mismatch` | `SELECT` | 206 |
| `oracle-error` | `SELECT` | 198 |
| `mismatch` | `CREATE` | 193 |
| `match` | `CREATE TABLE` | 166 |
| `rust-error` | `SELECT` | 164 |
| `match` | `TRUNC()` | 162 |
| `match` | `CREATE` | 150 |
| `oracle-error` | `SELECT operator multiply` | 143 |
| `rust-error` | `SELECT operator multiply` | 139 |
| `mismatch` | `SELECT operator multiply` | 136 |
| `match` | `ALTER TABLE` | 116 |
| `oracle-error` | `CREATE TABLE` | 113 |
| `match` | `X` | 107 |
| `rust-error` | `FROM` | 104 |
| `rust-error` | `CREATE TABLE` | 99 |
| `mismatch` | `SELECT UNNEST()` | 90 |
| `match` | `SELECT DATEDIFF()` | 82 |
| `mismatch` | `CAST()` | 80 |
| `rust-error` | `CAST()` | 80 |
| `match` | `SET` | 78 |
| `mismatch` | `DATE_ADD()` | 78 |
| `match` | `WITH` | 73 |
| `match` | `DATE_TRUNC()` | 69 |
| `match` | `LOG()` | 67 |
| `match` | `GRANT` | 62 |
| `match` | `ANALYZE` | 60 |
| `match` | `SELECT CAST()` | 57 |
| `mismatch` | `TIME_STR_TO_TIME()` | 57 |
| `match` | `REGEXP_INSTR()` | 56 |
| `match` | `REVOKE` | 56 |
| `mismatch` | `WITH` | 56 |
| `match` | `A` | 55 |
| `match` | `SELECT TO_TIMESTAMP()` | 55 |
| `match` | `SELECT UNNEST()` | 55 |
| `rust-error` | `SELECT CAST()` | 54 |

## Rust/Oracle/Unsupported Error Buckets

| Status | Error Bucket | Count |
| --- | --- | ---: |
| `oracle-error` | `oracle parse: Invalid expression / Unexpected token` | 752 |
| `oracle-error` | `oracle parse: Expecting )` | 474 |
| `oracle-error` | `oracle parse: Required keyword missing` | 132 |
| `rust-error` | `parser: Expected identifier` | 121 |
| `unsupported-harness-shape` | `SQLGlot expects UnsupportedError` | 119 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: From, value: "FROM", line: 1, col: 1, position: 0, quote_char: '\0' }` | 113 |
| `rust-error` | `parser: Expected RParen, got Comma (',')` | 102 |
| `rust-error` | `parser: Expected statement` | 69 |
| `rust-error` | `parser: Expected RParen, got FatArrow ('=>')` | 67 |
| `rust-error` | `parser: Expected RParen, got As ('AS')` | 40 |
| `rust-error` | `parser: Expected RParen, got Identifier ('TO')` | 34 |
| `rust-error` | `parser: Expected RParen, got Identifier ('VARYING')` | 32 |
| `rust-error` | `parser: Expected RParen, got LParen ('(')` | 31 |
| `rust-error` | `parser: Expected RParen, got Ignore ('IGNORE')` | 30 |
| `oracle-error` | `oracle parse: The number of provided arguments (2) is greater than the maximum number of supported arguments (1)` | 24 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Range, value: "RANGE", line: 1, col: 8, position: 7, quote_char: '\0' }` | 22 |
| `rust-error` | `parser: Expected And, got Number ('10')` | 22 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: LBrace, value: "{", line: 1, col: 8, position: 7, quote_char: '\0' }` | 21 |
| `rust-error` | `parser: Expected VALUES, SELECT, or DEFAULT VALUES after INSERT` | 20 |
| `oracle-error` | `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)` | 19 |
| `rust-error` | `parser: Expected RParen, got Respect ('RESPECT')` | 19 |
| `oracle-error` | `oracle parse: Expected AS after CAST` | 18 |
| `rust-error` | `parser: Expected RParen, got Identifier ('FORMAT')` | 18 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Lateral, value: "LATERAL", line: 1, col: 17, position: 16, quote_char: '\0' }` | 17 |
| `rust-error` | `parser: Expected data type, got Map` | 17 |
| `oracle-error` | `oracle parse: Expecting (` | 16 |
| `rust-error` | `parser: Expected RParen, got Order ('ORDER')` | 15 |
| `rust-error` | `parser: Expected Join, got Union ('UNION')` | 14 |
| `unsupported-harness-shape` | `identify helper option is not supported yet` | 14 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: By, value: "BY", line: 1, col: 57, position: 56, quote_char: '\0' }` | 13 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: By, value: "BY", line: 1, col: 69, position: 68, quote_char: '\0' }` | 13 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 14, start: 13, end: 13, comments: []>` | 12 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Dot, value: ".", line: 1, col: 31, position: 30, quote_char: '\0' }` | 10 |
| `rust-error` | `parser: Expected data type, got Struct` | 10 |
| `oracle-error` | `oracle parse: Expected table name but got <Token token_type: TokenType.HASH, text: #, line: 1, col: 15, start: 14, end: 14, comments: []>` | 9 |
| `oracle-error` | `oracle parse: The number of provided arguments (3) is greater than the maximum number of supported arguments (2)` | 9 |
| `rust-error` | `parser: Expected RParen, got Having ('HAVING')` | 9 |
| `rust-error` | `parser: Expected SELECT or INSERT after WITH clause` | 9 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: All, value: "ALL", line: 1, col: 19, position: 18, quote_char: '\0' }` | 8 |
| `rust-error` | `ValueError: Unexpected token: Token { token_type: Colon, value: ":", line: 1, col: 40, position: 39, quote_char: '\0' }` | 8 |

## Mismatch Signature Buckets

| Status | Signature | Count |
| --- | --- | ---: |
| `mismatch` | `missing AS or alias rendering` | 430 |
| `mismatch` | `DDL/create-table rendering` | 255 |
| `mismatch` | `missing quoted identifier` | 245 |
| `mismatch` | `case-only rendering difference` | 212 |
| `mismatch` | `SELECT` | 136 |
| `mismatch` | `SELECT operator multiply` | 134 |
| `mismatch` | `CREATE` | 116 |
| `mismatch` | `cast/type rendering: CAST()` | 72 |
| `mismatch` | `date/time rendering: DATE_ADD()` | 69 |
| `mismatch` | `date/time rendering: TIME_STR_TO_TIME()` | 57 |
| `mismatch` | `STR_POSITION()` | 52 |
| `mismatch` | `date/time rendering: SELECT DATE_SUB()` | 49 |
| `mismatch` | `SELECT NVL2()` | 45 |
| `mismatch` | `ALTER TABLE` | 42 |
| `mismatch` | `cast/type rendering: SELECT TO_CHAR()` | 41 |
| `mismatch` | `cast/type rendering: SELECT CAST()` | 32 |
| `mismatch` | `date/time rendering: SELECT DATEADD()` | 32 |
| `mismatch` | `quote-style difference` | 32 |
| `mismatch` | `LOCATE()` | 30 |
| `mismatch` | `SELECT DECODE()` | 30 |
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
| `mismatch` | `YEAR()` | 24 |
| `mismatch` | `date/time rendering: CREATE` | 24 |
| `mismatch` | `SELECT REGEXP_EXTRACT()` | 22 |
| `mismatch` | `WITH` | 22 |
| `mismatch` | `cast/type rendering: SELECT EXTRACT()` | 22 |
| `mismatch` | `SHA256()` | 21 |
| `mismatch` | `DAY()` | 20 |
| `mismatch` | `SELECT COUNT_IF()` | 20 |
| `mismatch` | `date/time rendering: EOMONTH()` | 20 |
| `mismatch` | `STRPOS()` | 19 |

## Source Test Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 793 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 358 |
| `mismatch` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 324 |
| `mismatch` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 280 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 259 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 225 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_time` | 224 |
| `match` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 193 |
| `mismatch` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 175 |
| `match` | `tests/dialects/test_spark.py` | `test_spark` | 162 |
| `mismatch` | `tests/dialects/test_dialect.py` | `test_operators` | 159 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 150 |
| `match` | `tests/dialects/test_dialect.py` | `test_cast` | 136 |
| `rust-error` | `tests/dialects/test_snowflake.py` | `test_snowflake` | 131 |
| `match` | `tests/dialects/test_dialect.py` | `test_time` | 128 |
| `rust-error` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 118 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 117 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 113 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 108 |
| `match` | `tests/dialects/test_hive.py` | `test_hive` | 107 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 91 |
| `match` | `tests/dialects/test_oracle.py` | `test_trunc` | 88 |
| `match` | `tests/dialects/test_tsql.py` | `test_tsql` | 87 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 86 |
| `oracle-error` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 84 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 80 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 80 |
| `match` | `tests/dialects/test_dialect.py` | `test_array` | 77 |
| `mismatch` | `tests/dialects/test_presto.py` | `test_presto` | 76 |
| `oracle-error` | `tests/dialects/test_snowflake.py` | `test_match_recognize` | 75 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 70 |
| `mismatch` | `tests/dialects/test_exasol.py` | `test_datetime_functions` | 69 |
| `match` | `tests/dialects/test_dialect.py` | `test_json` | 67 |
| `match` | `tests/dialects/test_dialect.py` | `test_set_operators` | 66 |
| `match` | `tests/dialects/test_redshift.py` | `test_redshift` | 66 |
| `match` | `tests/dialects/test_snowflake.py` | `test_timestamps` | 65 |
| `match` | `tests/dialects/test_databricks.py` | `test_databricks` | 64 |
| `match` | `tests/dialects/test_dialect.py` | `test_string_functions` | 64 |
| `mismatch` | `tests/dialects/test_oracle.py` | `test_oracle` | 64 |
| `mismatch` | `tests/dialects/test_spark.py` | `test_spark` | 63 |

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

- `tests/test_transpile.py`:1008 `test_sql_security` via `validate`: `CREATE VIEW v SQL SECURITY INVOKER AS SELECT 1`
  - expected: `CREATE VIEW v AS SELECT 1`
  - actual: `CREATE VIEW v SQL SECURITY INVOKER AS SELECT 1`
- `tests/test_transpile.py`:1009 `test_sql_security` via `validate`: `CREATE VIEW v SQL SECURITY INVOKER AS SELECT 1`
  - expected: `CREATE VIEW v AS SELECT 1`
  - actual: `CREATE VIEW v SQL SECURITY INVOKER AS SELECT 1`
- `tests/test_transpile.py`:1008 `test_sql_security` via `validate`: `CREATE VIEW v SECURITY INVOKER AS SELECT 1`
  - expected: `CREATE VIEW v AS SELECT 1`
  - actual: `CREATE VIEW v SECURITY INVOKER AS SELECT 1`

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

### `mismatch` `LOCATE()`

- `tests/dialects/test_dialect.py`:2308 `test_operators` via `validate_all`: `LOCATE(needle, haystack)`
  - expected: `INSTR(haystack, needle)`
  - actual: `LOCATE(needle, haystack)`
- `tests/dialects/test_dialect.py`:2308 `test_operators` via `validate_all`: `LOCATE(needle, haystack)`
  - expected: `INSTR(haystack, needle)`
  - actual: `LOCATE(needle, haystack)`
- `tests/dialects/test_dialect.py`:2308 `test_operators` via `validate_all`: `LOCATE(needle, haystack)`
  - expected: `INSTR(haystack, needle)`
  - actual: `LOCATE(needle, haystack)`

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

### `mismatch` `SELECT DECODE()`

- `tests/dialects/test_dialect.py`:577 `test_decode` via `validate_all`: `SELECT DECODE(a, 1, 'one')`
  - expected: `SELECT CASE WHEN a = 1 THEN 'one' END`
  - actual: `SELECT DECODE(a, 1, 'one')`
- `tests/dialects/test_dialect.py`:577 `test_decode` via `validate_all`: `SELECT DECODE(a, 1, 'one')`
  - expected: `SELECT CASE WHEN a = 1 THEN 'one' END`
  - actual: `SELECT DECODE(a, 1, 'one')`
- `tests/dialects/test_dialect.py`:577 `test_decode` via `validate_all`: `SELECT DECODE(a, 1, 'one')`
  - expected: `SELECT CASE WHEN a = 1 THEN 'one' END`
  - actual: `SELECT DECODE(a, 1, 'one')`

### `mismatch` `SELECT NVL2()`

- `tests/dialects/test_dialect.py`:698 `test_nvl2` via `validate_all`: `SELECT NVL2(a, b, c)`
  - expected: `SELECT CASE WHEN NOT a IS NULL THEN b ELSE c END`
  - actual: `SELECT NVL2(a, b, c)`
- `tests/dialects/test_dialect.py`:698 `test_nvl2` via `validate_all`: `SELECT NVL2(a, b, c)`
  - expected: `SELECT CASE WHEN NOT a IS NULL THEN b ELSE c END`
  - actual: `SELECT NVL2(a, b, c)`
- `tests/dialects/test_dialect.py`:698 `test_nvl2` via `validate_all`: `SELECT NVL2(a, b, c)`
  - expected: `SELECT CASE WHEN NOT a IS NULL THEN b ELSE c END`
  - actual: `SELECT NVL2(a, b, c)`

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

### `mismatch` `STR_POSITION()`

- `tests/dialects/test_dialect.py`:2331 `test_operators` via `validate_all`: `STR_POSITION(haystack, needle)`
  - expected: `INSTR(haystack, needle)`
  - actual: `STR_POSITION(haystack, needle)`
- `tests/dialects/test_dialect.py`:2331 `test_operators` via `validate_all`: `STR_POSITION(haystack, needle)`
  - expected: `INSTR(haystack, needle)`
  - actual: `STR_POSITION(haystack, needle)`
- `tests/dialects/test_dialect.py`:2331 `test_operators` via `validate_all`: `STR_POSITION(haystack, needle)`
  - expected: `INSTR(haystack, needle)`
  - actual: `STR_POSITION(haystack, needle)`

### `mismatch` `case-only rendering difference`

- `tests/test_transpile.py`:704 `test_extract` via `validate`: `extract(week from current_date + 2)`
  - expected: `EXTRACT(WEEK FROM CURRENT_DATE + 2)`
  - actual: `EXTRACT(WEEK FROM current_date + 2)`
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
- `tests/dialects/test_bigquery.py`:236 `test_bigquery` via `validate_identity`: `CAST(x AS TIMESTAMPTZ)`
  - expected: `CAST(x AS TIMESTAMPTZ)`
  - actual: `CAST(x AS TIMESTAMP WITH TIME ZONE)`
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
- `tests/dialects/test_clickhouse.py`:65 `test_clickhouse` via `validate_identity`: `SELECT CAST(x AS Tuple(String, Array(Nullable(Float64))))`
  - expected: `SELECT CAST(x AS Tuple(STRING, ARRAY(NULLABLE(Float64))))`
  - actual: `SELECT CAST(x AS TUPLE)`

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

- `tests/test_transpile.py`:52 `test_alias` via `validate`: `SELECT x "union"`
  - expected: `SELECT x AS "union"`
  - actual: `SELECT x`
- `tests/test_transpile.py`:52 `test_alias` via `validate`: `SELECT x "from"`
  - expected: `SELECT x AS "from"`
  - actual: `SELECT x`
- `tests/test_transpile.py`:52 `test_alias` via `validate`: `SELECT x "join"`
  - expected: `SELECT x AS "join"`
  - actual: `SELECT x`

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

### `oracle-error` `oracle parse: The number of provided arguments (4) is greater than the maximum number of supported arguments (2)`

- `tests/dialects/test_clickhouse.py`:90 `test_clickhouse` via `validate_identity`: `'a' IN mapKeys(map('a', 1, 'b', 2))`
  - error: `ParseError: The number of provided arguments (4) is greater than the maximum number of supported arguments (2). Line 1, Col: 34. 'a' IN mapKeys(map('a', 1, 'b', 2))`
- `tests/dialects/test_clickhouse.py`:1765 `test_functions` via `validate_identity`: `SELECT TRANSFORM(foo, [1, 2], ['first', 'second'], 'default') FROM table`
  - error: `ParseError: The number of provided arguments (4) is greater than the maximum number of supported arguments (2). Line 1, Col: 61. SELECT TRANSFORM(foo, [1, 2], ['first', 'second'], 'default') FROM table`
- `tests/dialects/test_hive.py`:751 `test_hive` via `validate_all`: `map(a, b, c, d)`
  - error: `ParseError: The number of provided arguments (4) is greater than the maximum number of supported arguments (2). Line 1, Col: 15. map(a, b, c, d)`

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

### `rust-error` `parser: Expected RParen, got As ('AS')`

- `tests/dialects/test_bigquery.py`:108 `test_bigquery` via `validate_identity`: `STRUCT(values AS value)`
  - expected: `STRUCT(values AS value)`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 15`
- `tests/dialects/test_bigquery.py`:173 `test_bigquery` via `validate_identity`: `SAFE_CAST(encrypted_value AS STRING FORMAT 'BASE64')`
  - expected: `CAST(encrypted_value AS TEXT FORMAT 'BASE64')`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 27`
- `tests/dialects/test_bigquery.py`:180 `test_bigquery` via `validate_identity`: `SAFE_CAST(x AS STRING)`
  - expected: `CAST(x AS TEXT)`
  - error: `ValueError: Parser error: Expected RParen, got As ('AS') at line 1 col 13`

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

### `rust-error` `parser: Expected RParen, got Ignore ('IGNORE')`

- `tests/test_transpile.py`:518 `test_comments` via `validate`: `-- comment SOME_FUNC(arg IGNORE NULLS) OVER (PARTITION BY foo ORDER BY bla) AS col`
  - expected: `SOME_FUNC(arg) OVER (PARTITION BY foo ORDER BY bla) AS col /* comment */`
  - error: `ValueError: Parser error: Expected RParen, got Ignore ('IGNORE') at line 2 col 15`
- `tests/dialects/test_bigquery.py`:3731 `test_approx_quantiles` via `validate_identity`: `APPROX_QUANTILES(x, 2 IGNORE NULLS)`
  - expected: `APPROX_QUANTILES(x, 2)`
  - error: `ValueError: Parser error: Expected RParen, got Ignore ('IGNORE') at line 1 col 23`
- `tests/dialects/test_bigquery.py`:3803 `test_approx_quantiles_to_duckdb` via `validate_all`: `APPROX_QUANTILES(x, 2 IGNORE NULLS)`
  - expected: `APPROX_QUANTILES(x, 2)`
  - error: `ValueError: Parser error: Expected RParen, got Ignore ('IGNORE') at line 1 col 23`

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

### `rust-error` `parser: Expected identifier`

- `tests/test_transpile.py`:51 `test_alias` via `validate`: `SELECT x AS union`
  - expected: `SELECT x AS union`
  - error: `ValueError: Parser error: Expected identifier, got Union ('union') at line 1 col 13`
- `tests/test_transpile.py`:51 `test_alias` via `validate`: `SELECT x AS from`
  - expected: `SELECT x AS from`
  - error: `ValueError: Parser error: Expected identifier, got From ('from') at line 1 col 13`
- `tests/test_transpile.py`:51 `test_alias` via `validate`: `SELECT x AS join`
  - expected: `SELECT x AS join`
  - error: `ValueError: Parser error: Expected identifier, got Join ('join') at line 1 col 13`

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

