# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_transpile_mysql_sqlite.jsonl`

Mode: `helper-route`
Requested pair: `mysql` -> `sqlite`

Total cases: `23`
Observed helper attempts: `15164`
Filtered by read/write: `15141`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 23 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_all` | 23 |

## Filtered Routes

| Helper | Read | Write | Count |
| --- | --- | --- | ---: |
| `validate_identity` | `snowflake` | `snowflake` | 1006 |
| `validate_all` | `snowflake` | `snowflake` | 557 |
| `validate_all` | `snowflake` | `duckdb` | 509 |
| `validate_identity` | `postgres` | `postgres` | 461 |
| `validate_identity` | `tsql` | `tsql` | 438 |
| `validate_identity` | `mysql` | `mysql` | 424 |
| `validate_identity` | `duckdb` | `duckdb` | 399 |
| `validate_identity` | `bigquery` | `bigquery` | 392 |
| `validate_identity` | `clickhouse` | `clickhouse` | 383 |
| `validate_all` | `bigquery` | `bigquery` | 273 |
| `validate_all` | `bigquery` | `duckdb` | 211 |
| `validate_identity` | `oracle` | `oracle` | 202 |
| `validate_all` | `tsql` | `tsql` | 183 |
| `validate_all` | `duckdb` | `duckdb` | 174 |
| `validate_all` | `None` | `duckdb` | 150 |
| `validate_all` | `None` | `presto` | 145 |
| `validate_all` | `None` | `spark` | 139 |
| `validate_all` | `tsql` | `spark` | 136 |
| `validate` | `None` | `None` | 135 |
| `validate_identity` | `None` | `None` | 135 |
| `validate_all` | `presto` | `presto` | 133 |
| `validate_identity` | `databricks` | `databricks` | 132 |
| `validate_identity` | `redshift` | `redshift` | 130 |
| `validate_all` | `None` | `bigquery` | 121 |
| `validate_all` | `spark` | `spark` | 119 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 10 |
| `match` | `tests/dialects/test_mysql.py` | `test_hexadecimal_literal` | 5 |
| `match` | `tests/dialects/test_mysql.py` | `test_bits_literal` | 2 |
| `match` | `tests/dialects/test_mysql.py` | `test_canonical_functions` | 2 |
| `match` | `tests/dialects/test_mysql.py` | `test_ddl` | 2 |
| `match` | `tests/dialects/test_mysql.py` | `test_safe_div` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_ddl` | 1 |
