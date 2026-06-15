/// Tests ported from Python sqlglot's dialect-specific test files.
///
/// Follows Python's testing patterns:
///   - `validate_identity(sql)` → parse→generate roundtrip (here: `assert_identity`)
///   - `validate(sql, target)` → parse→generate normalization (here: `assert_transpile`)
///   - `validate_all(sql, read={..}, write={..})` → cross-dialect (here: `assert_validate_all`)
///
/// Coverage mirrors the main cross-dialect test categories from:
///   - tests/dialects/test_dialect.py (cast, operators, random, transactions, etc.)
///   - tests/dialects/test_{bigquery,mysql,postgres,duckdb,snowflake,tsql,...}.py
use sqlgrok::{Dialect, transpile};

// ═════════════════════════════════════════════════════════════════════════════
// Helpers
// ═════════════════════════════════════════════════════════════════════════════

fn transpile_ok(sql: &str, read: Dialect, write: Dialect) -> String {
    transpile(sql, read, write).unwrap_or_else(|e| panic!("Transpile failed for '{}': {}", sql, e))
}

fn assert_transpile(sql: &str, expected: &str, read: Dialect, write: Dialect) {
    let result = transpile_ok(sql, read, write);
    assert_eq!(
        result, expected,
        "\n  SQL:    {}\n  {:?} → {:?}",
        sql, read, write
    );
}

/// Verify that SQL roundtrips through a specific dialect pair.
fn assert_identity(sql: &str, dialect: Dialect) {
    let result = transpile_ok(sql, dialect, dialect);
    assert_eq!(result, sql, "\n  Identity failed for {:?}", dialect);
}

