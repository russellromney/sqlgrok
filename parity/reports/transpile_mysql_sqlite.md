# SQLGlot Import Report

Source: `parity/reports/transpile_mysql_sqlite.jsonl`

Total candidates: `494`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 271 |
| `mismatch` | 216 |
| `oracle-error` | 7 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 98 |
| `mismatch` | `SELECT` | 70 |
| `mismatch` | `CREATE TABLE` | 45 |
| `match` | `SET` | 33 |
| `mismatch` | `ALTER TABLE` | 26 |
| `mismatch` | `DATE_ADD()` | 12 |
| `match` | `CREATE TABLE` | 11 |
| `match` | `CAST()` | 9 |
| `match` | `ALTER TABLE` | 8 |
| `match` | `ANALYZE` | 8 |
| `match` | `GROUP_CONCAT()` | 8 |
| `match` | `GRANT` | 7 |
| `match` | `REVOKE` | 7 |
| `mismatch` | `CAST()` | 7 |
| `oracle-error` | `CREATE TABLE` | 7 |
| `match` | `INSERT` | 6 |
| `mismatch` | `MATCH()` | 6 |
| `match` | `ALTER` | 5 |
| `match` | `CHAR()` | 4 |
| `match` | `CREATE` | 4 |
| `mismatch` | `DELETE` | 4 |
| `match` | `REGEXP_INSTR()` | 3 |
| `match` | `STR_TO_DATE()` | 3 |
| `match` | `X` | 3 |
| `mismatch` | `CREATE` | 3 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_ddl` | 80 |
| `match` | `tests/dialects/test_mysql.py` | `test_identity` | 64 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_identity` | 31 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 28 |
| `match` | `tests/dialects/test_mysql.py` | `test_ddl` | 26 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql` | 12 |
| `match` | `tests/dialects/test_mysql.py` | `test_canonical_functions` | 11 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_valid_interval_units` | 11 |
| `match` | `tests/dialects/test_mysql.py` | `test_date_format` | 10 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql_time` | 10 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_mysql_time` | 9 |
| `match` | `tests/dialects/test_mysql.py` | `test_analyze` | 8 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_convert` | 8 |
| `match` | `tests/dialects/test_mysql.py` | `test_grant` | 7 |
| `match` | `tests/dialects/test_mysql.py` | `test_revoke` | 7 |
| `oracle-error` | `tests/dialects/test_mysql.py` | `test_ddl` | 7 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 6 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_match_against` | 6 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_types` | 6 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 5 |
| `match` | `tests/dialects/test_mysql.py` | `test_escape` | 5 |
| `mismatch` | `tests/dialects/test_mysql.py` | `test_json_value` | 5 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bit_aggs` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-mysql-to-sqlite-tests-dialects-test-dialect-2881-test-alias`: `SELECT 1 'foo'`
  - expected: `SELECT 1 AS "foo"`
  - actual: `SELECT 1`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-dialect-2951-test-nullsafe-eq`: `SELECT a <=> b`
  - expected: `SELECT a IS NOT DISTINCT FROM b`
  - actual: `SELECT a <=> b`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-dialect-2976-test-hash-comments`: `SELECT 1 # arbitrary content,,, until end-of-line`
  - expected: `SELECT 1 /* arbitrary content,,, until end-of-line */`
  - actual: `SELECT 1`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-dialect-2984-test-hash-comments`: `SELECT # comment1 x, # comment2 y # comment3`
  - expected: `/* comment1 */ SELECT x /* comment2 */, y /* comment3 */`
  - actual: `SELECT x, y`
- `sqlglot-mysql-to-sqlite-tests-dialects-test-duckdb-1827-test-time`: `SELECT DATE '2020-01-01' + INTERVAL -1 DAY`
  - expected: `SELECT DATE('2020-01-01') + INTERVAL '-1' DAY`
  - actual: `SELECT DATE('2020-01-01') + INTERVAL -1 DAY`

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

