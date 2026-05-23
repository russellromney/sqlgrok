# SQLGlot Import Report

Source: `parity/reports/transpile_postgres_sqlite.jsonl`

Total candidates: `686`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 360 |
| `mismatch` | 269 |
| `rust-error` | 57 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 121 |
| `mismatch` | `SELECT` | 76 |
| `mismatch` | `CREATE` | 62 |
| `rust-error` | `SELECT` | 42 |
| `mismatch` | `CREATE TABLE` | 36 |
| `match` | `CREATE TABLE` | 20 |
| `match` | `REVOKE` | 19 |
| `match` | `CAST()` | 18 |
| `match` | `X` | 17 |
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
| `mismatch` | `X` | 5 |
| `match` | `A` | 4 |
| `match` | `ANALYZE` | 4 |
| `match` | `CREATE` | 4 |

## Top Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 148 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 87 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 84 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres` | 35 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres_create_trigger` | 34 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 32 |
| `match` | `tests/dialects/test_postgres.py` | `test_revoke` | 19 |
| `match` | `tests/dialects/test_postgres.py` | `test_grant` | 16 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_begin_transaction` | 11 |
| `match` | `tests/dialects/test_postgres.py` | `test_interval_span` | 10 |
| `match` | `tests/dialects/test_postgres.py` | `test_round` | 7 |
| `mismatch` | `tests/dialects/test_dune.py` | `test_dune` | 7 |
| `match` | `tests/dialects/test_doris.py` | `test_doris` | 6 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_xmlelement` | 6 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 5 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 5 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 5 |
| `mismatch` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 5 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_heredoc_strings` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_regexp_instr` | 4 |
| `match` | `tests/dialects/test_doris.py` | `test_table_alias_conversion` | 4 |

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
  - actual: `any(ARRAY(1)) <> x`

### `rust-error`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-oracle-0296-test-oracle`: `CAST(x AS sch.udt)`
  - expected: `CAST(x AS sch.udt)`
  - error: `Parser error: Expected RParen, got Dot ('.') at line 1 col 14`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1096-test-postgres`: `SELECT NUMRANGE(1.1, 2.2) -|- NUMRANGE(2.2, 3.3)`
  - expected: `SELECT NUMRANGE(1.1, 2.2) -|- NUMRANGE(2.2, 3.3)`
  - error: `Unexpected token: Token { token_type: BitwiseOr, value: "|", line: 1, col: 28, position: 27, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1119-test-postgres`: `SELECT MLEAST(VARIADIC ARRAY[10, -1, 5, 4.4])`
  - expected: `SELECT MLEAST(VARIADIC ARRAY(10, -1, 5, 4.4))`
  - error: `Parser error: Expected RParen, got Array ('ARRAY') at line 1 col 24`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-0111-test-postgres`: `SELECT id, name FROM xml_data AS t, XMLTABLE('/root/user' PASSING t.xml COLUMNS id INT PATH '@id', name TEXT PATH 'name/text()') AS x`
  - expected: `SELECT id, name FROM xml_data AS t, XMLTABLE('/root/user' PASSING t.xml COLUMNS id INTEGER PATH '@id', name TEXT PATH 'name/text()') AS x`
  - error: `Parser error: Expected RParen, got Identifier ('PASSING') at line 1 col 59`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1120-test-postgres`: `SELECT MLEAST(VARIADIC ARRAY[]::numeric[])`
  - expected: `SELECT MLEAST(VARIADIC CAST(ARRAY() AS ARRAY<REAL>))`
  - error: `Parser error: Expected RParen, got Array ('ARRAY') at line 1 col 24`