/// Mirrors Python's `validate_all(sql, write={dialect: expected, ...})`.
///
/// Parses `sql` with `read_dialect` and verifies that generating for each
/// `(write_dialect, expected_sql)` pair produces the expected output.
fn assert_validate_all(sql: &str, read_dialect: Dialect, writes: &[(Dialect, &str)]) {
    for (write_d, expected) in writes {
        let result = transpile_ok(sql, read_dialect, *write_d);
        assert_eq!(
            result, *expected,
            "\n  validate_all:\n    Input: {}\n    Read:  {:?}\n    Write: {:?}",
            sql, read_dialect, write_d
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Dialect identity – all 30 dialects should roundtrip basic SQL
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_dialect_identities() {
    let queries = [
        "SELECT 1",
        "SELECT * FROM t",
        "SELECT a, b FROM t WHERE a > 1",
        "SELECT * FROM t ORDER BY a LIMIT 10",
        "SELECT a, COUNT(*) FROM t GROUP BY a",
        "SELECT a FROM t WHERE a IS TRUE",
    ];
    for dialect in Dialect::all() {
        for sql in &queries {
            assert_identity(sql, *dialect);
        }
    }
}

#[test]
fn test_dialect_count() {
    // Ensure we have all 30 Python sqlglot dialects
    assert_eq!(Dialect::all().len(), 30);
}

#[test]
fn test_dialect_from_str() {
    assert_eq!(Dialect::from_str("postgres"), Some(Dialect::Postgres));
    assert_eq!(Dialect::from_str("postgresql"), Some(Dialect::Postgres));
    assert_eq!(Dialect::from_str("BIGQUERY"), Some(Dialect::BigQuery));
    assert_eq!(Dialect::from_str("tsql"), Some(Dialect::Tsql));
    assert_eq!(Dialect::from_str("mssql"), Some(Dialect::Tsql));
    assert_eq!(Dialect::from_str("sqlserver"), Some(Dialect::Tsql));
    assert_eq!(Dialect::from_str("clickhouse"), Some(Dialect::ClickHouse));
    assert_eq!(Dialect::from_str("hive"), Some(Dialect::Hive));
    assert_eq!(Dialect::from_str("spark"), Some(Dialect::Spark));
    assert_eq!(Dialect::from_str("unknown"), None);
}

#[test]
fn test_dialect_support_levels() {
    assert_eq!(Dialect::Postgres.support_level(), "Official");
    assert_eq!(Dialect::BigQuery.support_level(), "Official");
    assert_eq!(Dialect::Tsql.support_level(), "Official");
    assert_eq!(Dialect::Doris.support_level(), "Community");
    assert_eq!(Dialect::Teradata.support_level(), "Community");
    assert_eq!(Dialect::Prql.support_level(), "Community");
}

#[test]
fn test_dialect_display() {
    assert_eq!(format!("{}", Dialect::Ansi), "ANSI SQL");
    assert_eq!(format!("{}", Dialect::Postgres), "PostgreSQL");
    assert_eq!(format!("{}", Dialect::Tsql), "T-SQL");
    assert_eq!(format!("{}", Dialect::ClickHouse), "ClickHouse");
    assert_eq!(format!("{}", Dialect::DuckDb), "DuckDB");
}

// ═════════════════════════════════════════════════════════════════════════════
// Function mapping: SUBSTR ↔ SUBSTRING
// (from Python test_mysql.py, test_postgres.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_substr_postgres_to_mysql() {
    // MySQL renders Substring as SUBSTRING (SQLGlot)
    assert_transpile(
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        Dialect::Postgres,
        Dialect::Mysql,
    );
}

#[test]
fn test_substr_mysql_to_postgres() {
    // Postgres renders Substring in SQL-standard FROM/FOR form (SQLGlot)
    assert_transpile(
        "SELECT SUBSTR(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x FROM 1 FOR 3) FROM t",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_substring_to_sqlite() {
    // Python SQLGlot normalizes SUBSTR/SUBSTRING to SUBSTRING for SQLite output.
    assert_transpile(
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        Dialect::Ansi,
        Dialect::Sqlite,
    );
}

#[test]
fn test_substr_from_duckdb() {
    // MySQL renders Substring as SUBSTRING (SQLGlot)
    assert_transpile(
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        Dialect::DuckDb,
        Dialect::Mysql,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Function mapping: NOW → CURRENT_TIMESTAMP
// (from Python test_bigquery.py, test_snowflake.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_now_to_bigquery() {
    assert_transpile(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

#[test]
fn test_now_to_snowflake() {
    assert_transpile(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::Snowflake,
    );
}

#[test]
fn test_now_to_ansi() {
    assert_transpile(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::Ansi,
    );
}

#[test]
fn test_now_to_hive() {
    assert_transpile(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::Hive,
    );
}

#[test]
fn test_now_to_spark() {
    assert_transpile(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::Spark,
    );
}

#[test]
fn test_now_to_presto() {
    // The presto family renders CurrentTimestamp bare (SQLGlot)
    assert_transpile(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP",
        Dialect::Postgres,
        Dialect::Presto,
    );
}

#[test]
fn test_now_to_trino() {
    assert_transpile(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP",
        Dialect::Postgres,
        Dialect::Trino,
    );
}

#[test]
fn test_now_to_tsql() {
    // T-SQL uses GETDATE() instead of NOW()
    assert_transpile(
        "SELECT NOW()",
        "SELECT GETDATE()",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_now_to_clickhouse() {
    assert_transpile(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::ClickHouse,
    );
}

#[test]
fn test_getdate_to_postgres() {
    // Postgres renders CurrentTimestamp bare (SQLGlot)
    assert_transpile(
        "SELECT GETDATE()",
        "SELECT CURRENT_TIMESTAMP",
        Dialect::Tsql,
        Dialect::Postgres,
    );
}

#[test]
fn test_getdate_to_bigquery() {
    assert_transpile(
        "SELECT GETDATE()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Tsql,
        Dialect::BigQuery,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Function mapping: LEN → LENGTH
// (from Python test_bigquery.py → test_postgres.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_len_to_postgres() {
    assert_transpile(
        "SELECT LEN(name) FROM t",
        "SELECT LENGTH(name) FROM t",
        Dialect::BigQuery,
        Dialect::Postgres,
    );
}

#[test]
fn test_len_to_mysql() {
    // MySQL reserves LENGTH for byte counting; character length renders
    // CHAR_LENGTH (SQLGlot)
    assert_transpile(
        "SELECT LEN(x) FROM t",
        "SELECT CHAR_LENGTH(x) FROM t",
        Dialect::BigQuery,
        Dialect::Mysql,
    );
}

#[test]
fn test_len_to_sqlite() {
    assert_transpile(
        "SELECT LEN(x) FROM t",
        "SELECT LENGTH(x) FROM t",
        Dialect::BigQuery,
        Dialect::Sqlite,
    );
}

#[test]
fn test_length_binary_flag() {
    // MySQL's own LENGTH counts bytes (SQLGlot Length(binary=True)): it must
    // KEEP the LENGTH spelling for a mysql target, while a character-counting
    // source renders CHAR_LENGTH there.
    assert_transpile(
        "SELECT LENGTH(x) FROM t",
        "SELECT LENGTH(x) FROM t",
        Dialect::Mysql,
        Dialect::Mysql,
    );
    assert_transpile(
        "SELECT LENGTH(x) FROM t",
        "SELECT CHAR_LENGTH(x) FROM t",
        Dialect::Postgres,
        Dialect::Mysql,
    );
    // Both spellings leave mysql as the canonical LENGTH for other targets.
    assert_transpile(
        "SELECT LENGTH(x), CHAR_LENGTH(x) FROM t",
        "SELECT LENGTH(x), LENGTH(x) FROM t",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_bare_current_timestamp_is_node() {
    // Bare CURRENT_TIMESTAMP parses to the CurrentTimestamp node (SQLGlot),
    // so it renders with parens for mysql and as GETDATE() for tsql.
    assert_transpile(
        "SELECT CURRENT_TIMESTAMP",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::Mysql,
    );
    assert_transpile(
        "SELECT CURRENT_TIMESTAMP",
        "SELECT GETDATE()",
        Dialect::Postgres,
        Dialect::Tsql,
    );
    // An empty-paren call canonicalizes to the same node.
    assert_transpile(
        "SELECT CURRENT_TIMESTAMP()",
        "SELECT CURRENT_TIMESTAMP",
        Dialect::Mysql,
        Dialect::Postgres,
    );
    // ClickHouse treats bare CURRENT_TIMESTAMP as an identifier (SQLGlot).
    assert_transpile(
        "SELECT CURRENT_TIMESTAMP",
        "SELECT CURRENT_TIMESTAMP",
        Dialect::ClickHouse,
        Dialect::Postgres,
    );
}

#[test]
fn test_now_not_canonicalized_for_mysql_source() {
    // Only postgres/presto families, databricks, and exasol parse NOW() into
    // CurrentTimestamp (SQLGlot); a mysql-source NOW() stays NOW() for every
    // target.
    assert_transpile(
        "SELECT NOW()",
        "SELECT NOW()",
        Dialect::Mysql,
        Dialect::Postgres,
    );
    assert_transpile(
        "SELECT NOW()",
        "SELECT NOW()",
        Dialect::Mysql,
        Dialect::Tsql,
    );
}

#[test]
fn test_len_to_duckdb() {
    assert_transpile(
        "SELECT LEN(x) FROM t",
        "SELECT LENGTH(x) FROM t",
        Dialect::BigQuery,
        Dialect::DuckDb,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Type mapping: read-side canonicalization + target-keyed rendering
// (verified against the Python SQLGlot oracle)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_mysql_timestamp_is_tz_aware() {
    // MySQL TIMESTAMP is timezone-aware (SQLGlot tokenizer maps it to
    // TIMESTAMPTZ); DATETIME is its naive type.
    assert_transpile(
        "CREATE TABLE t (a TIMESTAMP, b TIMESTAMP(3), e DATETIME)",
        "CREATE TABLE t (a TIMESTAMPTZ, b TIMESTAMPTZ(3), e TIMESTAMP)",
        Dialect::Mysql,
        Dialect::Postgres,
    );
    // Identity round-trips through the canonical type.
    assert_transpile(
        "CREATE TABLE t (a TIMESTAMP, e DATETIME)",
        "CREATE TABLE t (a TIMESTAMP, e DATETIME)",
        Dialect::Mysql,
        Dialect::Mysql,
    );
    // Postgres TIMESTAMP stays naive and becomes mysql DATETIME.
    assert_transpile(
        "CREATE TABLE t (a TIMESTAMP, b TIMESTAMPTZ, e DATETIME)",
        "CREATE TABLE t (a DATETIME, b TIMESTAMP, e DATETIME)",
        Dialect::Postgres,
        Dialect::Mysql,
    );
    // Postgres renders canonical TIMESTAMPTZ (not TIMESTAMP WITH TIME ZONE),
    // and canonical DATETIME as TIMESTAMP.
    assert_transpile(
        "CREATE TABLE t (b TIMESTAMP WITH TIME ZONE, e DATETIME)",
        "CREATE TABLE t (b TIMESTAMPTZ, e TIMESTAMP)",
        Dialect::Postgres,
        Dialect::Postgres,
    );
    assert_transpile(
        "CREATE TABLE t (a TIMESTAMP, e DATETIME)",
        "CREATE TABLE t (a DATETIME2, e DATETIME)",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_tsql_timestamp_is_rowversion() {
    // T-SQL TIMESTAMP is the legacy rowversion type (SQLGlot).
    assert_transpile(
        "CREATE TABLE t (a TIMESTAMP)",
        "CREATE TABLE t (a ROWVERSION)",
        Dialect::Tsql,
        Dialect::Tsql,
    );
}

#[test]
fn test_mysql_signed_unsigned_casts() {
    // MySQL SIGNED/UNSIGNED are canonical BIGINT/UBIGINT (SQLGlot tokenizer
    // keywords); mysql's own CAST renders them back as SIGNED/UNSIGNED.
    assert_transpile(
        "SELECT CAST(x AS SIGNED), CAST(y AS UNSIGNED)",
        "SELECT CAST(x AS SIGNED), CAST(y AS UNSIGNED)",
        Dialect::Mysql,
        Dialect::Mysql,
    );
    assert_transpile(
        "SELECT CAST(x AS SIGNED), CAST(y AS UNSIGNED)",
        "SELECT CAST(x AS BIGINT), CAST(y AS UBIGINT)",
        Dialect::Mysql,
        Dialect::Postgres,
    );
    assert_transpile(
        "SELECT CAST(x AS SIGNED), CAST(y AS UNSIGNED)",
        "SELECT CAST(x AS INTEGER), CAST(y AS UBIGINT)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    // Any integer/text cast lowers to mysql's restricted cast-type set.
    assert_transpile(
        "SELECT CAST(a AS BIGINT), CAST(b AS TEXT), CAST(c AS VARCHAR(10))",
        "SELECT CAST(a AS SIGNED), CAST(b AS CHAR), CAST(c AS CHAR(10))",
        Dialect::Postgres,
        Dialect::Mysql,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Function mapping: IFNULL → COALESCE
// (from Python test_mysql.py → test_postgres.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ifnull_to_postgres() {
    assert_transpile(
        "SELECT IFNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_ifnull_to_ansi() {
    assert_transpile(
        "SELECT IFNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Mysql,
        Dialect::Ansi,
    );
}

#[test]
fn test_ifnull_to_duckdb() {
    assert_transpile(
        "SELECT IFNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Mysql,
        Dialect::DuckDb,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// ILIKE → LOWER(x) LIKE LOWER(pattern) for non-ILIKE dialects
// (from Python test_mysql.py, test_sqlite.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ilike_to_mysql() {
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')",
        Dialect::Postgres,
        Dialect::Mysql,
    );
}

#[test]
fn test_ilike_to_sqlite() {
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_ilike_preserved_in_postgres() {
    // ILIKE is native to Postgres; should remain as-is
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        Dialect::Postgres,
        Dialect::Postgres,
    );
}

#[test]
fn test_ilike_preserved_in_duckdb() {
    // DuckDB supports ILIKE natively
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        Dialect::Postgres,
        Dialect::DuckDb,
    );
}

#[test]
fn test_ilike_preserved_in_snowflake() {
    // Snowflake supports ILIKE natively
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        Dialect::Postgres,
        Dialect::Snowflake,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Data type mapping: TEXT ↔ STRING
// (from Python test_bigquery.py, test_postgres.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_text_to_string_bigquery() {
    assert_transpile(
        "SELECT CAST(x AS TEXT) FROM t",
        "SELECT CAST(x AS STRING) FROM t",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

#[test]
fn test_string_to_text_postgres() {
    assert_transpile(
        "SELECT CAST(x AS STRING) FROM t",
        "SELECT CAST(x AS TEXT) FROM t",
        Dialect::BigQuery,
        Dialect::Postgres,
    );
}

#[test]
fn test_string_to_text_mysql() {
    // MySQL CAST renders text types as CHAR (SQLGlot CAST_MAPPING)
    assert_transpile(
        "SELECT CAST(x AS STRING) FROM t",
        "SELECT CAST(x AS CHAR) FROM t",
        Dialect::BigQuery,
        Dialect::Mysql,
    );
}

#[test]
fn test_string_to_text_sqlite() {
    assert_transpile(
        "SELECT CAST(x AS STRING) FROM t",
        "SELECT CAST(x AS TEXT) FROM t",
        Dialect::BigQuery,
        Dialect::Sqlite,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Data type mapping: INT → INT64 (BigQuery)
// (from Python test_bigquery.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_int_to_bigint_bigquery() {
    assert_transpile(
        "SELECT CAST(x AS INT) FROM t",
        "SELECT CAST(x AS INT64) FROM t",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Data type mapping: FLOAT → FLOAT64 (BigQuery)
// (from Python test_bigquery.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_float_to_double_bigquery() {
    assert_transpile(
        "SELECT CAST(x AS FLOAT) FROM t",
        "SELECT CAST(x AS FLOAT64) FROM t",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Data type mapping: BYTEA ↔ BLOB
// (from Python test_postgres.py, test_mysql.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_bytea_to_blob_mysql() {
    assert_transpile(
        "SELECT CAST(x AS BYTEA) FROM t",
        "SELECT CAST(x AS BLOB) FROM t",
        Dialect::Postgres,
        Dialect::Mysql,
    );
}

#[test]
fn test_blob_to_bytea_postgres() {
    assert_transpile(
        "SELECT CAST(x AS BLOB) FROM t",
        "SELECT CAST(x AS BYTEA) FROM t",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_bytea_to_blob_sqlite() {
    assert_transpile(
        "SELECT CAST(x AS BYTEA) FROM t",
        "SELECT CAST(x AS BLOB) FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Compound transformations – multiple functions + types in one query
// (from Python dialect tests – complex transpilation)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_compound_function_and_type() {
    // MySQL renders Substring as SUBSTRING and casts TEXT as CHAR (SQLGlot)
    assert_transpile(
        "SELECT SUBSTRING(CAST(x AS TEXT), 1, 3) FROM t",
        "SELECT SUBSTRING(CAST(x AS CHAR), 1, 3) FROM t",
        Dialect::Postgres,
        Dialect::Mysql,
    );
}

#[test]
fn test_multiple_functions_in_query() {
    // Multiple function calls that need mapping
    assert_transpile(
        "SELECT LEN(name), IFNULL(email, 'none') FROM users",
        "SELECT LENGTH(name), COALESCE(email, 'none') FROM users",
        Dialect::BigQuery,
        Dialect::Postgres,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Dialect-specific DDL roundtrips
// (from Python dialect tests for CREATE TABLE)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_create_table_identity_each_dialect() {
    let sql = "CREATE TABLE t (id INT, name VARCHAR(100))";
    for dialect in Dialect::all() {
        if *dialect == Dialect::Sqlite {
            assert_transpile(
                sql,
                "CREATE TABLE t (id INTEGER, name TEXT(100))",
                *dialect,
                *dialect,
            );
        } else if *dialect == Dialect::BigQuery {
            assert_transpile(
                sql,
                "CREATE TABLE t (id INT64, name STRING(100))",
                *dialect,
                *dialect,
            );
        } else if *dialect == Dialect::DuckDb {
            assert_transpile(
                sql,
                "CREATE TABLE t (id INT, name TEXT)",
                *dialect,
                *dialect,
            );
        } else {
            assert_identity(sql, *dialect);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Edge cases – same-dialect should be no-op
// (from Python test pattern: read={D}, write={D} should roundtrip)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_same_dialect_noop() {
    // Typed functions normalize to the canonical name for the target dialect.
    // SUBSTR → SUBSTRING (ANSI canonical), NOW → CURRENT_TIMESTAMP (ANSI canonical).
    // IFNULL is canonicalized read-side to COALESCE (matching SQLGlot).
    assert_transpile(
        "SELECT SUBSTR(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        Dialect::Ansi,
        Dialect::Ansi,
    );
    // ANSI does not canonicalize NOW (only postgres/presto families,
    // databricks, and exasol do in SQLGlot), so it round-trips.
    assert_transpile("SELECT NOW()", "SELECT NOW()", Dialect::Ansi, Dialect::Ansi);
    assert_transpile(
        "SELECT LEN(x) FROM t",
        "SELECT LENGTH(x) FROM t",
        Dialect::Ansi,
        Dialect::Ansi,
    );
    assert_transpile(
        "SELECT IFNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Ansi,
        Dialect::Ansi,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Dialect transform on INSERT/UPDATE
// (from Python dialect tests)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_transpile_insert_across_dialects() {
    let sql = "INSERT INTO t VALUES (1, 'a')";
    for write_d in Dialect::all() {
        assert_transpile(sql, sql, Dialect::Ansi, *write_d);
    }
}

#[test]
fn test_transpile_update_identity() {
    let sql = "UPDATE t SET a = 1 WHERE b = 2";
    for dialect in [Dialect::Ansi, Dialect::Postgres, Dialect::Mysql] {
        assert_identity(sql, dialect);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: IFNULL → COALESCE (read-side canonicalization, matching SQLGlot)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ifnull_to_tsql() {
    // SQLGlot canonicalizes IFNULL to COALESCE; every target (incl. T-SQL)
    // renders COALESCE.
    assert_transpile(
        "SELECT IFNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Mysql,
        Dialect::Tsql,
    );
}

#[test]
fn test_isnull_to_postgres() {
    assert_transpile(
        "SELECT ISNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Tsql,
        Dialect::Postgres,
    );
}

#[test]
fn test_isnull_to_mysql() {
    assert_transpile(
        "SELECT ISNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Tsql,
        Dialect::Mysql,
    );
}

#[test]
fn test_isnull_to_sqlite() {
    assert_transpile(
        "SELECT ISNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Tsql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_isnull_is_null_predicate() {
    assert_transpile(
        "SELECT ISNULL(a, b) FROM t",
        "SELECT (a IS NULL) FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: NVL mapping (Oracle)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_nvl_to_postgres() {
    assert_transpile(
        "SELECT NVL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Oracle,
        Dialect::Postgres,
    );
}

#[test]
fn test_nvl_to_mysql() {
    assert_transpile(
        "SELECT NVL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Oracle,
        Dialect::Mysql,
    );
}

#[test]
fn test_nvl_to_tsql() {
    assert_transpile(
        "SELECT NVL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Oracle,
        Dialect::Tsql,
    );
}

#[test]
fn test_nvl_to_snowflake() {
    // Python SQLGlot rewrites NVL to COALESCE for Snowflake too.
    assert_transpile(
        "SELECT NVL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Oracle,
        Dialect::Snowflake,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: RANDOM / RAND cross-dialect mapping
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_random_postgres_to_mysql() {
    assert_transpile(
        "SELECT RANDOM()",
        "SELECT RAND()",
        Dialect::Postgres,
        Dialect::Mysql,
    );
}

#[test]
fn test_rand_mysql_to_postgres() {
    assert_transpile(
        "SELECT RAND()",
        "SELECT RANDOM()",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_rand_to_duckdb() {
    assert_transpile(
        "SELECT RAND()",
        "SELECT RANDOM()",
        Dialect::Mysql,
        Dialect::DuckDb,
    );
}

#[test]
fn test_rand_to_sqlite() {
    assert_transpile(
        "SELECT RAND()",
        "SELECT RANDOM()",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: SUBSTR mapping for Hive/Spark family
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_substring_to_hive() {
    // The hive family renders Substring as SUBSTRING (SQLGlot)
    assert_transpile(
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        Dialect::Postgres,
        Dialect::Hive,
    );
}

#[test]
fn test_substring_to_spark() {
    assert_transpile(
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        Dialect::Postgres,
        Dialect::Spark,
    );
}

#[test]
fn test_substring_to_databricks() {
    assert_transpile(
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        Dialect::Postgres,
        Dialect::Databricks,
    );
}

#[test]
fn test_substring_to_oracle() {
    assert_transpile(
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        "SELECT SUBSTR(x, 1, 3) FROM t",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_substr_to_presto() {
    // The presto family renders Substring as SUBSTR (SQLGlot)
    assert_transpile(
        "SELECT SUBSTR(x, 1, 3) FROM t",
        "SELECT SUBSTR(x, 1, 3) FROM t",
        Dialect::Mysql,
        Dialect::Presto,
    );
}

#[test]
fn test_substr_to_trino() {
    assert_transpile(
        "SELECT SUBSTR(x, 1, 3) FROM t",
        "SELECT SUBSTR(x, 1, 3) FROM t",
        Dialect::Mysql,
        Dialect::Trino,
    );
}

#[test]
fn test_substr_to_clickhouse() {
    assert_transpile(
        "SELECT SUBSTR(x, 1, 3) FROM t",
        "SELECT SUBSTRING(x, 1, 3) FROM t",
        Dialect::Mysql,
        Dialect::ClickHouse,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: LEN ↔ LENGTH for T-SQL
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_length_to_tsql() {
    assert_transpile(
        "SELECT LENGTH(x) FROM t",
        "SELECT LEN(x) FROM t",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_len_tsql_to_postgres() {
    assert_transpile(
        "SELECT LEN(x) FROM t",
        "SELECT LENGTH(x) FROM t",
        Dialect::Tsql,
        Dialect::Postgres,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: ILIKE to other dialects
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ilike_to_oracle() {
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_ilike_to_tsql() {
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_ilike_to_teradata() {
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')",
        Dialect::Postgres,
        Dialect::Teradata,
    );
}

#[test]
fn test_ilike_preserved_in_clickhouse() {
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        Dialect::Postgres,
        Dialect::ClickHouse,
    );
}

#[test]
fn test_ilike_preserved_in_redshift() {
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        Dialect::Postgres,
        Dialect::Redshift,
    );
}

#[test]
fn test_ilike_preserved_in_trino() {
    assert_transpile(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        Dialect::Postgres,
        Dialect::Trino,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: Data type mapping for Hive/Spark family
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_text_to_string_hive() {
    assert_transpile(
        "SELECT CAST(x AS TEXT) FROM t",
        "SELECT CAST(x AS STRING) FROM t",
        Dialect::Postgres,
        Dialect::Hive,
    );
}

#[test]
fn test_text_to_string_spark() {
    assert_transpile(
        "SELECT CAST(x AS TEXT) FROM t",
        "SELECT CAST(x AS STRING) FROM t",
        Dialect::Postgres,
        Dialect::Spark,
    );
}

#[test]
fn test_string_to_text_redshift() {
    assert_transpile(
        "SELECT CAST(x AS STRING) FROM t",
        "SELECT CAST(x AS TEXT) FROM t",
        Dialect::BigQuery,
        Dialect::Redshift,
    );
}

#[test]
fn test_bytea_to_blob_oracle() {
    assert_transpile(
        "SELECT CAST(x AS BYTEA) FROM t",
        "SELECT CAST(x AS BLOB) FROM t",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_bytea_to_blob_hive() {
    assert_transpile(
        "SELECT CAST(x AS BYTEA) FROM t",
        "SELECT CAST(x AS BLOB) FROM t",
        Dialect::Postgres,
        Dialect::Hive,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: Postgres-family dialects (Redshift, Materialize, RisingWave)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_postgres_family_blob_to_bytea() {
    // All Postgres-family dialects should map BLOB → BYTEA
    for target in [Dialect::Redshift, Dialect::Materialize, Dialect::RisingWave] {
        assert_transpile(
            "SELECT CAST(x AS BLOB) FROM t",
            "SELECT CAST(x AS BYTEA) FROM t",
            Dialect::Mysql,
            target,
        );
    }
}

#[test]
fn test_postgres_family_string_to_text() {
    for target in [Dialect::Redshift, Dialect::Materialize, Dialect::RisingWave] {
        assert_transpile(
            "SELECT CAST(x AS STRING) FROM t",
            "SELECT CAST(x AS TEXT) FROM t",
            Dialect::BigQuery,
            target,
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: MySQL-family dialects (Doris, SingleStore, StarRocks)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_mysql_family_substring() {
    // The mysql family renders Substring as SUBSTRING (SQLGlot)
    for target in [Dialect::Doris, Dialect::SingleStore, Dialect::StarRocks] {
        assert_transpile(
            "SELECT SUBSTRING(x, 1, 3) FROM t",
            "SELECT SUBSTRING(x, 1, 3) FROM t",
            Dialect::Postgres,
            target,
        );
    }
}

#[test]
fn test_mysql_family_ifnull_to_coalesce() {
    // SQLGlot canonicalizes IFNULL to COALESCE everywhere, including the
    // MySQL family.
    for target in [Dialect::Doris, Dialect::SingleStore, Dialect::StarRocks] {
        assert_transpile(
            "SELECT IFNULL(a, b) FROM t",
            "SELECT COALESCE(a, b) FROM t",
            Dialect::Mysql,
            target,
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: T-SQL family (Fabric)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_fabric_same_as_tsql() {
    // Fabric is T-SQL variant, should behave the same
    assert_transpile(
        "SELECT NOW()",
        "SELECT GETDATE()",
        Dialect::Postgres,
        Dialect::Fabric,
    );
    assert_transpile(
        "SELECT IFNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Mysql,
        Dialect::Fabric,
    );
    assert_transpile(
        "SELECT LENGTH(x) FROM t",
        "SELECT LEN(x) FROM t",
        Dialect::Postgres,
        Dialect::Fabric,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// NEW: Compound transforms across all new dialects
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_compound_postgres_to_hive() {
    // Substring stays SUBSTRING for hive (SQLGlot); TEXT → STRING
    assert_transpile(
        "SELECT SUBSTRING(CAST(x AS TEXT), 1, 3) FROM t",
        "SELECT SUBSTRING(CAST(x AS STRING), 1, 3) FROM t",
        Dialect::Postgres,
        Dialect::Hive,
    );
}

#[test]
fn test_compound_postgres_to_tsql() {
    // LEN stays as LENGTH→LEN, ILIKE→LOWER/LIKE
    assert_transpile(
        "SELECT LENGTH(name) FROM t WHERE name ILIKE '%test%'",
        "SELECT LEN(name) FROM t WHERE LOWER(name) LIKE LOWER('%test%')",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_compound_oracle_to_bigquery() {
    assert_transpile(
        "SELECT NVL(SUBSTR(x, 1, 3), 'default') FROM t",
        "SELECT COALESCE(SUBSTRING(x, 1, 3), 'default') FROM t",
        Dialect::Oracle,
        Dialect::BigQuery,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// validate_all-style tests — mirrors Python's test_dialect.py patterns
// Tests a single canonical SQL generating correctly across many dialects.
// ═════════════════════════════════════════════════════════════════════════════

// ── test_random (from Python test_dialect.py::test_random) ──

#[test]
fn test_validate_all_rand() {
    // Python: RAND() → writes to many dialects
    assert_validate_all(
        "SELECT RAND()",
        Dialect::Mysql,
        &[
            (Dialect::Mysql, "SELECT RAND()"),
            (Dialect::Postgres, "SELECT RANDOM()"),
            (Dialect::DuckDb, "SELECT RANDOM()"),
            (Dialect::Sqlite, "SELECT RANDOM()"),
            (Dialect::Snowflake, "SELECT RANDOM()"),
            (Dialect::Teradata, "SELECT RANDOM()"),
            (Dialect::Redshift, "SELECT RANDOM()"),
            (Dialect::Oracle, "SELECT DBMS_RANDOM.VALUE()"),
            (Dialect::ClickHouse, "SELECT randCanonical()"),
            (Dialect::BigQuery, "SELECT RAND()"),
            (Dialect::Hive, "SELECT RAND()"),
            (Dialect::Spark, "SELECT RAND()"),
            (Dialect::Presto, "SELECT RAND()"),
            (Dialect::Trino, "SELECT RAND()"),
            (Dialect::Tsql, "SELECT RAND()"),
            (Dialect::Databricks, "SELECT RAND()"),
            (Dialect::Athena, "SELECT RAND()"),
            (Dialect::Doris, "SELECT RAND()"),
            (Dialect::StarRocks, "SELECT RAND()"),
        ],
    );
}

#[test]
fn test_validate_all_random_reads() {
    // Python: read RANDOM() from postgres, write to many
    assert_validate_all(
        "SELECT RANDOM()",
        Dialect::Postgres,
        &[
            (Dialect::Postgres, "SELECT RANDOM()"),
            (Dialect::DuckDb, "SELECT RANDOM()"),
            (Dialect::Sqlite, "SELECT RANDOM()"),
            (Dialect::Mysql, "SELECT RAND()"),
            (Dialect::BigQuery, "SELECT RAND()"),
            (Dialect::Hive, "SELECT RAND()"),
            (Dialect::Presto, "SELECT RAND()"),
            (Dialect::Tsql, "SELECT RAND()"),
        ],
    );
}

// ── test_cast type mappings (from Python test_dialect.py::test_cast) ──

#[test]
fn test_validate_all_cast_text() {
    // Python: CAST(a AS TEXT) → writes to many dialects
    assert_validate_all(
        "SELECT CAST(a AS TEXT)",
        Dialect::Postgres,
        &[
            (Dialect::Postgres, "SELECT CAST(a AS TEXT)"),
            (Dialect::Mysql, "SELECT CAST(a AS CHAR)"),
            (Dialect::Sqlite, "SELECT CAST(a AS TEXT)"),
            (Dialect::BigQuery, "SELECT CAST(a AS STRING)"),
            (Dialect::DuckDb, "SELECT CAST(a AS TEXT)"),
            (Dialect::Hive, "SELECT CAST(a AS STRING)"),
            (Dialect::Spark, "SELECT CAST(a AS STRING)"),
            (Dialect::Databricks, "SELECT CAST(a AS STRING)"),
            (Dialect::Redshift, "SELECT CAST(a AS TEXT)"),
            (Dialect::Materialize, "SELECT CAST(a AS TEXT)"),
        ],
    );
}

#[test]
fn test_validate_all_cast_string_to_text() {
    // Python: CAST(a AS STRING) → writes to many dialects
    assert_validate_all(
        "SELECT CAST(a AS STRING)",
        Dialect::BigQuery,
        &[
            (Dialect::Postgres, "SELECT CAST(a AS TEXT)"),
            (Dialect::Mysql, "SELECT CAST(a AS CHAR)"),
            (Dialect::Sqlite, "SELECT CAST(a AS TEXT)"),
            (Dialect::BigQuery, "SELECT CAST(a AS STRING)"),
            (Dialect::DuckDb, "SELECT CAST(a AS TEXT)"),
            (Dialect::Redshift, "SELECT CAST(a AS TEXT)"),
            (Dialect::Materialize, "SELECT CAST(a AS TEXT)"),
            (Dialect::RisingWave, "SELECT CAST(a AS TEXT)"),
            (Dialect::Doris, "SELECT CAST(a AS TEXT)"),
            (Dialect::SingleStore, "SELECT CAST(a AS TEXT)"),
            (Dialect::StarRocks, "SELECT CAST(a AS TEXT)"),
        ],
    );
}

#[test]
fn test_validate_all_cast_bytea() {
    // Python: CAST(x AS BYTEA) → writes to many dialects
    assert_validate_all(
        "SELECT CAST(x AS BYTEA)",
        Dialect::Postgres,
        &[
            (Dialect::Postgres, "SELECT CAST(x AS BYTEA)"),
            (Dialect::Redshift, "SELECT CAST(x AS BYTEA)"),
            (Dialect::Materialize, "SELECT CAST(x AS BYTEA)"),
            (Dialect::Mysql, "SELECT CAST(x AS BLOB)"),
            (Dialect::Sqlite, "SELECT CAST(x AS BLOB)"),
            (Dialect::Oracle, "SELECT CAST(x AS BLOB)"),
            (Dialect::Hive, "SELECT CAST(x AS BLOB)"),
            (Dialect::Doris, "SELECT CAST(x AS BLOB)"),
        ],
    );
}

// ── IFNULL/NVL/ISNULL/COALESCE validate_all ──

#[test]
fn test_validate_all_ifnull_writes() {
    // Python: IFNULL(x, y) → writes to many dialects
    assert_validate_all(
        "SELECT IFNULL(x, y)",
        Dialect::Mysql,
        &[
            // SQLGlot canonicalizes IFNULL to COALESCE for every target.
            (Dialect::Mysql, "SELECT COALESCE(x, y)"),
            (Dialect::Doris, "SELECT COALESCE(x, y)"),
            (Dialect::SingleStore, "SELECT COALESCE(x, y)"),
            (Dialect::StarRocks, "SELECT COALESCE(x, y)"),
            (Dialect::Ansi, "SELECT COALESCE(x, y)"),
            (Dialect::Postgres, "SELECT COALESCE(x, y)"),
            (Dialect::Redshift, "SELECT COALESCE(x, y)"),
            (Dialect::DuckDb, "SELECT COALESCE(x, y)"),
            (Dialect::Sqlite, "SELECT COALESCE(x, y)"),
            (Dialect::BigQuery, "SELECT COALESCE(x, y)"),
            (Dialect::Snowflake, "SELECT COALESCE(x, y)"),
            (Dialect::Hive, "SELECT COALESCE(x, y)"),
            (Dialect::Spark, "SELECT COALESCE(x, y)"),
            (Dialect::Presto, "SELECT COALESCE(x, y)"),
            (Dialect::Trino, "SELECT COALESCE(x, y)"),
            (Dialect::ClickHouse, "SELECT COALESCE(x, y)"),
            (Dialect::Oracle, "SELECT COALESCE(x, y)"),
            (Dialect::Tsql, "SELECT COALESCE(x, y)"),
            (Dialect::Fabric, "SELECT COALESCE(x, y)"),
        ],
    );
}

#[test]
fn test_validate_all_nvl_writes() {
    // Python: NVL(x, y) → writes to many dialects
    assert_validate_all(
        "SELECT NVL(x, y)",
        Dialect::Oracle,
        &[
            (Dialect::Oracle, "SELECT NVL(x, y)"),
            (Dialect::Snowflake, "SELECT COALESCE(x, y)"),
            (Dialect::Postgres, "SELECT COALESCE(x, y)"),
            (Dialect::BigQuery, "SELECT COALESCE(x, y)"),
            (Dialect::DuckDb, "SELECT COALESCE(x, y)"),
            (Dialect::Presto, "SELECT COALESCE(x, y)"),
            (Dialect::Hive, "SELECT COALESCE(x, y)"),
            (Dialect::Mysql, "SELECT COALESCE(x, y)"),
            (Dialect::Sqlite, "SELECT COALESCE(x, y)"),
            (Dialect::Tsql, "SELECT COALESCE(x, y)"),
            (Dialect::Fabric, "SELECT COALESCE(x, y)"),
        ],
    );
}

#[test]
fn test_validate_all_isnull_writes() {
    // Python: ISNULL(x, y) from T-SQL → writes to many dialects
    assert_validate_all(
        "SELECT ISNULL(x, y)",
        Dialect::Tsql,
        &[
            (Dialect::Tsql, "SELECT ISNULL(x, y)"),
            (Dialect::Fabric, "SELECT ISNULL(x, y)"),
            (Dialect::Postgres, "SELECT COALESCE(x, y)"),
            (Dialect::BigQuery, "SELECT COALESCE(x, y)"),
            (Dialect::DuckDb, "SELECT COALESCE(x, y)"),
            (Dialect::Snowflake, "SELECT COALESCE(x, y)"),
            (Dialect::Mysql, "SELECT COALESCE(x, y)"),
            (Dialect::Sqlite, "SELECT COALESCE(x, y)"),
        ],
    );
}

#[test]
fn test_validate_all_coalesce_metadata_writes() {
    assert_validate_all(
        "SELECT NVL(x, y)",
        Dialect::BigQuery,
        &[
            (Dialect::BigQuery, "SELECT NVL(x, y)"),
            (Dialect::ClickHouse, "SELECT NVL(x, y)"),
            (Dialect::Oracle, "SELECT COALESCE(x, y)"),
            (Dialect::Postgres, "SELECT COALESCE(x, y)"),
        ],
    );

    assert_validate_all(
        "SELECT IFNULL(x, y)",
        Dialect::ClickHouse,
        &[
            (Dialect::BigQuery, "SELECT IFNULL(x, y)"),
            (Dialect::ClickHouse, "SELECT IFNULL(x, y)"),
            (Dialect::Oracle, "SELECT COALESCE(x, y)"),
            (Dialect::Postgres, "SELECT COALESCE(x, y)"),
        ],
    );
}

// ── NOW/CURRENT_TIMESTAMP/GETDATE validate_all ──

#[test]
fn test_validate_all_now_writes() {
    // Python: NOW() from Postgres → writes to many dialects
    assert_validate_all(
        "SELECT NOW()",
        Dialect::Postgres,
        &[
            // Postgres canonicalizes NOW() to CurrentTimestamp (SQLGlot),
            // rendered per target.
            // → bare CURRENT_TIMESTAMP
            (Dialect::Postgres, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::DuckDb, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::Sqlite, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::Oracle, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::Presto, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::Trino, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::Athena, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::Teradata, "SELECT CURRENT_TIMESTAMP"),
            // → CURRENT_TIMESTAMP()
            (Dialect::Mysql, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::Ansi, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::BigQuery, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::Snowflake, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::Hive, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::Spark, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::Databricks, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::ClickHouse, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::Exasol, "SELECT CURRENT_TIMESTAMP()"),
            // → GETDATE
            (Dialect::Tsql, "SELECT GETDATE()"),
            (Dialect::Fabric, "SELECT GETDATE()"),
            (Dialect::Redshift, "SELECT GETDATE()"),
        ],
    );
}

#[test]
fn test_validate_all_getdate_writes() {
    // Python: GETDATE() from T-SQL parses to CurrentTimestamp (SQLGlot),
    // rendered per target.
    assert_validate_all(
        "SELECT GETDATE()",
        Dialect::Tsql,
        &[
            (Dialect::Tsql, "SELECT GETDATE()"),
            (Dialect::Fabric, "SELECT GETDATE()"),
            (Dialect::Postgres, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::Sqlite, "SELECT CURRENT_TIMESTAMP"),
            (Dialect::BigQuery, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::Snowflake, "SELECT CURRENT_TIMESTAMP()"),
            (Dialect::Hive, "SELECT CURRENT_TIMESTAMP()"),
        ],
    );
}

#[test]
fn test_validate_all_current_date_alias_writes() {
    assert_validate_all(
        "SELECT CURDATE()",
        Dialect::Mysql,
        &[
            (Dialect::Mysql, "SELECT CURRENT_DATE"),
            (Dialect::Doris, "SELECT CURRENT_DATE()"),
            (Dialect::SingleStore, "SELECT CURRENT_DATE()"),
            (Dialect::StarRocks, "SELECT CURRENT_DATE"),
            (Dialect::Sqlite, "SELECT CURRENT_DATE"),
            (Dialect::Postgres, "SELECT CURRENT_DATE"),
            (Dialect::Tsql, "SELECT CAST(GETDATE() AS DATE)"),
        ],
    );
}

#[test]
fn test_validate_all_time_from_parts_writes() {
    assert_validate_all(
        "SELECT MAKETIME(15, 30, 0)",
        Dialect::Mysql,
        &[
            (Dialect::Mysql, "SELECT MAKETIME(15, 30, 0)"),
            (Dialect::Sqlite, "SELECT TIME_FROM_PARTS(15, 30, 0)"),
            (Dialect::Postgres, "SELECT MAKE_TIME(15, 30, 0)"),
            (Dialect::DuckDb, "SELECT MAKE_TIME(15, 30, 0)"),
            (Dialect::BigQuery, "SELECT TIME(15, 30, 0)"),
            (Dialect::Tsql, "SELECT TIMEFROMPARTS(15, 30, 0, 0, 0)"),
        ],
    );
}

#[test]
fn test_validate_all_unix_to_time_writes() {
    assert_validate_all(
        "SELECT FROM_UNIXTIME(col)",
        Dialect::Mysql,
        &[
            (Dialect::Mysql, "SELECT FROM_UNIXTIME(col)"),
            (Dialect::Sqlite, "SELECT UNIX_TO_TIME(col)"),
            (Dialect::Postgres, "SELECT TO_TIMESTAMP(col)"),
            (Dialect::DuckDb, "SELECT TO_TIMESTAMP(col)"),
            (Dialect::BigQuery, "SELECT TIMESTAMP_SECONDS(col)"),
            (Dialect::Tsql, "SELECT UNIX_TO_TIME(col)"),
        ],
    );
}

// ── SUBSTR/SUBSTRING validate_all ──

#[test]
fn test_validate_all_substring_writes() {
    // Python: SUBSTRING(x, 1, 3) → writes to many dialects
    assert_validate_all(
        "SELECT SUBSTRING(x, 1, 3)",
        Dialect::Postgres,
        &[
            // SQL-standard FROM/FOR form (postgres family)
            (Dialect::Postgres, "SELECT SUBSTRING(x FROM 1 FOR 3)"),
            (Dialect::Redshift, "SELECT SUBSTRING(x FROM 1 FOR 3)"),
            (Dialect::Materialize, "SELECT SUBSTRING(x FROM 1 FOR 3)"),
            // SUBSTRING dialects
            (Dialect::DuckDb, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::BigQuery, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::Snowflake, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::ClickHouse, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::Ansi, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::Sqlite, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::Mysql, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::Hive, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::Spark, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::Databricks, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::Doris, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::SingleStore, "SELECT SUBSTRING(x, 1, 3)"),
            (Dialect::StarRocks, "SELECT SUBSTRING(x, 1, 3)"),
            // SUBSTR dialects (oracle, presto family)
            (Dialect::Oracle, "SELECT SUBSTR(x, 1, 3)"),
            (Dialect::Presto, "SELECT SUBSTR(x, 1, 3)"),
            (Dialect::Trino, "SELECT SUBSTR(x, 1, 3)"),
            (Dialect::Athena, "SELECT SUBSTR(x, 1, 3)"),
        ],
    );
}

// ── LEN/LENGTH validate_all ──

#[test]
fn test_validate_all_len_writes() {
    // Python: LEN(x) → writes to many dialects
    assert_validate_all(
        "SELECT LEN(x)",
        Dialect::BigQuery,
        &[
            // Known divergence: SQLGlot preserves the LEN spelling for a
            // bigquery identity round-trip; we render the canonical LENGTH.
            (Dialect::BigQuery, "SELECT LENGTH(x)"),
            (Dialect::Snowflake, "SELECT LENGTH(x)"),
            (Dialect::Tsql, "SELECT LEN(x)"),
            (Dialect::Fabric, "SELECT LEN(x)"),
            (Dialect::Postgres, "SELECT LENGTH(x)"),
            // MySQL family and clickhouse reserve LENGTH for byte counting;
            // character length renders CHAR_LENGTH (SQLGlot).
            (Dialect::Mysql, "SELECT CHAR_LENGTH(x)"),
            (Dialect::ClickHouse, "SELECT CHAR_LENGTH(x)"),
            (Dialect::Sqlite, "SELECT LENGTH(x)"),
            (Dialect::DuckDb, "SELECT LENGTH(x)"),
            (Dialect::Oracle, "SELECT LENGTH(x)"),
            (Dialect::Hive, "SELECT LENGTH(x)"),
            (Dialect::Presto, "SELECT LENGTH(x)"),
        ],
    );
}

// ── ILIKE validate_all ──

#[test]
fn test_validate_all_ilike_writes() {
    // Python: x ILIKE '%y' → writes to many dialects
    assert_validate_all(
        "SELECT * FROM t WHERE x ILIKE '%y'",
        Dialect::Postgres,
        &[
            // Native ILIKE support
            (Dialect::Postgres, "SELECT * FROM t WHERE x ILIKE '%y'"),
            (Dialect::DuckDb, "SELECT * FROM t WHERE x ILIKE '%y'"),
            (Dialect::Snowflake, "SELECT * FROM t WHERE x ILIKE '%y'"),
            (Dialect::ClickHouse, "SELECT * FROM t WHERE x ILIKE '%y'"),
            (Dialect::Redshift, "SELECT * FROM t WHERE x ILIKE '%y'"),
            (Dialect::Trino, "SELECT * FROM t WHERE x ILIKE '%y'"),
            (Dialect::Presto, "SELECT * FROM t WHERE x ILIKE '%y'"),
            (Dialect::Spark, "SELECT * FROM t WHERE x ILIKE '%y'"),
            (Dialect::Hive, "SELECT * FROM t WHERE x ILIKE '%y'"),
            // Lowered to LIKE
            (
                Dialect::Mysql,
                "SELECT * FROM t WHERE LOWER(x) LIKE LOWER('%y')",
            ),
            (
                Dialect::Sqlite,
                "SELECT * FROM t WHERE LOWER(x) LIKE LOWER('%y')",
            ),
            (
                Dialect::Oracle,
                "SELECT * FROM t WHERE LOWER(x) LIKE LOWER('%y')",
            ),
            (
                Dialect::Tsql,
                "SELECT * FROM t WHERE LOWER(x) LIKE LOWER('%y')",
            ),
            (
                Dialect::Ansi,
                "SELECT * FROM t WHERE LOWER(x) LIKE LOWER('%y')",
            ),
            (
                Dialect::BigQuery,
                "SELECT * FROM t WHERE LOWER(x) LIKE LOWER('%y')",
            ),
            (
                Dialect::Teradata,
                "SELECT * FROM t WHERE LOWER(x) LIKE LOWER('%y')",
            ),
        ],
    );
}

// ── NOT ILIKE validate_all ──

#[test]
fn test_validate_all_not_ilike() {
    assert_validate_all(
        "SELECT * FROM t WHERE x NOT ILIKE '%y'",
        Dialect::Postgres,
        &[
            (Dialect::Postgres, "SELECT * FROM t WHERE x NOT ILIKE '%y'"),
            (Dialect::DuckDb, "SELECT * FROM t WHERE x NOT ILIKE '%y'"),
            (Dialect::Snowflake, "SELECT * FROM t WHERE x NOT ILIKE '%y'"),
            (
                Dialect::Mysql,
                "SELECT * FROM t WHERE LOWER(x) NOT LIKE LOWER('%y')",
            ),
            (
                Dialect::Oracle,
                "SELECT * FROM t WHERE LOWER(x) NOT LIKE LOWER('%y')",
            ),
        ],
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Per-dialect identity tests (from Python test_<dialect>.py::test_<dialect>)
// ═════════════════════════════════════════════════════════════════════════════

// ── BigQuery (from test_bigquery.py) ──

#[test]
fn test_bigquery_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a = 1",
        "SELECT CAST(x AS STRING)",
        "SELECT a, b, c FROM t GROUP BY 1, 2, 3",
        "SELECT * FROM t1 INNER JOIN t2 ON t1.id = t2.id",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::BigQuery);
    }
}

// ── ClickHouse (from test_clickhouse.py) ──

#[test]
fn test_clickhouse_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t LIMIT 10",
        "SELECT a, COUNT(*) FROM t GROUP BY a",
        "SELECT CAST(x AS INT)",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::ClickHouse);
    }
}

// ── DuckDB (from test_duckdb.py) ──

#[test]
fn test_duckdb_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a ILIKE '%x%'",
        "SELECT CAST('2024-01-01' AS DATE)",
        "SELECT a, b FROM t ORDER BY a LIMIT 5",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::DuckDb);
    }
}

// ── Hive (from test_hive.py) ──

#[test]
fn test_hive_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t",
        "SELECT a, b FROM t WHERE a > 1 ORDER BY a LIMIT 10",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Hive);
    }
}

// ── MySQL (from test_mysql.py) ──

#[test]
fn test_mysql_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a LIKE '%test%'",
        "SELECT a FROM t GROUP BY a HAVING COUNT(*) > 1",
        "INSERT INTO t VALUES (1, 'a')",
        "UPDATE t SET a = 1 WHERE b = 2",
        "DELETE FROM t WHERE a = 1",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Mysql);
    }
}

// ── Oracle (from test_oracle.py) ──

#[test]
fn test_oracle_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE ROWNUM <= 10",
        "SELECT a, b FROM t ORDER BY a",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Oracle);
    }
}

// ── PostgreSQL (from test_postgres.py) ──

#[test]
fn test_postgres_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a ILIKE '%test%'",
        "SELECT CAST(x AS TEXT)",
        "SELECT * FROM t1 LEFT JOIN t2 ON t1.id = t2.id",
        "CREATE TABLE t (id SERIAL PRIMARY KEY, name VARCHAR(100))",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Postgres);
    }
}

// ── Presto (from test_presto.py) ──

#[test]
fn test_presto_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t LIMIT 10",
        "SELECT CAST(x AS VARCHAR)",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Presto);
    }
}

// ── Redshift (from test_redshift.py) ──

#[test]
fn test_redshift_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a ILIKE '%test%'",
        "SELECT a, b FROM t ORDER BY a LIMIT 100",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Redshift);
    }
}

// ── Snowflake (from test_snowflake.py) ──

#[test]
fn test_snowflake_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a ILIKE '%test%'",
        "SELECT CAST(x AS VARCHAR)",
        "SELECT a, b FROM t ORDER BY a LIMIT 10",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Snowflake);
    }
}

// ── Spark (from test_spark.py) ──

#[test]
fn test_spark_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t",
        "SELECT a, COUNT(*) FROM t GROUP BY a",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Spark);
    }
}

// ── SQLite (from test_sqlite.py) ──

#[test]
fn test_sqlite_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a LIKE '%test%'",
        "SELECT TYPEOF(x)",
        "INSERT INTO t VALUES (1, 'a')",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Sqlite);
    }
}

// ── T-SQL (from test_tsql.py) ──

#[test]
fn test_tsql_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT a, b FROM t WHERE a = 1",
        "SELECT CAST(x AS INT) FROM t",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Tsql);
    }
}

// ── Trino (from test_trino via Presto) ──

#[test]
fn test_trino_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t LIMIT 10",
        "SELECT CAST(x AS VARCHAR)",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Trino);
    }
}

// ── Athena ──

#[test]
fn test_athena_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a > 1",
        "SELECT a, b FROM t LIMIT 100",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Athena);
    }
}

// ── Databricks ──

#[test]
fn test_databricks_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t",
        "SELECT a, b FROM t ORDER BY a LIMIT 10",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Databricks);
    }
}

// ── StarRocks ──

#[test]
fn test_starrocks_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a > 1",
        "SELECT * FROM t ORDER BY a LIMIT 10",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::StarRocks);
    }
}

// ── Teradata ──

#[test]
fn test_teradata_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t",
        "SELECT a, b FROM t WHERE a = 1",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Teradata);
    }
}

// ── Exasol ──

#[test]
fn test_exasol_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t",
        "SELECT a, COUNT(*) FROM t GROUP BY a",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Exasol);
    }
}

// ── Materialize ──

#[test]
fn test_materialize_identity() {
    let sqls = [
        "SELECT 1",
        "SELECT * FROM t WHERE a ILIKE '%x%'",
        "SELECT CAST(a AS INT) FROM t",
    ];
    for sql in &sqls {
        assert_identity(sql, Dialect::Materialize);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Cross-dialect DDL tests (from Python test_dialect.py::test_ddl)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_create_table_type_mapping_postgres_to_bigquery() {
    assert_transpile(
        "CREATE TABLE t (id INT, name TEXT, data BYTEA)",
        "CREATE TABLE t (id INT64, name STRING, data BYTES)",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

#[test]
fn test_create_table_type_mapping_postgres_to_hive() {
    assert_transpile(
        "CREATE TABLE t (id INT, name TEXT, data BYTEA)",
        "CREATE TABLE t (id INT, name STRING, data BLOB)",
        Dialect::Postgres,
        Dialect::Hive,
    );
}

#[test]
fn test_create_table_type_mapping_bigquery_to_postgres() {
    assert_transpile(
        "CREATE TABLE t (id INT, name STRING)",
        "CREATE TABLE t (id INT, name TEXT)",
        Dialect::BigQuery,
        Dialect::Postgres,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Cross-dialect INSERT/UPDATE with function transforms
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_insert_with_function_transform() {
    assert_transpile(
        "INSERT INTO t VALUES (NOW(), 'data')",
        "INSERT INTO t VALUES (CURRENT_TIMESTAMP(), 'data')",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

#[test]
fn test_update_with_function_transform() {
    assert_transpile(
        "UPDATE t SET a = NOW() WHERE b = 1",
        "UPDATE t SET a = GETDATE() WHERE b = 1",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_update_with_null_coalesce_transform() {
    assert_transpile(
        "UPDATE t SET a = NVL(b, 0) WHERE c = 1",
        "UPDATE t SET a = COALESCE(b, 0) WHERE c = 1",
        Dialect::Oracle,
        Dialect::Postgres,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Compound multi-function validate_all
// (matches Python's complex validate_all with many functions in one query)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_validate_all_compound_query() {
    // Complex query with SUBSTR + IFNULL + type cast
    assert_validate_all(
        "SELECT IFNULL(SUBSTR(CAST(x AS TEXT), 1, 3), 'none') FROM t",
        Dialect::Mysql,
        &[
            (
                Dialect::Mysql,
                "SELECT COALESCE(SUBSTRING(CAST(x AS CHAR), 1, 3), 'none') FROM t",
            ),
            (
                Dialect::Postgres,
                "SELECT COALESCE(SUBSTRING(CAST(x AS TEXT) FROM 1 FOR 3), 'none') FROM t",
            ),
            (
                Dialect::BigQuery,
                "SELECT COALESCE(SUBSTRING(CAST(x AS STRING), 1, 3), 'none') FROM t",
            ),
            (
                Dialect::Hive,
                "SELECT COALESCE(SUBSTRING(CAST(x AS STRING), 1, 3), 'none') FROM t",
            ),
            (
                Dialect::Tsql,
                "SELECT COALESCE(SUBSTRING(CAST(x AS TEXT), 1, 3), 'none') FROM t",
            ),
            (
                Dialect::Oracle,
                "SELECT COALESCE(SUBSTR(CAST(x AS TEXT), 1, 3), 'none') FROM t",
            ),
        ],
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Transaction identity per dialect (from Python test_dialect.py::test_transactions)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_transaction_identity_all_dialects() {
    for dialect in Dialect::all() {
        assert_transpile("BEGIN", "BEGIN TRANSACTION", *dialect, *dialect);
        assert_identity("COMMIT", *dialect);
        assert_identity("ROLLBACK", *dialect);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// DDL identity per dialect
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_drop_table_identity_all_dialects() {
    for dialect in Dialect::all() {
        assert_identity("DROP TABLE t", *dialect);
        assert_identity("DROP TABLE IF EXISTS t", *dialect);
    }
}

#[test]
fn test_truncate_identity_all_dialects() {
    for dialect in Dialect::all() {
        assert_identity("TRUNCATE TABLE t", *dialect);
    }
}

#[test]
fn test_alter_table_identity_all_dialects() {
    for dialect in Dialect::all() {
        if *dialect == Dialect::Sqlite {
            assert_transpile(
                "ALTER TABLE t ADD COLUMN c INT",
                "ALTER TABLE t ADD COLUMN c INTEGER",
                *dialect,
                *dialect,
            );
        } else if *dialect == Dialect::BigQuery {
            assert_transpile(
                "ALTER TABLE t ADD COLUMN c INT",
                "ALTER TABLE t ADD COLUMN c INT64",
                *dialect,
                *dialect,
            );
        } else {
            assert_identity("ALTER TABLE t ADD COLUMN c INT", *dialect);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Complex SELECT identity per dialect
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_complex_select_identity_all_dialects() {
    let sqls = [
        "SELECT * FROM t1 INNER JOIN t2 ON t1.id = t2.id WHERE t1.a > 1 ORDER BY t1.a LIMIT 10",
        "SELECT a, SUM(b) FROM t GROUP BY a HAVING SUM(b) > 100",
        "WITH cte AS (SELECT * FROM t) SELECT * FROM cte",
        "SELECT * FROM t1 UNION ALL SELECT * FROM t2",
        "SELECT CASE WHEN a > 1 THEN 'yes' ELSE 'no' END FROM t",
        "SELECT a FROM t WHERE a BETWEEN 1 AND 10",
        "SELECT a FROM t WHERE a IN (1, 2, 3)",
        "SELECT a FROM t WHERE a IS NULL",
        "SELECT EXTRACT(YEAR FROM d) FROM t",
    ];
    for dialect in Dialect::all() {
        for sql in &sqls {
            assert_identity(sql, *dialect);
        }
    }
    // CAST identity. SQLite folds INT to INTEGER; MySQL CAST only accepts
    // SIGNED/UNSIGNED/CHAR-style cast types (SQLGlot CAST_MAPPING).
    for dialect in Dialect::all() {
        if *dialect == Dialect::Sqlite {
            assert_transpile(
                "SELECT CAST(a AS INT) FROM t",
                "SELECT CAST(a AS INTEGER) FROM t",
                *dialect,
                *dialect,
            );
        } else if *dialect == Dialect::Mysql {
            assert_transpile(
                "SELECT CAST(a AS INT) FROM t",
                "SELECT CAST(a AS SIGNED) FROM t",
                *dialect,
                *dialect,
            );
        } else if *dialect == Dialect::BigQuery {
            assert_transpile(
                "SELECT CAST(a AS INT) FROM t",
                "SELECT CAST(a AS INT64) FROM t",
                *dialect,
                *dialect,
            );
        } else {
            assert_identity("SELECT CAST(a AS INT) FROM t", *dialect);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SELECT TOP N — Cross-Dialect (Issue #1)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_top_tsql_to_postgres() {
    // T-SQL TOP → Postgres LIMIT
    assert_transpile(
        "SELECT TOP 5 * FROM t",
        "SELECT * FROM t LIMIT 5",
        Dialect::Tsql,
        Dialect::Postgres,
    );
}

#[test]
fn test_top_tsql_to_mysql() {
    // T-SQL TOP → MySQL LIMIT
    assert_transpile(
        "SELECT TOP 10 id FROM t",
        "SELECT id FROM t LIMIT 10",
        Dialect::Tsql,
        Dialect::Mysql,
    );
}

#[test]
fn test_top_tsql_star_to_duckdb() {
    // The exact bug case: TOP N * should not confuse * with multiplication
    assert_transpile(
        "SELECT TOP 5 * FROM t",
        "SELECT * FROM t LIMIT 5",
        Dialect::Tsql,
        Dialect::DuckDb,
    );
}

#[test]
fn test_limit_postgres_to_tsql() {
    // Postgres LIMIT → T-SQL TOP (reverse direction)
    assert_transpile(
        "SELECT * FROM t LIMIT 10",
        "SELECT TOP 10 * FROM t",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_top_parenthesized_tsql_to_postgres() {
    // Parenthesized TOP expr
    assert_transpile(
        "SELECT TOP (5) * FROM t",
        "SELECT * FROM t LIMIT (5)",
        Dialect::Tsql,
        Dialect::Postgres,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// PIVOT / UNPIVOT – cross-dialect identity
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_pivot_identity_tsql() {
    assert_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2', 'Q3', 'Q4')) AS pvt",
        Dialect::Tsql,
    );
}

#[test]
fn test_pivot_identity_snowflake() {
    assert_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2', 'Q3'))",
        Dialect::Snowflake,
    );
}

#[test]
fn test_pivot_identity_bigquery() {
    assert_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1' AS q1, 'Q2' AS q2))",
        Dialect::BigQuery,
    );
}

#[test]
fn test_pivot_identity_oracle() {
    assert_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2'))",
        Dialect::Oracle,
    );
}

#[test]
fn test_unpivot_identity_tsql() {
    assert_identity(
        "SELECT * FROM quarterly UNPIVOT (amount FOR quarter IN (Q1, Q2, Q3, Q4)) AS unpvt",
        Dialect::Tsql,
    );
}

#[test]
fn test_unpivot_identity_snowflake() {
    assert_identity(
        "SELECT * FROM quarterly UNPIVOT (amount FOR quarter IN (Q1, Q2, Q3, Q4))",
        Dialect::Snowflake,
    );
}

#[test]
fn test_pivot_transpile_tsql_to_snowflake() {
    assert_transpile(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2')) AS pvt",
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2')) AS pvt",
        Dialect::Tsql,
        Dialect::Snowflake,
    );
}

#[test]
fn test_unpivot_transpile_tsql_to_snowflake() {
    assert_transpile(
        "SELECT * FROM quarterly UNPIVOT (amount FOR quarter IN (Q1, Q2)) AS unpvt",
        "SELECT * FROM quarterly UNPIVOT (amount FOR quarter IN (Q1, Q2)) AS unpvt",
        Dialect::Tsql,
        Dialect::Snowflake,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// PostgreSQL array type modifier (opt_array_bounds) — CR-004
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_postgres_array_type_cast_identity() {
    assert_identity("SELECT CAST(ARRAY[1, 2, 3] AS INT[])", Dialect::Postgres);
}

#[test]
fn test_postgres_array_type_string_cast() {
    assert_identity("SELECT CAST('{}' AS INT[])", Dialect::Postgres);
}

#[test]
fn test_postgres_array_type_in_function_arg() {
    assert_identity("SELECT MY_FUNC(CAST('{1,2}' AS INT[]))", Dialect::Postgres);
}

#[test]
fn test_postgres_array_type_multi_dimensional() {
    assert_identity("SELECT CAST('{{1,2},{3,4}}' AS INT[][])", Dialect::Postgres);
}

#[test]
fn test_postgres_array_type_text() {
    assert_identity("SELECT CAST(col AS TEXT[])", Dialect::Postgres);
}

#[test]
fn test_postgres_array_type_varchar() {
    assert_identity("SELECT CAST(col AS VARCHAR[])", Dialect::Postgres);
}

#[test]
fn test_postgres_array_type_column_def() {
    assert_identity(
        "CREATE TABLE t (tags TEXT[], scores INT[][])",
        Dialect::Postgres,
    );
}

#[test]
fn test_postgres_array_type_with_bound_ignored() {
    // PostgreSQL accepts [N] but ignores the size; we normalize to []
    assert_transpile(
        "SELECT '{1,2,3}'::int[3]",
        "SELECT CAST('{1,2,3}' AS INT[])",
        Dialect::Postgres,
        Dialect::Postgres,
    );
}

#[test]
fn test_postgres_array_type_to_bigquery() {
    // PostgreSQL INT[] should become ARRAY<INT64> for BigQuery
    assert_transpile(
        "SELECT col::INT[]",
        "SELECT CAST(col AS ARRAY<INT64>)",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

#[test]
fn test_bigquery_array_type_to_postgres() {
    // BigQuery ARRAY<INT> should become INT[] for PostgreSQL
    assert_transpile(
        "SELECT CAST(x AS ARRAY<INT>)",
        "SELECT CAST(x AS INT[])",
        Dialect::BigQuery,
        Dialect::Postgres,
    );
}
