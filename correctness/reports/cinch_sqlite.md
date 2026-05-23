# SQLite Correctness Report

Source: `correctness/cases/cinch_sqlite.jsonl`

This report runs Python SQLGlot's SQLite-targeted output against stock `sqlite3`. A `sqlite-error` row is not a sqlgrok parity failure by itself; it is a cinch correctness or upstream SQLGlot candidate to investigate.

Total candidates: `8`

## Status Counts

| Status | Count |
| --- | ---: |
| `sqlite-error` | 8 |

## SQLGlot Parity Counts

| Parity | Count |
| --- | ---: |
| `match` | 8 |

## Tag Buckets

| Status | Tag | Count |
| --- | --- | ---: |
| `sqlite-error` | `cinch` | 8 |
| `sqlite-error` | `sqlite` | 8 |
| `sqlite-error` | `mysql` | 4 |
| `sqlite-error` | `postgres` | 4 |
| `sqlite-error` | `function` | 3 |
| `sqlite-error` | `limit` | 2 |
| `sqlite-error` | `insert` | 1 |
| `sqlite-error` | `join` | 1 |
| `sqlite-error` | `regex` | 1 |

## Candidates

### `cinch-mysql-insert-ignore-sqlite-exec` (sqlite-error)

- source: `INSERT IGNORE INTO t (id, a) VALUES (1, 2)`
- sqlglot sqlite: `INSERT IGNORE INTO t (id, a) VALUES (1, 2)`
- sqlgrok parity: `match`
- sqlite stderr: `Parse error near line 2: near "IGNORE": syntax error INSERT IGNORE INTO t (id, a) VALUES (1, 2); ^--- error here`
- note: Python SQLGlot preserves INSERT IGNORE for SQLite; stock SQLite expects INSERT OR IGNORE.

### `cinch-postgres-limit-all-sqlite-exec` (sqlite-error)

- source: `SELECT x FROM t LIMIT ALL`
- sqlglot sqlite: `SELECT x FROM t LIMIT ALL`
- sqlgrok parity: `match`
- sqlite stderr: `Parse error near line 2: near "ALL": syntax error SELECT x FROM t LIMIT ALL; ^--- error here`
- note: Python SQLGlot preserves LIMIT ALL for SQLite; stock SQLite rejects ALL in LIMIT.

### `cinch-postgres-offset-only-sqlite-exec` (sqlite-error)

- source: `SELECT x FROM t OFFSET 1`
- sqlglot sqlite: `SELECT x FROM t OFFSET 1`
- sqlgrok parity: `match`
- sqlite stderr: `Parse error near line 2: near "1": syntax error SELECT x FROM t OFFSET 1; ^--- error here`
- note: Python SQLGlot preserves OFFSET without LIMIT for SQLite; stock SQLite requires a LIMIT clause.

### `cinch-postgres-similar-to-sqlite-exec` (sqlite-error)

- source: `SELECT x SIMILAR TO 'a%' FROM t`
- sqlglot sqlite: `SELECT x SIMILAR TO 'a%' FROM t`
- sqlgrok parity: `match`
- sqlite stderr: `Parse error near line 2: near "TO": syntax error SELECT x SIMILAR TO 'a%' FROM t; ^--- error here`
- note: Python SQLGlot preserves SIMILAR TO for SQLite; stock SQLite does not implement the predicate.

### `cinch-postgres-lateral-sqlite-exec` (sqlite-error)

- source: `SELECT * FROM t, LATERAL (SELECT 1) AS x`
- sqlglot sqlite: `SELECT * FROM t, LATERAL (SELECT 1) AS x`
- sqlgrok parity: `match`
- sqlite stderr: `Parse error near line 2: near "SELECT": syntax error SELECT * FROM t, LATERAL (SELECT 1) AS x; error here ---^`
- note: Python SQLGlot preserves LATERAL for SQLite; stock SQLite rejects this form.

### `cinch-mysql-lpad-sqlite-exec` (sqlite-error)

- source: `SELECT LPAD('x', 3, '0')`
- sqlglot sqlite: `SELECT LPAD('x', 3, '0')`
- sqlgrok parity: `match`
- sqlite stderr: `Parse error near line 1: no such function: LPAD SELECT LPAD('x', 3, '0'); ^--- error here`
- note: Python SQLGlot preserves LPAD for SQLite; stock SQLite does not provide LPAD by default.

### `cinch-mysql-rpad-sqlite-exec` (sqlite-error)

- source: `SELECT RPAD('x', 3, '0')`
- sqlglot sqlite: `SELECT RPAD('x', 3, '0')`
- sqlgrok parity: `match`
- sqlite stderr: `Parse error near line 1: no such function: RPAD SELECT RPAD('x', 3, '0'); ^--- error here`
- note: Python SQLGlot preserves RPAD for SQLite; stock SQLite does not provide RPAD by default.

### `cinch-mysql-substring-index-sqlite-exec` (sqlite-error)

- source: `SELECT SUBSTRING_INDEX('a,b,c', ',', 2)`
- sqlglot sqlite: `SELECT SUBSTRING_INDEX('a,b,c', ',', 2)`
- sqlgrok parity: `match`
- sqlite stderr: `Parse error near line 1: no such function: SUBSTRING_INDEX SELECT SUBSTRING_INDEX('a,b,c', ',', 2); ^--- error here`
- note: Python SQLGlot preserves SUBSTRING_INDEX for SQLite; stock SQLite does not provide SUBSTRING_INDEX by default.

