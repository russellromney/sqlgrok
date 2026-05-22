# SQLGlot Import Report

Source: `parity/reports/transpile_mysql_sqlite.jsonl`

Total candidates: `496`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 232 |
| `mismatch` | 257 |
| `oracle-error` | 7 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `mismatch` | `SELECT` | 87 |
| `match` | `SELECT` | 81 |
| `mismatch` | `CREATE TABLE` | 45 |
| `match` | `SET` | 33 |
| `mismatch` | `ALTER TABLE` | 26 |
| `match` | `CREATE TABLE` | 13 |
| `mismatch` | `DATE_ADD()` | 12 |
| `mismatch` | `CAST()` | 9 |
| `match` | `ALTER TABLE` | 8 |
| `match` | `ANALYZE` | 8 |
| `match` | `GROUP_CONCAT()` | 8 |
| `match` | `CAST()` | 7 |
| `match` | `GRANT` | 7 |
| `match` | `REVOKE` | 7 |
| `oracle-error` | `CREATE TABLE` | 7 |
| `match` | `INSERT` | 6 |
| `mismatch` | `MATCH()` | 6 |
| `match` | `ALTER` | 5 |
| `match` | `CHAR()` | 4 |
| `match` | `CREATE` | 4 |
| `mismatch` | `DELETE` | 4 |
| `mismatch` | `X` | 4 |
| `match` | `REGEXP_INSTR()` | 3 |
| `mismatch` | `CREATE` | 3 |
| `mismatch` | `DESCRIBE` | 3 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_ddl` | 80 |
| `match` | `tests/dialects/test_mysql.py` | `test_identity` | 64 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_identity` | 31 |
| `match` | `tests/dialects/test_mysql.py` | `test_ddl` | 26 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 22 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql` | 18 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql_time` | 16 |
| `match` | `tests/dialects/test_mysql.py` | `test_canonical_functions` | 11 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_valid_interval_units` | 11 |
| `match` | `tests/dialects/test_mysql.py` | `test_date_format` | 9 |
| `match` | `tests/dialects/test_mysql.py` | `test_analyze` | 8 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_convert` | 8 |
| `match` | `tests/dialects/test_mysql.py` | `test_grant` | 7 |
| `match` | `tests/dialects/test_mysql.py` | `test_revoke` | 7 |
| `oracle-error` | `tests/dialects/test_mysql.py` | `test_ddl` | 7 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_match_against` | 6 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_types` | 6 |
| `match` | `tests/dialects/test_mysql.py` | `test_escape` | 5 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_json_value` | 5 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bit_aggs` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_mysql.py` | `test_null_ordering_simulation_resolves_ordered_against_projection` | 4 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_explain` | 4 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 4 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql_time_python311` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-1530-test-bigquery`: `DATEDIFF(CAST('2010-07-07' AS DATE), CAST('2008-12-25' AS DATE))`
  - expected: `CAST((JULIANDAY(DATE('2010-07-07')) - JULIANDAY(DATE('2008-12-25'))) AS INTEGER)`
  - actual: `CAST((JULIANDAY(CAST('2010-07-07' AS DATE)) - JULIANDAY(CAST('2008-12-25' AS DATE))) AS INTEGER)`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-0709-test-bigquery`: `SELECT MAKETIME(15, 30, 00)`
  - expected: `SELECT TIME_FROM_PARTS(15, 30, 00)`
  - actual: `SELECT MAKETIME(15, 30, 00)`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-bigquery-0759-test-bigquery`: `TIMESTAMPDIFF(month, b, a)`
  - expected: `TIMESTAMPDIFF(a, b, MONTH)`
  - actual: `CAST((JULIANDAY(month) - JULIANDAY(b)) AS INTEGER)`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-clickhouse-0444-test-clickhouse`: `SELECT 1 XOR 0`
  - expected: `SELECT 1 XOR 0`
  - actual: `SELECT 1`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-clickhouse-0461-test-clickhouse`: `SELECT 1 XOR 0 XOR 1`
  - expected: `SELECT 1 XOR 0 XOR 1`
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

