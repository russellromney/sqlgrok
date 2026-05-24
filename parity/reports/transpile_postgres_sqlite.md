# SQLGlot Import Report

Source: `parity/reports/transpile_postgres_sqlite.jsonl`

Total candidates: `686`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 404 |
| `mismatch` | 254 |
| `rust-error` | 28 |

## Top Feature Buckets

| Status | Feature | Count |
| --- | --- | ---: |
| `match` | `SELECT` | 157 |
| `mismatch` | `SELECT` | 64 |
| `mismatch` | `CREATE` | 62 |
| `mismatch` | `CREATE TABLE` | 36 |
| `match` | `CREATE TABLE` | 20 |
| `match` | `REVOKE` | 19 |
| `match` | `CAST()` | 18 |
| `rust-error` | `SELECT` | 18 |
| `match` | `X` | 17 |
| `match` | `GRANT` | 16 |
| `mismatch` | `BEGIN` | 11 |
| `match` | `INSERT` | 9 |
| `match` | `INTERVAL` | 9 |
| `match` | `WITH` | 9 |
| `mismatch` | `CREATE INDEX` | 9 |
| `mismatch` | `ALTER TABLE` | 8 |
| `match` | `ROUND()` | 7 |
| `mismatch` | `TRUNCATE` | 7 |
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
| `match` | `tests/dialects/test_postgres.py` | `test_postgres` | 172 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_ddl` | 87 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres` | 75 |
| `match` | `tests/dialects/test_postgres.py` | `test_ddl` | 35 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_postgres_create_trigger` | 34 |
| `rust-error` | `tests/dialects/test_postgres.py` | `test_postgres` | 20 |
| `match` | `tests/dialects/test_postgres.py` | `test_revoke` | 19 |
| `match` | `tests/dialects/test_postgres.py` | `test_grant` | 16 |
| `mismatch` | `tests/dialects/test_postgres.py` | `test_begin_transaction` | 11 |
| `match` | `tests/dialects/test_postgres.py` | `test_interval_span` | 10 |
| `match` | `tests/dialects/test_postgres.py` | `test_round` | 7 |
| `mismatch` | `tests/dialects/test_dune.py` | `test_dune` | 7 |
| `match` | `tests/dialects/test_bigquery.py` | `test_bigquery` | 6 |
| `match` | `tests/dialects/test_doris.py` | `test_doris` | 6 |
| `match` | `tests/dialects/test_postgres.py` | `test_xmlelement` | 6 |
| `match` | `tests/dialects/test_clickhouse.py` | `test_clickhouse` | 5 |
| `match` | `tests/dialects/test_presto.py` | `test_presto` | 5 |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 5 |
| `match` | `tests/dialects/test_dialect.py` | `test_heredoc_strings` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_localtime_and_localtimestamp` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_logarithm` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_operators` | 4 |
| `match` | `tests/dialects/test_dialect.py` | `test_regexp_instr` | 4 |
| `match` | `tests/dialects/test_doris.py` | `test_table_alias_conversion` | 4 |
| `match` | `tests/dialects/test_duckdb.py` | `test_duckdb` | 4 |

## Non-Matching Examples

### `mismatch`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0131-test-clickhouse`: `TRUNC(3.14159, 2)`
  - expected: `TRUNC(3.14159)`
  - actual: `TRUNC(3.14159, 2)`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0348-test-clickhouse`: `x = any(array[1])`
  - expected: `x = ANY(ARRAY(1))`
  - actual: `x = ANY(ARRAY[1])`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0354-test-clickhouse`: `any(array[1]) <> x`
  - expected: `ANY(ARRAY(1)) <> x`
  - actual: `any(ARRAY(1)) <> x`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-clickhouse-0372-test-clickhouse`: `SELECT TIMESTAMP '2020-01-01' + INTERVAL '500 us'`
  - expected: `SELECT CAST('2020-01-01' AS TIMESTAMP) + INTERVAL '500' MICROSECOND`
  - actual: `SELECT CAST('2020-01-01' AS TIMESTAMP) + INTERVAL '500 us'`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-dialect-1490-test-array`: `ARRAY_PREPEND(x, arr)`
  - expected: `ARRAY_PREPEND(arr, x)`
  - actual: `ARRAY_PREPEND(x, arr)`

### `rust-error`

- `sqlglot-postgres-to-sqlite-tests-dialects-test-oracle-0296-test-oracle`: `CAST(x AS sch.udt)`
  - expected: `CAST(x AS sch.udt)`
  - error: `Parser error: Expected RParen, got Dot ('.') at line 1 col 14`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1096-test-postgres`: `SELECT NUMRANGE(1.1, 2.2) -|- NUMRANGE(2.2, 3.3)`
  - expected: `SELECT NUMRANGE(1.1, 2.2) -|- NUMRANGE(2.2, 3.3)`
  - error: `Unexpected token: Token { token_type: BitwiseOr, value: "|", line: 1, col: 28, position: 27, quote_char: '\0' }`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1709-test-recursive-cte`: `WITH RECURSIVE search_tree(id, link, data) AS (SELECT t.id, t.link, t.data FROM tree AS t UNION ALL SELECT t.id, t.link, t.data FROM tree AS t, search_tree AS st WHERE t.id = st.link) SEARCH BREADTH FIRST BY id SET ordercol SELECT * FROM search_tree ORDER BY ordercol`
  - expected: `WITH RECURSIVE search_tree(id, link, data) AS (SELECT t.id, t.link, t.data FROM tree AS t UNION ALL SELECT t.id, t.link, t.data FROM tree AS t, search_tree AS st WHERE t.id = st.link) SEARCH BREADTH FIRST BY id SET ordercol SELECT * FROM search_tree ORDER BY ordercol NULLS LAST`
  - error: `Parser error: Expected SELECT or INSERT after WITH clause`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1709-test-recursive-cte-2`: `WITH RECURSIVE search_tree(id, link, data) AS (SELECT t.id, t.link, t.data FROM tree AS t UNION ALL SELECT t.id, t.link, t.data FROM tree AS t, search_tree AS st WHERE t.id = st.link) SEARCH DEPTH FIRST BY id SET ordercol SELECT * FROM search_tree ORDER BY ordercol`
  - expected: `WITH RECURSIVE search_tree(id, link, data) AS (SELECT t.id, t.link, t.data FROM tree AS t UNION ALL SELECT t.id, t.link, t.data FROM tree AS t, search_tree AS st WHERE t.id = st.link) SEARCH DEPTH FIRST BY id SET ordercol SELECT * FROM search_tree ORDER BY ordercol NULLS LAST`
  - error: `Parser error: Expected SELECT or INSERT after WITH clause`
- `sqlglot-postgres-to-sqlite-tests-dialects-test-postgres-1713-test-recursive-cte`: `WITH RECURSIVE search_graph(id, link, data, depth) AS (SELECT g.id, g.link, g.data, 1 FROM graph AS g UNION ALL SELECT g.id, g.link, g.data, sg.depth + 1 FROM graph AS g, search_graph AS sg WHERE g.id = sg.link) CYCLE id SET is_cycle USING path SELECT * FROM search_graph`
  - expected: `WITH RECURSIVE search_graph(id, link, data, depth) AS (SELECT g.id, g.link, g.data, 1 FROM graph AS g UNION ALL SELECT g.id, g.link, g.data, sg.depth + 1 FROM graph AS g, search_graph AS sg WHERE g.id = sg.link) CYCLE id SET is_cycle USING path SELECT * FROM search_graph`
  - error: `Parser error: Expected SELECT or INSERT after WITH clause`

