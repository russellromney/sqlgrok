# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_transpile_sqlite_sqlite.jsonl`

Total cases: `107`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 102 |
| `unsupported-harness-shape` | 5 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_identity` | 81 |
| `match` | `validate_all` | 21 |
| `unsupported-harness-shape` | `validate_identity` | 4 |
| `unsupported-harness-shape` | `validate_all` | 1 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 53 |
| `match` | `tests/dialects/test_sqlite.py` | `test_ddl` | 33 |
| `match` | `tests/dialects/test_sqlite.py` | `test_strftime` | 5 |
| `unsupported-harness-shape` | `tests/dialects/test_sqlite.py` | `test_create_trigger` | 3 |
| `match` | `tests/dialects/test_sqlite.py` | `test_datediff` | 3 |
| `match` | `tests/dialects/test_sqlite.py` | `test_analyze` | 2 |
| `match` | `tests/dialects/test_sqlite.py` | `test_trunc` | 2 |
| `unsupported-harness-shape` | `tests/dialects/test_sqlite.py` | `test_ddl` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_hexadecimal_literal` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_longvarchar_dtype` | 1 |
| `unsupported-harness-shape` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_warnings` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_window_null_treatment` | 1 |

## Examples

### `unsupported-harness-shape` `tests/dialects/test_sqlite.py:367`

- test: `test_create_trigger`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `CREATE TRIGGER log_insert AFTER INSERT ON users BEGIN INSERT INTO audit_log (user_id, action, created_at) VALUES (NEW.id, 'INSERT', datetime('now')) END`
- expected: `CREATE TRIGGER log_insert AFTER INSERT ON users BEGIN INSERT INTO audit_log (user_id, action, created_at) VALUES (NEW.id, 'INSERT', datetime('now')) END`
- actual: ``
- error: `pretty/identify/check_command_warning helper options are not supported yet`

### `unsupported-harness-shape` `tests/dialects/test_sqlite.py:372`

- test: `test_create_trigger`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `CREATE TRIGGER check_balance BEFORE UPDATE OF balance ON accounts WHEN NEW.balance < 0 BEGIN UPDATE accounts SET balance = 0 WHERE id = NEW.id END`
- expected: `CREATE TRIGGER check_balance BEFORE UPDATE OF balance ON accounts WHEN NEW.balance < 0 BEGIN UPDATE accounts SET balance = 0 WHERE id = NEW.id END`
- actual: ``
- error: `pretty/identify/check_command_warning helper options are not supported yet`

### `unsupported-harness-shape` `tests/dialects/test_sqlite.py:377`

- test: `test_create_trigger`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `CREATE TRIGGER view_insert INSTEAD OF INSERT ON employee_view BEGIN INSERT INTO employees (id, name, department) VALUES (NEW.id, NEW.name, NEW.department) END`
- expected: `CREATE TRIGGER view_insert INSTEAD OF INSERT ON employee_view BEGIN INSERT INTO employees (id, name, department) VALUES (NEW.id, NEW.name, NEW.department) END`
- actual: ``
- error: `pretty/identify/check_command_warning helper options are not supported yet`

### `unsupported-harness-shape` `tests/dialects/test_sqlite.py:313`

- test: `test_ddl`
- helper: `validate_all`
- read/write: `sqlite` -> `sqlite`
- sql: `\n            CREATE TABLE "Track"\n            (\n                CONSTRAINT "PK_Track" FOREIGN KEY ("TrackId"),\n                FOREIGN KEY ("AlbumId") REFERENCES "Album" (\n                    "AlbumId"\n                ) ON DELETE NO ACTION ON UPDATE NO ACTION,\n                FOREIGN KEY ("AlbumId") ON DELETE CASCADE ON UPDATE RESTRICT,\n                FOREIGN KEY ("AlbumId") ON DELETE SET NULL ON UPDATE SET DEFAULT\n            )\n            `
- expected: `CREATE TABLE "Track" (\n  CONSTRAINT "PK_Track" FOREIGN KEY ("TrackId"),\n  FOREIGN KEY ("AlbumId") REFERENCES "Album" (\n    "AlbumId"\n  ) ON DELETE NO ACTION ON UPDATE NO ACTION,\n  FOREIGN KEY ("AlbumId") ON DELETE CASCADE ON UPDATE RESTRICT,\n  FOREIGN KEY ("AlbumId") ON DELETE SET NULL ON UPDATE SET DEFAULT\n)`
- actual: ``
- error: `pretty/identify helper options are not supported yet`

### `unsupported-harness-shape` `tests/dialects/test_sqlite.py:179`

- test: `test_sqlite`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `REPLACE INTO foo (x, y) VALUES (1, 2)`
- expected: `REPLACE INTO foo (x, y) VALUES (1, 2)`
- actual: ``
- error: `pretty/identify/check_command_warning helper options are not supported yet`

