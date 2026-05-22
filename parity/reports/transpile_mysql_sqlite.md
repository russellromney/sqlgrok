# SQLGlot Import Report

Source: `parity/reports/transpile_mysql_sqlite.jsonl`

Total candidates: `496`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 82 |
| `mismatch` | 85 |
| `oracle-error` | 7 |
| `rust-error` | 322 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `mismatch` | `SELECT` | 71 |
| `match` | `SELECT` | 62 |
| `rust-error` | `CREATE TABLE` | 42 |
| `rust-error` | `SELECT` | 35 |
| `rust-error` | `SET` | 34 |
| `rust-error` | `ALTER TABLE` | 32 |
| `rust-error` | `CAST()` | 16 |
| `match` | `CREATE TABLE` | 12 |
| `rust-error` | `DATE_ADD()` | 12 |
| `rust-error` | `ANALYZE` | 8 |
| `rust-error` | `GROUP_CONCAT()` | 8 |
| `oracle-error` | `CREATE TABLE` | 7 |
| `rust-error` | `CREATE` | 7 |
| `rust-error` | `GRANT` | 7 |
| `rust-error` | `REVOKE` | 7 |
| `rust-error` | `MATCH()` | 6 |
| `rust-error` | `ALTER` | 5 |
| `rust-error` | `X` | 5 |
| `match` | `INSERT` | 4 |
| `mismatch` | `CREATE TABLE` | 4 |
| `rust-error` | `CHAR()` | 4 |
| `mismatch` | `DELETE` | 3 |
| `rust-error` | `DESCRIBE` | 3 |
| `rust-error` | `REGEXP_INSTR()` | 3 |
| `rust-error` | `SHOW` | 3 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_ddl` | 87 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_identity` | 62 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_identity` | 20 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_mysql` | 20 |
| `match` | `tests/dialects/test_mysql.py` | `test_identity` | 13 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_mysql_time` | 12 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_valid_interval_units` | 11 |
| `match` | `tests/dialects/test_mysql.py` | `test_canonical_functions` | 10 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 10 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_ddl` | 10 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql` | 10 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_convert` | 10 |
| `match` | `tests/dialects/test_mysql.py` | `test_date_format` | 9 |
| `match` | `tests/dialects/test_mysql.py` | `test_ddl` | 9 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_analyze` | 8 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_escape` | 8 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_types` | 8 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql_time` | 7 |
| `oracle-error` | `tests/dialects/test_mysql.py` | `test_ddl` | 7 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_grant` | 7 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_revoke` | 7 |
| `rust-error` | `tests/dialects/test_mysql.py` | `test_match_against` | 6 |
| `rust-error` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 5 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_mysql.py` | `test_null_ordering_simulation_resolves_ordered_against_projection` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-0709-test-bigquery`: `SELECT MAKETIME(15, 30, 00)`
  - expected: `SELECT TIME_FROM_PARTS(15, 30, 00)`
  - actual: `SELECT MAKETIME(15, 30, 00)`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-clickhouse-0444-test-clickhouse`: `SELECT 1 XOR 0`
  - expected: `SELECT 1 XOR 0`
  - actual: `SELECT 1`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-clickhouse-0461-test-clickhouse`: `SELECT 1 XOR 0 XOR 1`
  - expected: `SELECT 1 XOR 0 XOR 1`
  - actual: `SELECT 1`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-dialect-2881-test-alias`: `SELECT 1 'foo'`
  - expected: `SELECT 1 AS "foo"`
  - actual: `SELECT 1`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-dialect-2976-test-hash-comments`: `SELECT 1 # arbitrary content,,, until end-of-line`
  - expected: `SELECT 1 /* arbitrary content,,, until end-of-line */`
  - actual: `SELECT 1`

### `oracle-error`

- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-0249-test-ddl`: `CREATE TABLE t (id INT, created_at DATE) PARTITION BY RANGE (id) (PARTITION p0 VALUES LESS THAN (10), PARTITION p1 VALUES LESS THAN (20), PARTITION p2 VALUES LESS THAN (MAXVALUE))`
  - error: `<class 'sqlglot.expressions.properties.PartitionByRangeProperty'>`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-0252-test-ddl`: `CREATE TABLE t (id INT, name VARCHAR(50)) PARTITION BY RANGE (id) (PARTITION p0 VALUES LESS THAN (100))`
  - error: `<class 'sqlglot.expressions.properties.PartitionByRangeProperty'>`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-0256-test-ddl`: `CREATE TABLE orders (id INT, order_date DATE) PARTITION BY RANGE (YEAR(order_date)) (PARTITION p2023 VALUES LESS THAN (2024), PARTITION p2024 VALUES LESS THAN (2025), PARTITION pmax VALUES LESS THAN (MAXVALUE))`
  - error: `<class 'sqlglot.expressions.properties.PartitionByRangeProperty'>`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-0259-test-ddl`: `CREATE TABLE sales (id INT, sale_date DATE) PARTITION BY RANGE (MONTH(sale_date)) (PARTITION q1 VALUES LESS THAN (4), PARTITION q2 VALUES LESS THAN (7), PARTITION q3 VALUES LESS THAN (10), PARTITION q4 VALUES LESS THAN (13))`
  - error: `<class 'sqlglot.expressions.properties.PartitionByRangeProperty'>`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-mysql-0263-test-ddl`: `CREATE TABLE t (id INT, region VARCHAR(10)) PARTITION BY LIST (id) (PARTITION p_east VALUES IN (1, 2, 3), PARTITION p_west VALUES IN (4, 5, 6))`
  - error: `<class 'sqlglot.expressions.properties.PartitionByListProperty'>`

### `rust-error`

- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-1171-test-bigquery`: `REGEXP_LIKE('foo', '.*')`
  - expected: `REGEXP_LIKE('foo', '.*')`
  - error: `Unexpected token: Token { token_type: Identifier, value: "REGEXP_LIKE", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-1530-test-bigquery`: `DATEDIFF(CAST('2010-07-07' AS DATE), CAST('2008-12-25' AS DATE))`
  - expected: `CAST((JULIANDAY(DATE('2010-07-07')) - JULIANDAY(DATE('2008-12-25'))) AS INTEGER)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "DATEDIFF", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-3558-test-bit-aggs`: `BIT_AND(x)`
  - expected: `BITWISE_AND_AGG(x)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "BIT_AND", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-3578-test-bit-aggs`: `BIT_OR(x)`
  - expected: `BITWISE_OR_AGG(x)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "BIT_OR", line: 1, col: 1, position: 0, quote_char: '\0' }`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-3598-test-bit-aggs`: `BIT_XOR(x)`
  - expected: `BITWISE_XOR_AGG(x)`
  - error: `Unexpected token: Token { token_type: Identifier, value: "BIT_XOR", line: 1, col: 1, position: 0, quote_char: '\0' }`

