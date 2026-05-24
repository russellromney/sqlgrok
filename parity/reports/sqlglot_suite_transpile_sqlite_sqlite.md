# SQLGlot Suite Bridge Report

Source: `parity/reports/sqlglot_suite_transpile_sqlite_sqlite.jsonl`

Total cases: `107`

## Status Counts

| Status | Count |
| --- | ---: |
| `match` | 56 |
| `mismatch` | 35 |
| `rust-error` | 11 |
| `unsupported-harness-shape` | 5 |

## Helper Buckets

| Status | Helper | Count |
| --- | --- | ---: |
| `match` | `validate_identity` | 44 |
| `mismatch` | `validate_identity` | 26 |
| `match` | `validate_all` | 12 |
| `rust-error` | `validate_identity` | 11 |
| `mismatch` | `validate_all` | 9 |
| `unsupported-harness-shape` | `validate_identity` | 4 |
| `unsupported-harness-shape` | `validate_all` | 1 |

## Source Buckets

| Status | Source | Test | Count |
| --- | --- | --- | ---: |
| `match` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 26 |
| `match` | `tests/dialects/test_sqlite.py` | `test_ddl` | 18 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 18 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_ddl` | 13 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 9 |
| `match` | `tests/dialects/test_sqlite.py` | `test_strftime` | 4 |
| `unsupported-harness-shape` | `tests/dialects/test_sqlite.py` | `test_create_trigger` | 3 |
| `match` | `tests/dialects/test_sqlite.py` | `test_datediff` | 3 |
| `match` | `tests/dialects/test_sqlite.py` | `test_analyze` | 2 |
| `rust-error` | `tests/dialects/test_sqlite.py` | `test_ddl` | 2 |
| `unsupported-harness-shape` | `tests/dialects/test_sqlite.py` | `test_ddl` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_hexadecimal_literal` | 1 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_longvarchar_dtype` | 1 |
| `unsupported-harness-shape` | `tests/dialects/test_sqlite.py` | `test_sqlite` | 1 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_strftime` | 1 |
| `match` | `tests/dialects/test_sqlite.py` | `test_trunc` | 1 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_trunc` | 1 |
| `mismatch` | `tests/dialects/test_sqlite.py` | `test_warnings` | 1 |
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

### `mismatch` `tests/dialects/test_sqlite.py:292`

- test: `test_ddl`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `CREATE TEMPORARY TABLE foo (id INTEGER)`
- expected: `CREATE TEMPORARY TABLE foo (id INTEGER)`
- actual: `CREATE TEMPORARY TABLE foo (id INT)`
- error: ``

### `mismatch` `tests/dialects/test_sqlite.py:300`

- test: `test_ddl`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `PRAGMA table_info`
- expected: `PRAGMA table_info`
- actual: `PRAGMA`
- error: ``

### `mismatch` `tests/dialects/test_sqlite.py:301`

- test: `test_ddl`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `PRAGMA schema`
- expected: `PRAGMA schema`
- actual: `PRAGMA`
- error: ``

### `rust-error` `tests/dialects/test_sqlite.py:302`

- test: `test_ddl`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `PRAGMA full_column_names = on`
- expected: `PRAGMA full_column_names = on`
- actual: ``
- error: `ValueError: Unexpected token: Token { token_type: On, value: "on", line: 1, col: 28, position: 27, quote_char: '\0' }`

### `mismatch` `tests/dialects/test_sqlite.py:303`

- test: `test_ddl`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `PRAGMA full_column_names = off`
- expected: `PRAGMA full_column_names = off`
- actual: `PRAGMA`
- error: ``

### `mismatch` `tests/dialects/test_sqlite.py:304`

- test: `test_ddl`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `PRAGMA cache_size = 2000`
- expected: `PRAGMA cache_size = 2000`
- actual: `PRAGMA`
- error: ``

### `mismatch` `tests/dialects/test_sqlite.py:305`

- test: `test_ddl`
- helper: `validate_identity`
- read/write: `sqlite` -> `sqlite`
- sql: `PRAGMA foo = -2000`
- expected: `PRAGMA foo = -2000`
- actual: `PRAGMA`
- error: ``

