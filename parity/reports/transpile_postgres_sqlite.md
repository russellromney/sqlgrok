# SQLGlot Import Report

Source: `parity/reports/transpile_postgres_sqlite.jsonl`

Total candidates: `682`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 291 |
| `mismatch` | 334 |
| `rust-error` | 57 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `mismatch` | `SELECT` | 108 |
| `match` | `SELECT` | 85 |
| `mismatch` | `CREATE` | 63 |
| `rust-error` | `SELECT` | 42 |
| `mismatch` | `CREATE TABLE` | 36 |
| `match` | `CREATE TABLE` | 20 |
| `match` | `REVOKE` | 19 |
| `match` | `CAST()` | 18 |
| `mismatch` | `X` | 18 |
| `match` | `GRANT` | 16 |
| `mismatch` | `BEGIN` | 11 |
| `match` | `INTERVAL` | 9 |
| `match` | `WITH` | 9 |
| `mismatch` | `CREATE INDEX` | 9 |
| `mismatch` | `ALTER TABLE` | 8 |
| `match` | `ROUND()` | 7 |
| `mismatch` | `TRUNCATE` | 7 |
| `match` | `INSERT` | 6 |
| `match` | `MERGE` | 5 |
| `match` | `UPDATE` | 5 |
| `mismatch` | `WITH` | 5 |
| `match` | `ANALYZE` | 4 |
| `match` | `DROP INDEX` | 4 |
| `match` | `REGEXP_INSTR()` | 4 |
| `match` | `STRING_AGG()` | 4 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 121 |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 107 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 88 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres` | 35 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres_create_trigger` | 34 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 31 |
| `match` | `tests/dialects/test_postgres.py` | `test_revoke` | 19 |
| `match` | `tests/dialects/test_postgres.py` | `test_grant` | 16 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_begin_transaction` | 11 |
| `match` | `tests/dialects/test_postgres.py` | `test_interval_span` | 10 |
| `match` | `tests/dialects/test_postgres.py` | `test_round` | 7 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 7 |
| `mismatch` | `tests/dialects/test_dune.py` | `test_dune` | 7 |
| `match` | `tests/dialects/test_doris.py` | `test_doris` | 6 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_xmlelement` | 6 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 5 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 5 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 5 |
| `match` | `tests/dialects/test_dialect.py` | `test_heredoc_strings` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_regexp_instr` | 4 |
| `match` | `tests/dialects/test_doris.py` | `test_table_alias_conversion` | 4 |
| `match` | `tests/dialects/test_mysql.py` | `test_mysql` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_analyze` | 4 |
| `match` | `tests/dialects/test_postgres.py` | `test_json_extract` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-bigquery-0709-test-bigquery`: `SELECT MAKE_TIME(15, 30, 00)`
  - expected: `SELECT TIME_FROM_PARTS(15, 30, 00)`
  - actual: `SELECT MAKE_TIME(15, 30, 00)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0131-test-clickhouse`: `TRUNC(3.14159, 2)`
  - expected: `TRUNC(3.14159)`
  - actual: `TRUNC(3.14159, 2)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0309-test-clickhouse`: `SELECT TO_DATE('05 12 2000', 'DD MM YYYY')`
  - expected: `SELECT STR_TO_DATE('05 12 2000', '%d %m %Y')`
  - actual: `SELECT TO_DATE('05 12 2000', 'DD MM YYYY')`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0348-test-clickhouse`: `x = any(array[1])`
  - expected: `x = ANY(ARRAY(1))`
  - actual: `x = ANY(ARRAY[1])`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0354-test-clickhouse`: `any(array[1]) <> x`
  - expected: `ANY(ARRAY(1)) <> x`
  - actual: `any(ARRAY[1]) <> x`

### `rust-error`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-oracle-0289-test-oracle`: `CAST(x AS sch.udt)`
  - expected: `CAST(x AS sch.udt)`
  - error: `Parser error: Expected RParen, got Dot ('.') at line 1 col 14`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1076-test-postgres`: `SELECT NUMRANGE(1.1, 2.2) -|- NUMRANGE(2.2, 3.3)`
  - expected: `SELECT NUMRANGE(1.1, 2.2) -|- NUMRANGE(2.2, 3.3)`
  - error: `Unexpected token: Token { token_type: BitwiseOr, value: "|", line: 1, col: 28, position: 27, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1099-test-postgres`: `SELECT MLEAST(VARIADIC ARRAY[10, -1, 5, 4.4])`
  - expected: `SELECT MLEAST(VARIADIC ARRAY(10, -1, 5, 4.4))`
  - error: `Parser error: Expected RParen, got Array ('ARRAY') at line 1 col 24`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1100-test-postgres`: `SELECT MLEAST(VARIADIC ARRAY[]::numeric[])`
  - expected: `SELECT MLEAST(VARIADIC CAST(ARRAY() AS ARRAY<REAL>))`
  - error: `Parser error: Expected RParen, got Array ('ARRAY') at line 1 col 24`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1104-test-postgres`: `SELECT * FROM schema_name.table_name st WHERE JSON_EXTRACT_PATH_TEXT((st.data)::json, variadic array['test'::text]) = 'test'::text`
  - expected: `SELECT * FROM schema_name.table_name AS st WHERE CAST((st.data) AS JSON) ->> VARIADIC ARRAY(CAST('test' AS TEXT)) = CAST('test' AS TEXT)`
  - error: `Parser error: Expected RParen, got Array ('array') at line 1 col 96`

