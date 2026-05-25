/// Tests ported from Python sqlglot's `test_transpile.py` and `identity.sql` fixture.
///
/// These test parse→generate roundtrips (identity), normalization transforms,
/// and basic cross-dialect transpilation. Modeled after the `validate` and
/// `validate_identity` helpers in the Python test suite.
use sqlgrok::ast::CreateTableOption;
use sqlgrok::{Dialect, Statement, generate, generate_pretty, parse, transpile};

// ═════════════════════════════════════════════════════════════════════════════
// Helpers (mirrors Python sqlglot's TestTranspile.validate / validate_identity)
// ═════════════════════════════════════════════════════════════════════════════

/// Parse SQL → generate SQL, assert output == input.
/// Equivalent to Python sqlglot's `validate_identity`.
fn validate_identity(sql: &str) {
    let ast =
        parse(sql, Dialect::Ansi).unwrap_or_else(|e| panic!("Parse failed for '{}': {}", sql, e));
    let output = generate(&ast, Dialect::Ansi);
    assert_eq!(output, sql, "\n  Identity roundtrip failed");
}

/// Parse SQL → generate SQL, assert output == expected.
/// Equivalent to Python sqlglot's `validate(sql, target)`.
fn validate(sql: &str, expected: &str) {
    let ast =
        parse(sql, Dialect::Ansi).unwrap_or_else(|e| panic!("Parse failed for '{}': {}", sql, e));
    let output = generate(&ast, Dialect::Ansi);
    assert_eq!(output, expected, "\n  Input: {}", sql);
}

fn validate_with_dialect(sql: &str, expected: &str, read: Dialect, write: Dialect) {
    let result = transpile(sql, read, write)
        .unwrap_or_else(|e| panic!("Transpile failed for '{}': {}", sql, e));
    assert_eq!(
        result, expected,
        "\n  Input: {} ({:?} → {:?})",
        sql, read, write
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – Expressions & Literals
// (from Python identity.sql fixture)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_literals() {
    let cases = [
        "SELECT 1",
        "SELECT 1.0",
        "SELECT 'x'",
        "SELECT ''",
        "SELECT TRUE",
        "SELECT FALSE",
        "SELECT NULL",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_arithmetic() {
    let cases = [
        "SELECT 1 + 1",
        "SELECT 1 - 1",
        "SELECT 1 * 1",
        "SELECT 1 / 1",
        "SELECT 1 % 1",
        "SELECT 1 + 2 * 3",
        "SELECT (1 + 2) * 3",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_comparisons() {
    let cases = [
        "SELECT 1 < 2",
        "SELECT 1 <= 2",
        "SELECT 1 > 2",
        "SELECT 1 >= 2",
        "SELECT 1 <> 2",
        "SELECT 1 = 2",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_boolean_logic() {
    let cases = [
        "SELECT a AND b",
        "SELECT a OR b",
        "SELECT NOT a",
        "SELECT NOT NOT a",
        "SELECT a AND b OR c",
        "SELECT (a OR b) AND c",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_unary() {
    let cases = ["SELECT -1", "SELECT -a", "SELECT +a", "SELECT ~x"];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_bitwise() {
    let cases = [
        "SELECT x & 1",
        "SELECT x | 1",
        "SELECT x ^ 1",
        "SELECT x << 1",
        "SELECT x >> 1",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_postgres_power_and_bitwise_xor_to_sqlite() {
    validate_with_dialect(
        "SELECT 2 ^ 3",
        "SELECT POWER(2, 3)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 5 # 3",
        "SELECT 5 ^ 3",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect("a # b", "a ^ b", Dialect::Postgres, Dialect::Sqlite);
    validate_with_dialect("a # b", "a", Dialect::Mysql, Dialect::Sqlite);
}

#[test]
fn test_postgres_grouping_modifiers_spacing_to_sqlite() {
    validate_with_dialect(
        "SELECT a, SUM(b) FROM t GROUP BY ROLLUP(a)",
        "SELECT a, SUM(b) FROM t GROUP BY ROLLUP (a)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT a, SUM(b) FROM t GROUP BY GROUPING SETS((a),())",
        "SELECT a, SUM(b) FROM t GROUP BY GROUPING SETS ((a), ())",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_create_type_enum_to_sqlite() {
    validate_with_dialect(
        "CREATE TYPE mood AS ENUM ('sad', 'ok')",
        "CREATE TYPE mood AS ENUM('sad', 'ok')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "create type public.mood as enum ('sad')",
        "CREATE TYPE public.mood AS ENUM('sad')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_escaped_string_to_sqlite() {
    validate_with_dialect(
        "SELECT E'a\\nb'",
        "SELECT 'a\nb'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT LENGTH(E'a\\nb')",
        "SELECT LENGTH('a\nb')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT E'a\\'b'",
        "SELECT 'a''b'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT E'a\\nb'",
        "SELECT e'a\\nb'",
        Dialect::Postgres,
        Dialect::Postgres,
    );
}

#[test]
fn test_identity_string_concat() {
    validate_identity("SELECT 'a' || 'b'");
    validate_identity("SELECT a || b || c");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – SELECT basics
// (from Python identity.sql and test_transpile.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_select_basic() {
    let cases = [
        "SELECT * FROM test",
        "SELECT a FROM test",
        "SELECT a, b FROM test",
        "SELECT a, b, c FROM test",
        "SELECT 1 FROM test",
        "SELECT 1 + 1 FROM test",
        "SELECT 1 AS b FROM test",
        "SELECT a AS b FROM test",
        "SELECT test.* FROM test",
        "SELECT a.b FROM a",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_select_distinct() {
    let cases = [
        "SELECT DISTINCT x FROM test",
        "SELECT DISTINCT x, y FROM test",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_qualified_columns() {
    let cases = ["SELECT a.b FROM a"];
    for sql in &cases {
        validate_identity(sql);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – WHERE clause
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_where() {
    let cases = [
        "SELECT a FROM test WHERE a = 1",
        "SELECT a FROM test WHERE a = 1 AND b = 2",
        "SELECT a FROM test WHERE (a > 1)",
        "SELECT a FROM test WHERE NOT FALSE",
        "SELECT a FROM test WHERE a > 1 OR b < 2",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – FROM and JOINs
// (from Python identity.sql: JOIN section)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_joins() {
    let cases = [
        "SELECT 1 FROM a INNER JOIN b ON a.x = b.x",
        "SELECT 1 FROM a LEFT JOIN b ON a.x = b.x",
        "SELECT 1 FROM a RIGHT JOIN b ON a.x = b.x",
        "SELECT 1 FROM a FULL JOIN b ON a.x = b.x",
        "SELECT 1 FROM a CROSS JOIN b",
        // Note: bare JOIN is parsed as INNER JOIN, so INNER JOIN is the identity
        "SELECT 1 FROM a INNER JOIN b USING (x)",
        "SELECT 1 FROM a INNER JOIN b USING (x, y, z)",
        "SELECT 1 FROM a LEFT JOIN b ON a.x = b.x INNER JOIN c ON a.y = c.y",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_join_subquery() {
    validate_identity("SELECT 1 FROM a INNER JOIN (SELECT a FROM c) AS b ON a.x = b.x");
}

#[test]
fn test_identity_multiple_from_tables() {
    assert_eq!(
        transpile("SELECT * FROM a, b", Dialect::Ansi, Dialect::Ansi).unwrap(),
        "SELECT * FROM a, b"
    );
    validate_identity("SELECT * FROM a CROSS JOIN b");
}

#[test]
fn test_comma_join_preserves_sources() {
    assert_eq!(
        transpile(
            "SELECT COUNT(*) FROM cj a, cj b",
            Dialect::Mysql,
            Dialect::Sqlite,
        )
        .unwrap(),
        "SELECT COUNT(*) FROM cj AS a, cj AS b"
    );
    assert_eq!(
        transpile(
            "SELECT COUNT(*) FROM cj a CROSS JOIN cj b",
            Dialect::Mysql,
            Dialect::Sqlite,
        )
        .unwrap(),
        "SELECT COUNT(*) FROM cj AS a CROSS JOIN cj AS b"
    );
}

#[test]
fn test_mysql_group_concat_to_sqlite() {
    assert_eq!(
        transpile(
            "SELECT GROUP_CONCAT(v SEPARATOR '|') FROM gc",
            Dialect::Mysql,
            Dialect::Sqlite,
        )
        .unwrap(),
        "SELECT GROUP_CONCAT(v, '|') FROM gc"
    );
    assert_eq!(
        transpile(
            "SELECT GROUP_CONCAT(v ORDER BY v SEPARATOR '|') FROM gc",
            Dialect::Mysql,
            Dialect::Sqlite,
        )
        .unwrap(),
        "SELECT GROUP_CONCAT(v, '|') FROM gc"
    );
}

#[test]
fn test_mysql_standalone_group_concat_to_sqlite() {
    let cases = [
        (
            "GROUP_CONCAT(DISTINCT x ORDER BY y DESC)",
            "GROUP_CONCAT(DISTINCT x)",
        ),
        (
            "GROUP_CONCAT(x ORDER BY y SEPARATOR z)",
            "GROUP_CONCAT(x, z)",
        ),
        (
            "GROUP_CONCAT(DISTINCT x ORDER BY y DESC SEPARATOR '')",
            "GROUP_CONCAT(DISTINCT x, '')",
        ),
        (
            "GROUP_CONCAT(a, b, c SEPARATOR ',')",
            "GROUP_CONCAT(a || b || c, ',')",
        ),
        (
            "GROUP_CONCAT(a, b, c SEPARATOR '')",
            "GROUP_CONCAT(a || b || c, '')",
        ),
        (
            "GROUP_CONCAT(DISTINCT a, b, c SEPARATOR '')",
            "GROUP_CONCAT(DISTINCT a || b || c, '')",
        ),
        (
            "GROUP_CONCAT(a, b, c ORDER BY d SEPARATOR '')",
            "GROUP_CONCAT(a || b || c, '')",
        ),
        (
            "GROUP_CONCAT(DISTINCT a, b, c ORDER BY d SEPARATOR '')",
            "GROUP_CONCAT(DISTINCT a || b || c, '')",
        ),
    ];

    for (sql, expected) in cases {
        validate_with_dialect(sql, expected, Dialect::Mysql, Dialect::Sqlite);
    }
}

#[test]
fn test_postgres_string_agg_to_sqlite_group_concat() {
    validate_with_dialect(
        "SELECT string_agg(a, ',') FROM t",
        "SELECT GROUP_CONCAT(a, ',') FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_gen_random_uuid_to_sqlite_uuid() {
    validate_with_dialect(
        "SELECT gen_random_uuid()",
        "SELECT UUID()",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_interval_literal_to_sqlite() {
    validate_with_dialect(
        "SELECT NOW() - INTERVAL '1 day'",
        "SELECT CURRENT_TIMESTAMP - INTERVAL '1' DAY",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_order_by_nulls_to_sqlite() {
    validate_with_dialect(
        "SELECT a FROM t ORDER BY b",
        "SELECT a FROM t ORDER BY b NULLS LAST",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT ROW_NUMBER() OVER (PARTITION BY a ORDER BY b) FROM t",
        "SELECT ROW_NUMBER() OVER (PARTITION BY a ORDER BY b NULLS LAST) FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT ROW_NUMBER() OVER (ORDER BY b DESC) FROM t",
        "SELECT ROW_NUMBER() OVER (ORDER BY b DESC NULLS FIRST) FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_json_access_to_sqlite_paths() {
    validate_with_dialect(
        "SELECT data->'k' FROM t",
        "SELECT data -> '$.k' FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT data->>'k' FROM t",
        "SELECT data ->> '$.k' FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT data->0 FROM t",
        "SELECT data -> '$[0]' FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "x #> 'y'",
        "JSONB_EXTRACT(x, 'y')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "x #>> 'y'",
        "JSONB_EXTRACT_SCALAR(x, 'y')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "'{\"a\":[1,2,3],\"b\":[4,5,6]}'::json#>'{a,2}'",
        "JSONB_EXTRACT(CAST('{\"a\":[1,2,3],\"b\":[4,5,6]}' AS JSON), '{a,2}')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "'{\"a\":[1,2,3],\"b\":[4,5,6]}'::json#>>'{a,2}'",
        "JSONB_EXTRACT_SCALAR(CAST('{\"a\":[1,2,3],\"b\":[4,5,6]}' AS JSON), '{a,2}')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "JSON_EXTRACT_PATH('{\"f2\":{\"f3\":1},\"f4\":{\"f5\":99,\"f6\":\"foo\"}}','f4')",
        "'{\"f2\":{\"f3\":1},\"f4\":{\"f5\":99,\"f6\":\"foo\"}}' -> '$.f4'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "JSON_EXTRACT_PATH_TEXT('{\"farm\": [\"a\", \"b\", \"c\"]}', 'farm', '0')",
        "'{\"farm\": [\"a\", \"b\", \"c\"]}' ->> '$.farm[0]'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT JSON_EXTRACT_PATH_TEXT(x, k1, k2, k3) FROM t",
        "SELECT x ->> k1 FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );

    let path_cases = [
        ("JSON_EXTRACT_PATH(x, 'x', 'y', 'z')", "x -> '$.x.y.z'"),
        (
            "JSON_EXTRACT_PATH_TEXT(x, 'space key')",
            "x ->> '$.\"space key\"'",
        ),
        (
            "JSON_EXTRACT_PATH_TEXT(x, 'quote\"key')",
            "x ->> '$.\"quote\\\"key\"'",
        ),
        ("JSON_EXTRACT_PATH(x, '0')", "x -> '$[0]'"),
        ("JSON_EXTRACT_PATH(doc, 'a', '2', 'b')", "doc -> '$.a[2].b'"),
        (
            "JSON_EXTRACT_PATH_TEXT(doc, 'a', '2', 'space key')",
            "doc ->> '$.a[2].\"space key\"'",
        ),
        (
            "JSON_EXTRACT_PATH(doc, 'dash-key', '0')",
            "doc -> '$.\"dash-key\"[0]'",
        ),
        (
            "JSON_EXTRACT_PATH_TEXT(doc, 'quote\"key', '3')",
            "doc ->> '$.\"quote\\\"key\"[3]'",
        ),
        (
            "JSON_EXTRACT_PATH(doc, 'snake_key', 'camelCase', '5')",
            "doc -> '$.snake_key.camelCase[5]'",
        ),
        (
            "JSON_EXTRACT_PATH_TEXT(doc, 'a.b', 'c')",
            "doc ->> '$.\"a.b\".c'",
        ),
        (
            "JSON_EXTRACT_PATH(doc, 'bracket[0]', '1')",
            "doc -> '$.\"bracket[0]\"[1]'",
        ),
    ];
    for (sql, expected) in path_cases {
        validate_with_dialect(sql, expected, Dialect::Postgres, Dialect::Sqlite);
    }
}

#[test]
fn test_mysql_json_extract_to_sqlite_arrow() {
    validate_with_dialect(
        "SELECT JSON_EXTRACT(data, '$.k') FROM t",
        "SELECT data -> '$.k' FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT JSON_EXTRACT_SCALAR(data, '$.k') FROM t",
        "SELECT data ->> '$.k' FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT JSON_EXTRACT('[10, 20, [30, 40]]', '$[1]', '$[0]')",
        "SELECT JSON_EXTRACT('[10, 20, [30, 40]]', '$[1]', '$[0]')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_sqlite_glob_function_to_operator() {
    validate_identity("SELECT 'xyz' GLOB '*y*'");
    validate_with_dialect(
        "SELECT GLOB('*y*', 'xyz')",
        "SELECT 'xyz' GLOB '*y*'",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT GLOB('*y*', 'xyz')",
        "SELECT 'xyz' GLOB '*y*'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT GLOB(a, b, c) FROM t",
        "SELECT b GLOB a FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_text_affinity_to_sqlite() {
    validate_with_dialect(
        "CREATE TABLE foo (bar LONGVARCHAR)",
        "CREATE TABLE foo (bar TEXT)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE foo (bar LONGVARCHAR)",
        "CREATE TABLE foo (bar TEXT)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE foo (bar LONGTEXT)",
        "CREATE TABLE foo (bar TEXT)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE foo (bar MEDIUMTEXT)",
        "CREATE TABLE foo (bar TEXT)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE foo (bar TINYTEXT)",
        "CREATE TABLE foo (bar TEXT)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE foo (bar LONGBLOB)",
        "CREATE TABLE foo (bar BLOB)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE foo (bar MEDIUMBLOB)",
        "CREATE TABLE foo (bar BLOB)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – GROUP BY, HAVING, ORDER BY, LIMIT, OFFSET
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_group_by_having() {
    let cases = [
        "SELECT a, b FROM test GROUP BY a",
        "SELECT a, b FROM test GROUP BY 1",
        "SELECT a, b FROM test GROUP BY a, b",
        "SELECT a, b FROM test WHERE a = 1 GROUP BY a HAVING a = 2",
        "SELECT a, b FROM test WHERE a = 1 GROUP BY a HAVING a = 2 ORDER BY a",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_order_by() {
    let cases = [
        "SELECT a FROM test ORDER BY a",
        "SELECT a FROM test ORDER BY a, b",
        "SELECT a FROM test ORDER BY a DESC",
        "SELECT a FROM test ORDER BY a ASC",
        "SELECT a FROM test ORDER BY a, b DESC",
        "SELECT a FROM test ORDER BY a NULLS FIRST",
        "SELECT a FROM test ORDER BY a DESC NULLS LAST",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_order_by_asc_normalization() {
    validate(
        "SELECT a FROM test ORDER BY a ASC, b DESC",
        "SELECT a FROM test ORDER BY a ASC, b DESC",
    );
}

#[test]
fn test_sqlite_order_by_explicit_asc_parity() {
    validate_with_dialect(
        "SELECT fname, lname, age FROM person ORDER BY age DESC NULLS FIRST, fname ASC NULLS LAST, lname",
        "SELECT fname, lname, age FROM person ORDER BY age DESC NULLS FIRST, fname ASC NULLS LAST, lname",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT item AS \"item\", some AS \"some\" FROM data WHERE (item = 'value_1' COLLATE NOCASE) AND (some = 't' COLLATE NOCASE) ORDER BY item ASC LIMIT 1 OFFSET 0",
        "SELECT item AS \"item\", some AS \"some\" FROM data WHERE (item = 'value_1' COLLATE NOCASE) AND (some = 't' COLLATE NOCASE) ORDER BY item ASC LIMIT 1 OFFSET 0",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
}

#[test]
fn test_sqlite_pragma_and_database_commands() {
    let cases = [
        ("PRAGMA table_info", "PRAGMA table_info"),
        ("PRAGMA schema", "PRAGMA schema"),
        (
            "PRAGMA full_column_names = on",
            "PRAGMA full_column_names = on",
        ),
        (
            "PRAGMA full_column_names = off",
            "PRAGMA full_column_names = off",
        ),
        ("PRAGMA cache_size = 2000", "PRAGMA cache_size = 2000"),
        ("PRAGMA foo = -2000", "PRAGMA foo = -2000"),
        ("PRAGMA foo(-2000)", "PRAGMA foo = -2000"),
        ("PRAGMA encoding = 'UTF-16'", "PRAGMA encoding = 'UTF-16'"),
        ("PRAGMA main.cache_size", "PRAGMA main.cache_size"),
        (
            "PRAGMA main.cache_size = 2000",
            "PRAGMA main.cache_size = 2000",
        ),
        ("PRAGMA cache_size(2000)", "PRAGMA cache_size = 2000"),
        (
            "PRAGMA main.cache_size(2000)",
            "PRAGMA main.cache_size = 2000",
        ),
        (
            "ATTACH DATABASE 'foo' AS schema_name",
            "ATTACH 'foo' AS schema_name",
        ),
        ("DETACH DATABASE schema_name", "DETACH schema_name"),
    ];

    for (sql, expected) in cases {
        validate_with_dialect(sql, expected, Dialect::Sqlite, Dialect::Sqlite);
    }
}

#[test]
fn test_sqlite_raw_create_table_pretty_parity() {
    let sql = r#"
            CREATE TABLE "Track"
            (
                CONSTRAINT "PK_Track" FOREIGN KEY ("TrackId"),
                FOREIGN KEY ("AlbumId") REFERENCES "Album" (
                    "AlbumId"
                ) ON DELETE NO ACTION ON UPDATE NO ACTION,
                FOREIGN KEY ("AlbumId") ON DELETE CASCADE ON UPDATE RESTRICT,
                FOREIGN KEY ("AlbumId") ON DELETE SET NULL ON UPDATE SET DEFAULT
            )
            "#;
    let expected = r#"CREATE TABLE "Track" (
  CONSTRAINT "PK_Track" FOREIGN KEY ("TrackId"),
  FOREIGN KEY ("AlbumId") REFERENCES "Album" (
    "AlbumId"
  ) ON DELETE NO ACTION ON UPDATE NO ACTION,
  FOREIGN KEY ("AlbumId") ON DELETE CASCADE ON UPDATE RESTRICT,
  FOREIGN KEY ("AlbumId") ON DELETE SET NULL ON UPDATE SET DEFAULT
)"#;

    let ast = parse(sql, Dialect::Sqlite).unwrap_or_else(|e| panic!("Parse failed: {}", e));
    assert_eq!(generate_pretty(&ast, Dialect::Sqlite), expected);
}

#[test]
fn test_pretty_raw_non_create_table_preserves_body() {
    let sql = "COPY t FROM STDIN\n    1\talpha\n    2\tbeta";
    let ast = parse(sql, Dialect::Sqlite).unwrap_or_else(|e| panic!("Parse failed: {}", e));
    assert_eq!(generate_pretty(&ast, Dialect::Sqlite), sql);
}

#[test]
fn test_sqlite_suite_identity_expression_ratchet() {
    let cases = [
        (
            "SELECT * FROM t AS t INDEXED BY s.i",
            "SELECT * FROM t AS t INDEXED BY s.i",
        ),
        (
            "SELECT * FROM t INDEXED BY s.i",
            "SELECT * FROM t INDEXED BY s.i",
        ),
        (
            "SELECT * FROM t INDEXED BY i",
            "SELECT * FROM t INDEXED BY i",
        ),
        ("SELECT * FROM t NOT INDEXED", "SELECT * FROM t NOT INDEXED"),
        (
            "SELECT a FROM t1 WHERE a NOT NULL AND a NOT NULL ORDER BY a",
            "SELECT a FROM t1 WHERE NOT a IS NULL AND NOT a IS NULL ORDER BY a",
        ),
        (
            "SELECT a, b FROM t1 WHERE b + a NOT NULL ORDER BY 1",
            "SELECT a, b FROM t1 WHERE NOT b + a IS NULL ORDER BY 1",
        ),
        (
            "SELECT rowid FROM t1 WHERE t1 MATCH 'lorem'",
            "SELECT rowid FROM t1 WHERE t1 MATCH 'lorem'",
        ),
        ("SELECT * FROM t1, t2", "SELECT * FROM t1 CROSS JOIN t2"),
        ("SELECT LIKE(y, x)", "SELECT x LIKE y"),
        ("SELECT GLOB('*y*', 'xyz')", "SELECT 'xyz' GLOB '*y*'"),
        (
            "SELECT LIKE('%y%', 'xyz', '')",
            "SELECT 'xyz' LIKE '%y%' ESCAPE ''",
        ),
        ("SELECT MIN(a, b) FROM t", "SELECT MIN(a, b) FROM t"),
        ("SELECT MAX(a, b) FROM t", "SELECT MAX(a, b) FROM t"),
        ("UNHEX(a, b)", "UNHEX(a, b)"),
        (
            "SELECT * FROM station WHERE city IS NOT ''",
            "SELECT * FROM station WHERE NOT city IS ''",
        ),
        (
            "SELECT * FROM t WHERE NULL IS y",
            "SELECT * FROM t WHERE NULL IS y",
        ),
        (
            "SELECT * FROM t WHERE NULL IS NOT y",
            "SELECT * FROM t WHERE NOT NULL IS y",
        ),
        (
            "SELECT STRFTIME('%s')",
            "SELECT STRFTIME('%s', CURRENT_TIMESTAMP)",
        ),
        ("TRUNC(3.14, 2)", "TRUNC(3.14)"),
        (
            "SELECT RANK() OVER (RANGE CURRENT ROW) FROM tbl",
            "SELECT RANK() OVER (RANGE CURRENT ROW) FROM tbl",
        ),
        (
            "ALTER TABLE t RENAME a TO b",
            "ALTER TABLE t RENAME COLUMN a TO b",
        ),
        ("SELECT * FROM t AS t(c1, c2)", "SELECT * FROM t AS t"),
        (
            "CREATE TABLE \"x\" (\"Name\" NVARCHAR(200) NOT NULL)",
            "CREATE TABLE \"x\" (\"Name\" TEXT(200) NOT NULL)",
        ),
        (
            "CREATE TABLE foo (bar LONGVARCHAR)",
            "CREATE TABLE foo (bar TEXT)",
        ),
    ];

    for (sql, expected) in cases {
        validate_with_dialect(sql, expected, Dialect::Sqlite, Dialect::Sqlite);
    }
}

#[test]
fn test_identity_limit_offset() {
    let cases = [
        "SELECT * FROM test LIMIT 100",
        "SELECT * FROM test LIMIT 100 OFFSET 200",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_mysql_comma_limit_to_sqlite() {
    validate_with_dialect(
        "SELECT a FROM t LIMIT 5, 10",
        "SELECT a FROM t LIMIT 10 OFFSET 5",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_div_to_sqlite_int_cast() {
    validate_with_dialect(
        "SELECT 7 DIV 2",
        "SELECT CAST(CAST(7 AS REAL) / 2 AS INTEGER)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_divide_to_sqlite_float_division() {
    validate_with_dialect(
        "SELECT a / b FROM t",
        "SELECT CAST(a AS REAL) / b FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT a + b / c FROM t",
        "SELECT a + CAST(b AS REAL) / c FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_div_function_to_sqlite() {
    validate_with_dialect(
        "SELECT DIV(4, 2)",
        "SELECT CAST(CAST(CAST(4 AS REAL) / 2 AS INTEGER) AS REAL)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 1 / DIV(4, 2)",
        "SELECT 1 / CAST(CAST(CAST(4 AS REAL) / 2 AS INTEGER) AS REAL)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT CAST(DIV(4, 2) AS DECIMAL(5, 3))",
        "SELECT CAST(CAST(CAST(CAST(4 AS REAL) / 2 AS INTEGER) AS REAL) AS REAL(5, 3))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_datediff_to_sqlite_julianday() {
    validate_with_dialect(
        "SELECT DATEDIFF('2020-01-03','2020-01-01')",
        "SELECT CAST((JULIANDAY('2020-01-03') - JULIANDAY('2020-01-01')) AS INTEGER)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "DATEDIFF(CAST('2010-07-07' AS DATE), CAST('2008-12-25' AS DATE))",
        "CAST((JULIANDAY(DATE('2010-07-07')) - JULIANDAY(DATE('2008-12-25'))) AS INTEGER)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_function_maps_to_sqlite() {
    let cases = [
        ("SELECT strpos('hello','l')", "SELECT INSTR('hello', 'l')"),
        ("SELECT chr(65)", "SELECT CHAR(65)"),
        ("SELECT ascii('A')", "SELECT ASCII('A')"),
        ("SELECT greatest(2,5,1)", "SELECT MAX(2, 5, 1)"),
        ("SELECT least(2,5,1)", "SELECT MIN(2, 5, 1)"),
        ("SELECT bool_and(x) FROM t", "SELECT MIN(x) FROM t"),
        ("SELECT bool_or(x) FROM t", "SELECT MAX(x) FROM t"),
        (
            "SELECT split_part('a,b,c',',',2)",
            "SELECT SPLIT_PART('a,b,c', ',', 2)",
        ),
        (
            "SELECT position('l' IN 'hello')",
            "SELECT INSTR('hello', 'l')",
        ),
        (
            "SELECT substring('hello' FROM 2 FOR 3)",
            "SELECT SUBSTRING('hello', 2, 3)",
        ),
    ];
    for (sql, expected) in cases {
        validate_with_dialect(sql, expected, Dialect::Postgres, Dialect::Sqlite);
    }
}

#[test]
fn test_mysql_date_function_maps_to_sqlite() {
    validate_with_dialect(
        "SELECT CURDATE()",
        "SELECT CURRENT_DATE",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_sqlite_datediff_unit_identity() {
    validate_with_dialect(
        "DATEDIFF(a, b, 'day')",
        "CAST((JULIANDAY(a) - JULIANDAY(b)) AS INTEGER)",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "DATEDIFF(a, b, 'hour')",
        "CAST((JULIANDAY(a) - JULIANDAY(b)) * 24.0 AS INTEGER)",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "DATEDIFF(a, b, 'year')",
        "CAST((JULIANDAY(a) - JULIANDAY(b)) / 365.0 AS INTEGER)",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "DATEDIFF(day, b, a)",
        "CAST((JULIANDAY(day) - JULIANDAY(b)) AS INTEGER)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "DATEDIFF(a, b, c)",
        "CAST((JULIANDAY(a) - JULIANDAY(b)) AS INTEGER)",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_typed_literals_to_sqlite() {
    validate_with_dialect(
        "SELECT DATE '2020-01-01'",
        "SELECT DATE('2020-01-01')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TIME '12:34:56'",
        "SELECT CAST('12:34:56' AS TIME)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TIMESTAMP '2020-01-01 12:00:00'",
        "SELECT CAST('2020-01-01 12:00:00' AS TIMESTAMP)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_extract_from_date_literal_to_sqlite() {
    validate_with_dialect(
        "SELECT EXTRACT(YEAR FROM DATE '2020-06-15')",
        "SELECT EXTRACT(YEAR FROM DATE('2020-06-15'))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_time_functions_to_sqlite() {
    validate_with_dialect(
        "DATE_TRUNC('day', x)",
        "TIMESTAMP_TRUNC(x, DAY)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "DATE_TRUNC('day', x::DATE)",
        "TIMESTAMP_TRUNC(DATE(x), DAY)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT DATE_PART('minute', timestamp '2023-01-04 04:05:06.789')",
        "SELECT EXTRACT(minute FROM CAST('2023-01-04 04:05:06.789' AS TIMESTAMP))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT DATE_PART('isodow'::varchar(6), current_date)",
        "SELECT EXTRACT(CAST('isodow' AS TEXT(6)) FROM CURRENT_DATE)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT EXTRACT(QUARTER FROM CAST('2025-04-26' AS DATE))",
        "SELECT EXTRACT(QUARTER FROM DATE('2025-04-26'))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "DATE_TRUNC('day', x)",
        "DATE_TRUNC('DAY', x)",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_limit_all_to_sqlite() {
    validate_with_dialect(
        "SELECT x FROM t LIMIT ALL",
        "SELECT x FROM t LIMIT ALL",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_offset_without_limit_to_sqlite() {
    validate_with_dialect(
        "SELECT x FROM t OFFSET 1",
        "SELECT x FROM t OFFSET 1",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_for_update_to_sqlite() {
    validate_with_dialect(
        "SELECT a FROM t FOR UPDATE",
        "SELECT a FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT a FROM t FOR UPDATE",
        "SELECT a FROM t FOR UPDATE",
        Dialect::Postgres,
        Dialect::Postgres,
    );
}

#[test]
fn test_postgres_distinct_on_to_sqlite_row_number() {
    validate_with_dialect(
        "SELECT DISTINCT ON (a) a, b FROM t",
        "SELECT a, b FROM (SELECT a AS a, b AS b, ROW_NUMBER() OVER (PARTITION BY a ORDER BY a) AS _row_number FROM t) AS _t WHERE _row_number = 1",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT DISTINCT ON (a) a, b FROM t ORDER BY b",
        "SELECT a, b FROM (SELECT a AS a, b AS b, ROW_NUMBER() OVER (PARTITION BY a ORDER BY b NULLS LAST) AS _row_number FROM t) AS _t WHERE _row_number = 1",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT DISTINCT ON (LOWER(a)) LOWER(a), b FROM t",
        "SELECT _col, b FROM (SELECT LOWER(a) AS _col, b AS b, ROW_NUMBER() OVER (PARTITION BY LOWER(a) ORDER BY LOWER(a)) AS _row_number FROM t) AS _t WHERE _row_number = 1",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT DISTINCT ON (a) * FROM t",
        "SELECT * FROM (SELECT *, ROW_NUMBER() OVER (PARTITION BY a ORDER BY a) AS _row_number FROM t) AS _t WHERE _row_number = 1",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – Subqueries
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_subqueries() {
    let cases = [
        "SELECT a FROM (SELECT a FROM test) AS x",
        "SELECT * FROM (SELECT 1 AS x) AS sub",
        "SELECT a FROM test WHERE a IN (SELECT b FROM z)",
        "SELECT a FROM test WHERE EXISTS (SELECT 1)",
        "SELECT * FROM t WHERE id IN (SELECT id FROM t2)",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_nested_subquery() {
    validate_identity("SELECT a FROM (SELECT a FROM (SELECT a FROM test) AS y) AS x");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – CASE expression
// (from Python identity.sql: CASE section)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_case() {
    let cases = [
        "SELECT CASE WHEN a > 1 THEN 1 ELSE 0 END",
        "SELECT CASE WHEN a < b THEN 1 WHEN a < c THEN 2 ELSE 3 END FROM test",
        "SELECT CASE 1 WHEN 1 THEN 1 ELSE 2 END",
        "SELECT CASE a WHEN 1 THEN 'one' WHEN 2 THEN 'two' ELSE 'other' END",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – BETWEEN, IN, IS NULL, LIKE, ILIKE
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_predicates() {
    let cases = [
        "SELECT * FROM t WHERE x BETWEEN 1 AND 10",
        "SELECT * FROM t WHERE x NOT BETWEEN 1 AND 10",
        "SELECT * FROM t WHERE x IN (1, 2, 3)",
        "SELECT * FROM t WHERE x NOT IN (1, 2, 3)",
        "SELECT * FROM t WHERE x IS NULL",
        "SELECT * FROM t WHERE x IS NOT NULL",
        "SELECT * FROM t WHERE x IS TRUE",
        "SELECT * FROM t WHERE x IS NOT TRUE",
        "SELECT * FROM t WHERE x IS FALSE",
        "SELECT * FROM t WHERE x IS NOT FALSE",
        "SELECT * FROM t WHERE x IS TRUE AND y IS NULL",
        "SELECT * FROM t WHERE x IS NOT FALSE OR y IS NOT NULL",
        "SELECT * FROM t WHERE x LIKE '%y%'",
        "SELECT * FROM t WHERE x NOT LIKE '%y%'",
        "SELECT * FROM t WHERE x ILIKE '%y%'",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_in_subquery() {
    validate_identity("SELECT * FROM t WHERE a IN (SELECT b FROM t2)");
    validate_identity("SELECT * FROM t WHERE a NOT IN (SELECT b FROM t2)");
}

#[test]
fn test_identity_exists() {
    validate_identity("SELECT * FROM t WHERE EXISTS (SELECT 1 FROM t2)");
    validate_identity("SELECT * FROM t WHERE NOT EXISTS (SELECT 1 FROM t2)");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – CAST, EXTRACT, functions
// (from Python identity.sql: CAST, EXTRACT, function sections)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_cast() {
    let cases = [
        "SELECT CAST(a AS INT) FROM test",
        "SELECT CAST(a AS VARCHAR) FROM test",
        "SELECT CAST(a AS DECIMAL(5, 3)) FROM test",
        "SELECT CAST(a AS TIMESTAMP) FROM test",
        "SELECT CAST(a AS DATE) FROM test",
        "SELECT CAST(a AS BOOLEAN) FROM test",
        "SELECT CAST(a AS TEXT) FROM test",
        "SELECT CAST(a AS BIGINT) FROM test",
        "SELECT CAST(a AS FLOAT) FROM test",
        "SELECT CAST(a AS DOUBLE) FROM test",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_extract() {
    let cases = [
        "SELECT EXTRACT(YEAR FROM x)",
        "SELECT EXTRACT(MONTH FROM x)",
        "SELECT EXTRACT(DAY FROM x)",
        "SELECT EXTRACT(HOUR FROM x)",
        "SELECT EXTRACT(MINUTE FROM x)",
        "SELECT EXTRACT(SECOND FROM x)",
        "SELECT EXTRACT(DOW FROM x)",
        "SELECT EXTRACT(EPOCH FROM x)",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_functions() {
    let cases = [
        "SELECT ABS(a) FROM test",
        "SELECT COUNT(*) FROM test",
        "SELECT COUNT(a) FROM test",
        "SELECT COUNT(DISTINCT a) FROM test",
        "SELECT SUM(a) FROM test",
        "SELECT AVG(a) FROM test",
        "SELECT MIN(a) FROM test",
        "SELECT MAX(a) FROM test",
        "SELECT ROUND(a) FROM test",
        "SELECT ROUND(a, 2) FROM test",
        "SELECT COALESCE(a, b, c) FROM test",
        "SELECT NULLIF(a, b) FROM test",
        "SELECT GREATEST(a, b, c) FROM test",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – Window functions
// (from Python identity.sql: Window section)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_window_functions() {
    let cases = [
        "SELECT RANK() OVER () FROM x",
        "SELECT RANK() OVER () AS y FROM x",
        "SELECT RANK() OVER (PARTITION BY a) FROM x",
        "SELECT RANK() OVER (PARTITION BY a, b) FROM x",
        "SELECT RANK() OVER (ORDER BY a) FROM x",
        "SELECT RANK() OVER (ORDER BY a, b) FROM x",
        "SELECT RANK() OVER (PARTITION BY a ORDER BY a) FROM x",
        "SELECT RANK() OVER (PARTITION BY a, b ORDER BY a, b DESC) FROM x",
        "SELECT SUM(x) OVER (PARTITION BY a) AS y FROM x",
        "SELECT ROW_NUMBER() OVER (PARTITION BY dept ORDER BY salary DESC) FROM emp",
        "SELECT LAG(x) OVER (ORDER BY y) AS x",
        "SELECT LEAD(a) OVER (ORDER BY b) AS a",
        "SELECT LEAD(a, 1) OVER (PARTITION BY a ORDER BY a) AS x",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_window_frames() {
    let cases = [
        "SELECT SUM(x) OVER (PARTITION BY a ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW)",
        "SELECT SUM(x) OVER (PARTITION BY a ORDER BY b ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW)",
        "SELECT SUM(x) OVER (PARTITION BY a ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING)",
        "SELECT SUM(x) OVER (PARTITION BY a ROWS BETWEEN CURRENT ROW AND UNBOUNDED FOLLOWING)",
        "SELECT SUM(x) OVER (PARTITION BY a RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW)",
        "SELECT SUM(x) OVER (PARTITION BY a ORDER BY b ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING)",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_window_filter() {
    validate_identity("SELECT SUM(x) FILTER (WHERE x > 1)");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – Set Operations (UNION, INTERSECT, EXCEPT)
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_set_operations() {
    let cases = [
        "SELECT 1 UNION ALL SELECT 2",
        "SELECT 1 UNION SELECT 2",
        "SELECT 1 INTERSECT SELECT 2",
        "SELECT 1 EXCEPT SELECT 2",
        "SELECT a FROM t1 UNION ALL SELECT b FROM t2",
        "SELECT a FROM t1 INTERSECT SELECT a FROM t2",
        "SELECT a FROM t1 EXCEPT SELECT a FROM t2",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – CTEs (WITH clause)
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_ctes() {
    let cases = [
        "WITH a AS (SELECT 1) SELECT * FROM a",
        "WITH a AS (SELECT 1 AS x) SELECT x FROM a",
        "WITH a AS (SELECT 1), b AS (SELECT 2) SELECT * FROM a CROSS JOIN b",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_recursive_cte() {
    validate_identity("WITH RECURSIVE nums AS (SELECT 1 AS n) SELECT n FROM nums");
}

#[test]
fn test_identity_cte_with_columns() {
    validate_identity("WITH cte(x, y) AS (SELECT 1, 2) SELECT x, y FROM cte");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – INSERT
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_insert() {
    let cases = [
        "INSERT INTO x VALUES (1, 'a', 2.0)",
        "INSERT INTO x VALUES (1, 'a', 2.0), (2, 'b', 3.0)",
        "INSERT INTO y (a, b, c) SELECT a, b, c FROM x",
        "INSERT INTO x SELECT * FROM y",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_mysql_replace_into_to_sqlite() {
    validate_with_dialect(
        "REPLACE INTO t (id, a) VALUES (1, 2)",
        "REPLACE INTO t (id, a) VALUES (1, 2)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "REPLACE INTO t (id,a) VALUES (1,2)",
        "REPLACE INTO t (id,a) VALUES (1,2)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_identity_insert_on_conflict() {
    validate_identity("INSERT INTO t (id) VALUES (1) ON CONFLICT (id) DO NOTHING");
    validate_identity(
        "INSERT INTO t (id, name) VALUES (1, 'a') ON CONFLICT (id) DO UPDATE SET name = 'b'",
    );
}

#[test]
fn test_cinch_holefinder_postgres_ddl_join_cte_batch_to_sqlite() {
    validate_with_dialect(
        "SELECT count(*) FROM jl NATURAL JOIN jr",
        "SELECT COUNT(*) FROM jl NATURAL JOIN jr",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE rcq (id INT PRIMARY KEY, \"select\" INT)",
        "CREATE TABLE rcq (id INTEGER PRIMARY KEY, \"select\" INTEGER)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "WITH a AS (SELECT 1 AS x), b AS (SELECT 2 AS y) SELECT a.x + b.y FROM a, b",
        "WITH a AS (SELECT 1 AS x), b AS (SELECT 2 AS y) SELECT a.x + b.y FROM a, b",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE t (id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY)",
        "CREATE TABLE t (id INTEGER PRIMARY KEY AUTOINCREMENT)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE t (id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY)",
        "CREATE TABLE t (id INTEGER PRIMARY KEY AUTOINCREMENT)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE z (a INT GENERATED BY DEFAULT AS IDENTITY NOT NULL UNIQUE PRIMARY KEY)",
        "CREATE TABLE z (a INTEGER UNIQUE PRIMARY KEY AUTOINCREMENT)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE t (created_at TIMESTAMP DEFAULT now())",
        "CREATE TABLE t (created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE INDEX CONCURRENTLY idx ON t (c)",
        "CREATE INDEX CONCURRENTLY idx ON t(c NULLS LAST)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE INDEX idx ON t USING btree (c)",
        "CREATE INDEX idx ON t USING btree(c NULLS LAST)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE INDEX idx ON t USING gin (data)",
        "CREATE INDEX idx ON t USING gin(data NULLS LAST)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE t (id BIGSERIAL PRIMARY KEY)",
        "CREATE TABLE t (id BIGSERIAL PRIMARY KEY)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE t (id UUID DEFAULT gen_random_uuid())",
        "CREATE TABLE t (id UUID DEFAULT UUID())",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_on_conflict_to_sqlite_spacing() {
    validate_with_dialect(
        "INSERT INTO t (id, a) VALUES (1, 2) ON CONFLICT (id) DO UPDATE SET a = excluded.a",
        "INSERT INTO t (id, a) VALUES (1, 2) ON CONFLICT(id) DO UPDATE SET a = excluded.a",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_identity_insert_returning() {
    validate_identity("INSERT INTO users (name) VALUES ('Alice') RETURNING id");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – UPDATE
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_update() {
    let cases = [
        "UPDATE tbl_name SET foo = 123",
        "UPDATE tbl_name SET foo = 123, bar = 345",
        "UPDATE db.tbl_name SET foo = 123 WHERE tbl_name.bar = 234",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_update_returning() {
    validate_identity("UPDATE products SET price = 10 WHERE id = 1 RETURNING name, price");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – DELETE
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_delete() {
    let cases = ["DELETE FROM x WHERE y > 1", "DELETE FROM y"];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_delete_using() {
    validate_identity("DELETE FROM event USING sales WHERE event.eventid = sales.eventid");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – DDL: CREATE TABLE
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_create_table() {
    let cases = [
        "CREATE TABLE z (a INT, b VARCHAR, c VARCHAR(100), d DECIMAL(5, 3))",
        "CREATE TABLE IF NOT EXISTS x AS SELECT a FROM d",
        "CREATE TEMPORARY TABLE x AS SELECT a FROM d",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_create_table_constraints() {
    let cases = [
        "CREATE TABLE z (a INT, PRIMARY KEY (a))",
        "CREATE TABLE z (a INT NOT NULL)",
        // Generator outputs NOT NULL before DEFAULT
        "CREATE TABLE z (a INT NOT NULL DEFAULT 0)",
        "CREATE TABLE z (a INT UNIQUE)",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_create_table_constraint_ordering() {
    // DEFAULT 0 NOT NULL gets normalized to NOT NULL DEFAULT 0
    validate(
        "CREATE TABLE z (a INT DEFAULT 0 NOT NULL)",
        "CREATE TABLE z (a INT NOT NULL DEFAULT 0)",
    );
}

#[test]
fn test_mysql_create_table_options_to_sqlite() {
    validate_with_dialect(
        "CREATE TABLE z (a INT) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARACTER SET=utf8 COLLATE=utf8_bin COMMENT='x'",
        "CREATE TABLE z (a INTEGER)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_create_table_options_roundtrip() {
    validate_with_dialect(
        "CREATE TABLE z (a INT) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARACTER SET=utf8 COLLATE=utf8_bin COMMENT='x'",
        "CREATE TABLE z (a INT) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARACTER SET=utf8 COLLATE=utf8_bin COMMENT='x'",
        Dialect::Mysql,
        Dialect::Mysql,
    );
}

#[test]
fn test_mysql_create_table_column_auto_increment_roundtrip() {
    validate_with_dialect(
        "CREATE TABLE x (id INT AUTO_INCREMENT, PRIMARY KEY (id))",
        "CREATE TABLE x (id INT AUTO_INCREMENT, PRIMARY KEY (id))",
        Dialect::Mysql,
        Dialect::Mysql,
    );
}

#[test]
fn test_mysql_create_table_options_ast() {
    let ast = parse(
        "CREATE TABLE z (a INT) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARACTER SET=utf8 COLLATE=utf8_bin COMMENT='x'",
        Dialect::Mysql,
    )
    .unwrap();

    let Statement::CreateTable(ct) = ast else {
        panic!("expected CREATE TABLE");
    };

    assert_eq!(
        ct.options,
        vec![
            CreateTableOption::Engine("InnoDB".to_string()),
            CreateTableOption::AutoIncrement("1".to_string()),
            CreateTableOption::CharacterSet {
                default: true,
                value: "utf8".to_string()
            },
            CreateTableOption::Collate {
                default: false,
                value: "utf8_bin".to_string()
            },
            CreateTableOption::Comment("x".to_string()),
        ]
    );
}

#[test]
fn test_mysql_create_table_column_options_to_sqlite() {
    validate_with_dialect(
        "CREATE TABLE z (id INT AUTO_INCREMENT PRIMARY KEY, name VARCHAR(255) COLLATE utf8_bin COMMENT 'n') ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
        "CREATE TABLE z (id INTEGER PRIMARY KEY, name TEXT(255) COLLATE utf8_bin COMMENT 'n')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_create_table_primary_key_auto_increment_order_to_sqlite() {
    validate_with_dialect(
        "CREATE TABLE z (id INT PRIMARY KEY AUTO_INCREMENT)",
        "CREATE TABLE z (id INTEGER PRIMARY KEY AUTOINCREMENT)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_create_table_table_primary_key_auto_increment_to_sqlite() {
    validate_with_dialect(
        "CREATE TABLE x (id INT NOT NULL AUTO_INCREMENT, PRIMARY KEY (id))",
        "CREATE TABLE x (id INTEGER NOT NULL AUTOINCREMENT PRIMARY KEY)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_create_table_type_affinity_to_sqlite() {
    validate_with_dialect(
        "CREATE TABLE z (a TINYINT, b SMALLINT, c INT, d BIGINT, e VARCHAR(10), f DATETIME, g BOOLEAN, h FLOAT, i DOUBLE, j DECIMAL(5, 2), k BINARY(4), l VARBINARY(8), m JSON)",
        "CREATE TABLE z (a INTEGER, b INTEGER, c INTEGER, d INTEGER, e TEXT(10), f DATETIME, g INTEGER, h REAL, i REAL, j REAL(5, 2), k BLOB(4), l BLOB(8), m JSON)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_create_table_constraints_to_sqlite() {
    let cases = [
        (
            "CREATE TABLE x (a INT DEFAULT 1, CONSTRAINT ck CHECK (a > 0))",
            "CREATE TABLE x (a INTEGER DEFAULT 1, CONSTRAINT ck CHECK (a > 0))",
        ),
        (
            "CREATE TABLE x (a INT, CONSTRAINT uq UNIQUE (a))",
            "CREATE TABLE x (a INTEGER, CONSTRAINT uq UNIQUE (a))",
        ),
        (
            "CREATE TABLE x (a INT, CONSTRAINT fk FOREIGN KEY (a) REFERENCES y (id) ON DELETE CASCADE ON UPDATE SET NULL)",
            "CREATE TABLE x (a INTEGER, CONSTRAINT fk FOREIGN KEY (a) REFERENCES y (id) ON DELETE CASCADE ON UPDATE SET NULL)",
        ),
    ];

    for (source, expected) in cases {
        validate_with_dialect(source, expected, Dialect::Mysql, Dialect::Sqlite);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – DDL: DROP TABLE, CREATE/DROP VIEW
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_drop_table() {
    let cases = [
        "DROP TABLE a",
        "DROP TABLE IF EXISTS a",
        "DROP TABLE a CASCADE",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_identity_views() {
    let cases = [
        "CREATE VIEW x AS SELECT a FROM b",
        "CREATE VIEW IF NOT EXISTS x AS SELECT a FROM b",
        "CREATE OR REPLACE VIEW x AS SELECT *",
        "DROP VIEW a",
        "DROP VIEW IF EXISTS a",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_create_view_sql_security_to_sqlite() {
    let cases = [
        "CREATE VIEW v SQL SECURITY INVOKER AS SELECT 1",
        "CREATE VIEW v SECURITY INVOKER AS SELECT 1",
        "CREATE SQL SECURITY INVOKER VIEW v AS SELECT 1",
        "CREATE VIEW v SQL SECURITY DEFINER AS SELECT 1",
        "CREATE VIEW v SECURITY DEFINER AS SELECT 1",
    ];
    for sql in cases {
        validate_with_dialect(
            sql,
            "CREATE VIEW v AS SELECT 1",
            Dialect::Postgres,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            sql,
            "CREATE VIEW v AS SELECT 1",
            Dialect::Mysql,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            sql,
            "CREATE VIEW v AS SELECT 1",
            Dialect::Sqlite,
            Dialect::Sqlite,
        );
    }
}

#[test]
fn test_create_index_to_sqlite() {
    validate_with_dialect(
        "CREATE INDEX idx ON x (a)",
        "CREATE INDEX idx ON x(a)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_create_index_expressions_to_postgres() {
    let cases = [
        (
            "CREATE INDEX idx ON x (a DESC)",
            "CREATE INDEX idx ON x(a DESC)",
        ),
        (
            "CREATE INDEX idx ON x (LOWER(a))",
            "CREATE INDEX idx ON x(LOWER(a))",
        ),
        (
            "CREATE INDEX idx ON x USING BTREE (a)",
            "CREATE INDEX idx ON x USING BTREE(a)",
        ),
    ];

    for (source, expected) in cases {
        validate_with_dialect(source, expected, Dialect::Postgres, Dialect::Postgres);
    }
}

#[test]
fn test_create_partial_index() {
    // SQLGlot accepts a `WHERE` predicate on CREATE INDEX regardless of read
    // dialect and renders it for SQLite/Postgres (partial indexes).
    let cases = [
        (Dialect::Mysql, Dialect::Sqlite),
        (Dialect::Postgres, Dialect::Postgres),
        (Dialect::Sqlite, Dialect::Sqlite),
    ];
    for (read, write) in cases {
        validate_with_dialect(
            "CREATE INDEX idx ON x (a) WHERE a > 0",
            "CREATE INDEX idx ON x(a) WHERE a > 0",
            read,
            write,
        );
    }
}

#[test]
fn test_create_unique_index_to_sqlite() {
    validate_with_dialect(
        "CREATE UNIQUE INDEX idx ON x (a, b)",
        "CREATE UNIQUE INDEX idx ON x(a, b)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_drop_index_to_sqlite() {
    validate_with_dialect(
        "DROP INDEX idx ON x",
        "DROP INDEX idx ON x",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_index_identity() {
    let cases = [
        "CREATE INDEX IF NOT EXISTS ON t(c)",
        "CREATE INDEX CONCURRENTLY IF NOT EXISTS ix_table_id ON tbl USING btree(id)",
        "DROP INDEX IF EXISTS ix_table_id",
        "DROP INDEX CONCURRENTLY IF EXISTS ix_table_id",
    ];
    for sql in &cases {
        validate_with_dialect(sql, sql, Dialect::Postgres, Dialect::Postgres);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – ALTER TABLE
// (from Python identity.sql: ALTER TABLE section)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_alter_table() {
    let cases = [
        "ALTER TABLE integers ADD COLUMN k INT",
        "ALTER TABLE integers DROP COLUMN k",
        "ALTER TABLE integers DROP COLUMN IF EXISTS k",
        "ALTER TABLE table1 RENAME COLUMN c1 TO c2",
        "ALTER TABLE table1 RENAME TO table2",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_alter_table_constraints_to_sqlite() {
    let cases = [
        "ALTER TABLE x ADD CONSTRAINT ck CHECK (a > 0)",
        "ALTER TABLE x ADD CONSTRAINT uq UNIQUE (a)",
        "ALTER TABLE x ADD CONSTRAINT fk FOREIGN KEY (a) REFERENCES y (id) ON DELETE CASCADE",
    ];

    for sql in cases {
        validate_with_dialect(sql, sql, Dialect::Mysql, Dialect::Sqlite);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – Transaction statements
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_transactions() {
    let cases = ["BEGIN", "COMMIT", "ROLLBACK"];
    for sql in &cases {
        validate_identity(sql);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – EXPLAIN, USE
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_explain_use() {
    validate_identity("EXPLAIN SELECT * FROM x");
    validate_identity("USE db");
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – INTERVAL
// (from Python identity.sql: INTERVAL section)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_interval() {
    let cases = [
        "SELECT INTERVAL '1' DAY",
        "SELECT INTERVAL '1' MONTH",
        "SELECT INTERVAL '1' YEAR",
        "SELECT INTERVAL '1' HOUR",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – ARRAY and complex expressions
// (from Python identity.sql)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_array() {
    // ARRAY[1, 2, 3] using bracket syntax
    validate_identity("SELECT ARRAY[1, 2, 3]");
}

#[test]
fn test_postgres_array_literal_to_sqlite() {
    validate_with_dialect(
        "SELECT ARRAY[1, 2, 3]",
        "SELECT ARRAY(1, 2, 3)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT ARRAY[ARRAY[1, 2], ARRAY[3, 4]]",
        "SELECT ARRAY(ARRAY(1, 2), ARRAY(3, 4))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT ARRAY[a, b] FROM t",
        "SELECT ARRAY(a, b) FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Identity tests – Postgres-style cast (::)
// (from Python test_transpile.py::test_types)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_postgres_cast_roundtrip() {
    // x::INT parses as CAST(x AS INT) when in a SELECT context
    validate("SELECT x::INT", "SELECT CAST(x AS INT)");
    validate(
        "SELECT x::INT::BOOLEAN",
        "SELECT CAST(CAST(x AS INT) AS BOOLEAN)",
    );
    validate(
        "SELECT CAST(x::INT AS BOOLEAN)",
        "SELECT CAST(CAST(x AS INT) AS BOOLEAN)",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Spacing normalization tests
// (from Python test_transpile.py::test_space)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_space_normalization() {
    // Operators get spaces around them
    validate("SELECT 1>0", "SELECT 1 > 0");
    validate("SELECT 1>=0", "SELECT 1 >= 0");
    validate("SELECT 1<0", "SELECT 1 < 0");
    validate("SELECT 1<=0", "SELECT 1 <= 0");
}

// ═════════════════════════════════════════════════════════════════════════════
// Transpile – cross-dialect tests
// (from Python test_transpile.py and dialect test files)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_transpile_identity_same_dialect() {
    let sql = "SELECT a, b FROM t WHERE a > 1";
    for dialect in [
        Dialect::Ansi,
        Dialect::Postgres,
        Dialect::Mysql,
        Dialect::Sqlite,
        Dialect::BigQuery,
        Dialect::Snowflake,
        Dialect::DuckDb,
    ] {
        validate_with_dialect(sql, sql, dialect, dialect);
    }
}

#[test]
fn test_transpile_substr_to_substring() {
    // SUBSTR → SUBSTRING when targeting ANSI/Postgres
    validate_with_dialect(
        "SELECT SUBSTR(name, 1, 3) FROM users",
        "SELECT SUBSTRING(name, 1, 3) FROM users",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_transpile_substring_to_substr() {
    // SUBSTRING → SUBSTR when targeting MySQL
    validate_with_dialect(
        "SELECT SUBSTRING(name, 1, 3) FROM users",
        "SELECT SUBSTR(name, 1, 3) FROM users",
        Dialect::Postgres,
        Dialect::Mysql,
    );
    validate_with_dialect(
        "SELECT SUBSTRING(name, 1, 3) FROM users",
        "SELECT SUBSTRING(name, 1, 3) FROM users",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_transpile_now_to_current_timestamp() {
    // NOW() → CURRENT_TIMESTAMP for BigQuery/Snowflake
    validate_with_dialect(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
    validate_with_dialect(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP()",
        Dialect::Postgres,
        Dialect::Snowflake,
    );
    validate_with_dialect(
        "SELECT NOW()",
        "SELECT CURRENT_TIMESTAMP",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT NOW()",
        "SELECT NOW()",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_transpile_len_to_length() {
    // LEN → LENGTH for Postgres, MySQL, SQLite, DuckDB
    validate_with_dialect(
        "SELECT LEN(name) FROM t",
        "SELECT LENGTH(name) FROM t",
        Dialect::BigQuery,
        Dialect::Postgres,
    );
    validate_with_dialect(
        "SELECT LEN(name) FROM t",
        "SELECT LENGTH(name) FROM t",
        Dialect::BigQuery,
        Dialect::Mysql,
    );
}

#[test]
fn test_transpile_ifnull_to_coalesce() {
    // IFNULL → COALESCE for ANSI/Postgres
    validate_with_dialect(
        "SELECT IFNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Mysql,
        Dialect::Postgres,
    );
    validate_with_dialect(
        "SELECT IFNULL(a, b) FROM t",
        "SELECT COALESCE(a, b) FROM t",
        Dialect::Mysql,
        Dialect::Ansi,
    );
    validate_with_dialect(
        "SELECT IFNULL(a, 0) FROM t",
        "SELECT COALESCE(a, 0) FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_bit_aggregates_to_sqlite() {
    for read in [Dialect::Mysql, Dialect::Postgres] {
        validate_with_dialect("BIT_AND(x)", "BITWISE_AND_AGG(x)", read, Dialect::Sqlite);
        validate_with_dialect("BIT_OR(x)", "BITWISE_OR_AGG(x)", read, Dialect::Sqlite);
        validate_with_dialect("BIT_XOR(x)", "BITWISE_XOR_AGG(x)", read, Dialect::Sqlite);
    }
    validate_with_dialect(
        "BIT_COUNT(x)",
        "BITWISE_COUNT(x)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "BIT_COUNT(x)",
        "BIT_COUNT(x)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect("BIT_AND(x)", "BIT_AND(x)", Dialect::Sqlite, Dialect::Sqlite);
}

#[test]
fn test_mysql_on_duplicate_key_to_sqlite() {
    validate_with_dialect(
        "INSERT INTO t (id, a) VALUES (1, 2) ON DUPLICATE KEY UPDATE a = 2",
        "INSERT INTO t (id, a) VALUES (1, 2) ON DUPLICATE KEY UPDATE SET a = 2",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_insert_ignore_to_sqlite() {
    validate_with_dialect(
        "INSERT IGNORE INTO t (id, a) VALUES (1, 2)",
        "INSERT IGNORE INTO t (id, a) VALUES (1, 2)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_opaque_command_parse_carriers() {
    validate_with_dialect(
        "SET @var_name = expr",
        "SET @var_name = expr",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "ANALYZE LOCAL TABLE tbl",
        "ANALYZE LOCAL TABLE tbl",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "REVOKE SELECT ON TABLE users FROM role1",
        "REVOKE SELECT ON TABLE users FROM role1",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE VIRTUAL TABLE docs USING fts5(title, content)",
        "CREATE VIRTUAL TABLE docs USING fts5(title, content)",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "ALTER TABLE t1 SET LOGGED",
        "ALTER TABLE t1 SET LOGGED",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CREATE TABLE foo (id INTEGER PRIMARY KEY ASC)",
        "CREATE TABLE foo (id INTEGER PRIMARY KEY ASC)",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
}

#[test]
fn test_sqlite_insert_or_conflict_carrier() {
    validate_with_dialect(
        "INSERT OR ABORT INTO foo (x, y) VALUES (1, 2)",
        "INSERT OR ABORT INTO foo (x, y) VALUES (1, 2)",
        Dialect::Sqlite,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_if_to_sqlite_iif() {
    validate_with_dialect(
        "SELECT IF(a > 0, 'y', 'n') FROM t",
        "SELECT IIF(a > 0, 'y', 'n') FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_transpile_ilike_to_like_lower() {
    // ILIKE → LOWER(x) LIKE LOWER(pattern) for MySQL/SQLite
    validate_with_dialect(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')",
        Dialect::Postgres,
        Dialect::Mysql,
    );
    validate_with_dialect(
        "SELECT * FROM t WHERE name ILIKE '%test%'",
        "SELECT * FROM t WHERE LOWER(name) LIKE LOWER('%test%')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_transpile_type_mapping_text_to_string() {
    // TEXT → STRING for BigQuery
    validate_with_dialect(
        "SELECT CAST(x AS TEXT) FROM t",
        "SELECT CAST(x AS STRING) FROM t",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

#[test]
fn test_transpile_type_mapping_string_to_text() {
    // STRING → TEXT for Postgres, MySQL, SQLite
    validate_with_dialect(
        "SELECT CAST(x AS STRING) FROM t",
        "SELECT x::TEXT FROM t",
        Dialect::BigQuery,
        Dialect::Postgres,
    );
}

#[test]
fn test_transpile_type_mapping_int_to_bigint() {
    // INT → BIGINT for BigQuery
    validate_with_dialect(
        "SELECT CAST(x AS INT) FROM t",
        "SELECT CAST(x AS BIGINT) FROM t",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

#[test]
fn test_mysql_signed_cast_to_sqlite_integer() {
    validate_with_dialect(
        "SELECT CAST(a AS SIGNED) FROM t",
        "SELECT CAST(a AS INTEGER) FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT CAST(a AS SIGNED INTEGER) FROM t",
        "SELECT CAST(a AS INTEGER) FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT IF(a > 0, CAST(a AS SIGNED INTEGER), 7 DIV 2), x / y FROM metrics",
        "SELECT IIF(a > 0, CAST(a AS INTEGER), CAST(CAST(7 AS REAL) / 2 AS INTEGER)), CAST(x AS REAL) / y FROM metrics",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_sqlite_function_operator_mapping_batch() {
    validate_with_dialect(
        "POSITION(needle in haystack)",
        "INSTR(haystack, needle)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "LOCATE(needle, haystack)",
        "INSTR(haystack, needle)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "LOCATE(needle, haystack, position)",
        "IIF(INSTR(SUBSTRING(haystack, position), needle) = 0, 0, INSTR(SUBSTRING(haystack, position), needle) + position - 1)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    for source in [Dialect::Sqlite, Dialect::Mysql, Dialect::Postgres] {
        validate_with_dialect(
            "SELECT STR_POSITION(haystack, needle)",
            "SELECT INSTR(haystack, needle)",
            source,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            "SELECT STR_POSITION(haystack, needle, position)",
            "SELECT IIF(INSTR(SUBSTRING(haystack, position), needle) = 0, 0, INSTR(SUBSTRING(haystack, position), needle) + position - 1)",
            source,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            "SELECT LOCATE(needle, haystack)",
            "SELECT INSTR(haystack, needle)",
            source,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            "SELECT NVL2(a, b, c)",
            "SELECT CASE WHEN NOT a IS NULL THEN b ELSE c END",
            source,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            "SELECT NVL2(a, b)",
            "SELECT CASE WHEN NOT a IS NULL THEN b END",
            source,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            "SELECT DECODE(a, 1, 'one', 2, 'two', 'other')",
            "SELECT CASE WHEN a = 1 THEN 'one' WHEN a = 2 THEN 'two' ELSE 'other' END",
            source,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            "SELECT DECODE(a, NULL, 'nil', 'other')",
            "SELECT CASE WHEN a IS NULL THEN 'nil' ELSE 'other' END",
            source,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            "SELECT DECODE(a, b, c, d + 1, e)",
            "SELECT CASE WHEN a = b OR (a IS NULL AND b IS NULL) THEN c WHEN a = (d + 1) OR (a IS NULL AND (d + 1) IS NULL) THEN e END",
            source,
            Dialect::Sqlite,
        );
        validate_with_dialect(
            "SELECT DECODE(a, F(b), c, TRUE, d, -1, e)",
            "SELECT CASE WHEN a = F(b) OR (a IS NULL AND F(b) IS NULL) THEN c WHEN a = TRUE OR (a IS NULL AND TRUE IS NULL) THEN d WHEN a = -1 OR (a IS NULL AND -1 IS NULL) THEN e END",
            source,
            Dialect::Sqlite,
        );
    }
    validate_with_dialect("CONCAT(a)", "a", Dialect::Postgres, Dialect::Sqlite);
    validate_with_dialect(
        "CONCAT(a, b, c)",
        "a || b || c",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CURRENT_SCHEMA()",
        "'main'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect("SCHEMA()", "'main'", Dialect::Mysql, Dialect::Sqlite);
    validate_with_dialect(
        "SELECT LOG2(a), LOG10(b), LOG(c) FROM t",
        "SELECT LOG(2, a), LOG(10, b), LN(c) FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT JSON_AGG(name), JSON_OBJECT_AGG(name, value) FROM t",
        "SELECT JSON_GROUP_ARRAY(name), JSON_GROUP_OBJECT(name, value) FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect("x IS UNKNOWN", "x IS NULL", Dialect::Mysql, Dialect::Sqlite);
    validate_with_dialect(
        "x IS NOT UNKNOWN",
        "NOT x IS NULL",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect("a XOR b", "a XOR b", Dialect::Mysql, Dialect::Sqlite);
}

#[test]
fn test_mysql_unsigned_cast_to_sqlite_ubigint() {
    validate_with_dialect(
        "SELECT CAST(a AS UNSIGNED) FROM t",
        "SELECT CAST(a AS UBIGINT) FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT CAST(a AS UNSIGNED INTEGER) FROM t",
        "SELECT CAST(a AS UBIGINT) FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_signed_cast_to_sqlite_preserves_unknown_type() {
    validate_with_dialect(
        "SELECT CAST(a AS SIGNED) FROM t",
        "SELECT CAST(a AS SIGNED) FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_transpile_type_mapping_float_to_double() {
    // FLOAT → DOUBLE for BigQuery
    validate_with_dialect(
        "SELECT CAST(x AS FLOAT) FROM t",
        "SELECT CAST(x AS DOUBLE) FROM t",
        Dialect::Postgres,
        Dialect::BigQuery,
    );
}

#[test]
fn test_transpile_type_mapping_bytea_blob() {
    // BYTEA → BLOB for MySQL/SQLite
    validate_with_dialect(
        "SELECT CAST(x AS BYTEA) FROM t",
        "SELECT CAST(x AS BLOB) FROM t",
        Dialect::Postgres,
        Dialect::Mysql,
    );
    // BLOB → BYTEA for Postgres
    validate_with_dialect(
        "SELECT CAST(x AS BLOB) FROM t",
        "SELECT x::BYTEA FROM t",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Parse error tests
// (from Python test_transpile.py::test_paren)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_parse_errors() {
    // Unmatched parentheses should fail
    assert!(parse("1 + (2 + 3", Dialect::Ansi).is_err());
    assert!(parse("SELECT (", Dialect::Ansi).is_err());
    assert!(parse("DELETE FROM", Dialect::Ansi).is_err());
    assert!(parse("SELECT TRIM()", Dialect::Mysql).is_err());
    assert!(parse("SELECT * FROM JSON_TABLE(data, '$.x'", Dialect::Mysql).is_err());
    // Empty input
    assert!(parse("", Dialect::Ansi).is_err());
}

// ═════════════════════════════════════════════════════════════════════════════
// Multi-statement parsing
// (from Python test_transpile.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_transpile_multiple_statements() {
    let results =
        sqlgrok::transpile_statements("SELECT 1; SELECT 2; SELECT 3", Dialect::Ansi, Dialect::Ansi)
            .unwrap();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], "SELECT 1");
    assert_eq!(results[1], "SELECT 2");
    assert_eq!(results[2], "SELECT 3");
}

// ═════════════════════════════════════════════════════════════════════════════
// Complex roundtrip tests combining multiple features
// (inspired by Python identity.sql complex queries)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_complex_join_where_order() {
    validate_identity(
        "SELECT u.id, u.name FROM users AS u INNER JOIN orders AS o ON u.id = o.user_id WHERE o.total > 100 ORDER BY u.name LIMIT 10",
    );
}

#[test]
fn test_identity_cte_with_join() {
    validate_identity(
        "WITH active_users AS (SELECT id, name FROM users WHERE active = TRUE) SELECT a.name, COUNT(*) FROM active_users AS a INNER JOIN orders AS o ON a.id = o.user_id GROUP BY a.name",
    );
}

#[test]
fn test_identity_subquery_in_select() {
    validate_identity("SELECT a, (SELECT MAX(b) FROM t2) AS max_b FROM t1");
}

#[test]
fn test_identity_union_with_order_limit() {
    validate_identity("SELECT a FROM t1 UNION ALL SELECT b FROM t2 ORDER BY 1 LIMIT 10");
}

#[test]
fn test_identity_nested_case_in_select() {
    validate_identity(
        "SELECT CASE WHEN x > 0 THEN CASE WHEN y > 0 THEN 'both' ELSE 'x_only' END ELSE 'none' END AS result FROM t",
    );
}

#[test]
fn test_identity_window_with_case() {
    validate_identity(
        "SELECT SUM(CASE WHEN status = 'active' THEN 1 ELSE 0 END) OVER (PARTITION BY dept) AS active_count FROM employees",
    );
}

#[test]
fn test_identity_multiple_ctes() {
    validate_identity(
        "WITH a AS (SELECT 1 AS x), b AS (SELECT 2 AS y), c AS (SELECT 3 AS z) SELECT * FROM a CROSS JOIN b CROSS JOIN c",
    );
}

#[test]
fn test_identity_insert_with_cte() {
    // Note: CTE with INSERT is complex; test the basic version
    validate_identity("INSERT INTO target SELECT * FROM src");
}

#[test]
fn test_identity_create_table_as() {
    validate_identity("CREATE TABLE new_t AS SELECT a, b FROM old_t WHERE a > 0");
}

// ═════════════════════════════════════════════════════════════════════════════
// Serde roundtrip tests
// (from Python test_serde.py)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_serde_roundtrip() {
    let test_cases = [
        "SELECT 1",
        "SELECT a, b FROM t WHERE a > 1",
        "WITH cte AS (SELECT 1) SELECT * FROM cte",
        "INSERT INTO t VALUES (1, 'a')",
        "CREATE TABLE t (a INT, b VARCHAR(100))",
    ];
    for sql in &test_cases {
        let ast = parse(sql, Dialect::Ansi).unwrap();
        let json = serde_json::to_string(&ast).unwrap();
        let deserialized: sqlgrok::Statement = serde_json::from_str(&json).unwrap();
        let output = generate(&deserialized, Dialect::Ansi);
        assert_eq!(output, *sql, "Serde roundtrip failed for: {}", sql);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// TRUNCATE
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identity_truncate() {
    validate_identity("TRUNCATE TABLE t");
}

// ═════════════════════════════════════════════════════════════════════════════
// SELECT TOP N (T-SQL) — Issue #1
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_top_n_star_tsql_roundtrip() {
    // Core bug: SELECT TOP 5 * was failing because * was consumed as multiply
    validate_with_dialect(
        "SELECT TOP 5 * FROM t",
        "SELECT TOP 5 * FROM t",
        Dialect::Tsql,
        Dialect::Tsql,
    );
}

#[test]
fn test_top_n_columns_tsql_roundtrip() {
    validate_with_dialect(
        "SELECT TOP 10 id, name FROM t",
        "SELECT TOP 10 id, name FROM t",
        Dialect::Tsql,
        Dialect::Tsql,
    );
}

#[test]
fn test_top_n_parenthesized_tsql_roundtrip() {
    validate_with_dialect(
        "SELECT TOP (5) * FROM t",
        "SELECT TOP (5) * FROM t",
        Dialect::Tsql,
        Dialect::Tsql,
    );
}

#[test]
fn test_top_distinct_tsql_roundtrip() {
    validate_with_dialect(
        "SELECT DISTINCT TOP 3 id FROM t",
        "SELECT DISTINCT TOP 3 id FROM t",
        Dialect::Tsql,
        Dialect::Tsql,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Typed Function Expressions — comprehensive tests
// ═════════════════════════════════════════════════════════════════════════════

// ── Date/Time typed functions ──

#[test]
fn test_typed_date_trunc_identity() {
    validate_identity("SELECT DATE_TRUNC('MONTH', created_at) FROM orders");
}

#[test]
fn test_typed_date_trunc_to_tsql() {
    validate_with_dialect(
        "SELECT DATE_TRUNC('MONTH', created_at) FROM orders",
        "SELECT DATETRUNC(MONTH, created_at) FROM orders",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_typed_date_trunc_to_oracle() {
    validate_with_dialect(
        "SELECT DATE_TRUNC('MONTH', created_at) FROM orders",
        "SELECT TRUNC(created_at, 'MONTH') FROM orders",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_typed_current_timestamp_roundtrip() {
    let cases = [
        "SELECT CURRENT_TIMESTAMP()",
        "SELECT COUNT(*) FROM t WHERE ts > CURRENT_TIMESTAMP()",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_typed_year_month_day() {
    // YEAR/MONTH/DAY → EXTRACT for non-TSQL
    validate_with_dialect(
        "SELECT YEAR(created_at) FROM t",
        "SELECT EXTRACT(YEAR FROM created_at) FROM t",
        Dialect::Ansi,
        Dialect::Postgres,
    );
    validate_with_dialect(
        "SELECT MONTH(created_at) FROM t",
        "SELECT EXTRACT(MONTH FROM created_at) FROM t",
        Dialect::Ansi,
        Dialect::Postgres,
    );
    validate_with_dialect(
        "SELECT DAY(created_at) FROM t",
        "SELECT EXTRACT(DAY FROM created_at) FROM t",
        Dialect::Ansi,
        Dialect::Postgres,
    );
}

#[test]
fn test_typed_year_tsql_preserves() {
    validate_with_dialect(
        "SELECT YEAR(created_at) FROM t",
        "SELECT YEAR(created_at) FROM t",
        Dialect::Tsql,
        Dialect::Tsql,
    );
}

// ── String typed functions ──

#[test]
fn test_typed_upper_lower_identity() {
    validate_identity("SELECT UPPER(name) FROM t");
    validate_identity("SELECT LOWER(name) FROM t");
}

#[test]
fn test_typed_trim_identity() {
    validate_identity("SELECT TRIM(name) FROM t");
}

#[test]
fn test_typed_length_cross_dialect() {
    validate_with_dialect(
        "SELECT LENGTH(name) FROM t",
        "SELECT LEN(name) FROM t",
        Dialect::Postgres,
        Dialect::Tsql,
    );
    validate_with_dialect(
        "SELECT LEN(name) FROM t",
        "SELECT LENGTH(name) FROM t",
        Dialect::Tsql,
        Dialect::Postgres,
    );
}

#[test]
fn test_typed_substring_cross_dialect() {
    validate_with_dialect(
        "SELECT SUBSTRING(name, 1, 3) FROM t",
        "SELECT SUBSTR(name, 1, 3) FROM t",
        Dialect::Postgres,
        Dialect::Mysql,
    );
    validate_with_dialect(
        "SELECT SUBSTR(name, 1, 3) FROM t",
        "SELECT SUBSTRING(name, 1, 3) FROM t",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_typed_replace_identity() {
    validate_identity("SELECT REPLACE(name, 'old', 'new') FROM t");
}

#[test]
fn test_typed_reverse_identity() {
    validate_identity("SELECT REVERSE(name) FROM t");
}

#[test]
fn test_typed_left_right_identity() {
    validate_identity("SELECT LEFT(name, 3) FROM t");
    validate_identity("SELECT RIGHT(name, 3) FROM t");
}

#[test]
fn test_typed_lpad_rpad_identity() {
    validate_identity("SELECT LPAD(name, 10, '*') FROM t");
    validate_identity("SELECT RPAD(name, 10) FROM t");
}

#[test]
fn test_typed_concat_ws_identity() {
    validate_identity("SELECT CONCAT_WS(', ', a, b, c) FROM t");
}

#[test]
fn test_typed_split_cross_dialect() {
    validate_with_dialect(
        "SELECT SPLIT(name, ',') FROM t",
        "SELECT STRING_SPLIT(name, ',') FROM t",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_typed_initcap_identity() {
    validate_identity("SELECT INITCAP(name) FROM t");
}

#[test]
fn test_typed_regexp_like_identity() {
    validate_identity("SELECT REGEXP_LIKE(name, '^A.*') FROM t");
}

#[test]
fn test_typed_regexp_replace_identity() {
    validate_identity("SELECT REGEXP_REPLACE(name, '[0-9]', 'X') FROM t");
}

// ── Aggregate typed functions ──

#[test]
fn test_typed_count_variations() {
    validate_identity("SELECT COUNT(*) FROM t");
    validate_identity("SELECT COUNT(a) FROM t");
    validate_identity("SELECT COUNT(DISTINCT a) FROM t");
}

#[test]
fn test_typed_sum_avg_min_max() {
    validate_identity("SELECT SUM(amount) FROM t");
    validate_identity("SELECT AVG(price) FROM t");
    validate_identity("SELECT MIN(created_at) FROM t");
    validate_identity("SELECT MAX(score) FROM t");
}

#[test]
fn test_typed_sum_distinct() {
    validate_identity("SELECT SUM(DISTINCT amount) FROM t");
}

#[test]
fn test_typed_array_agg_cross_dialect() {
    validate_with_dialect(
        "SELECT ARRAY_AGG(name) FROM t",
        "SELECT LIST(name) FROM t",
        Dialect::Postgres,
        Dialect::DuckDb,
    );
    validate_with_dialect(
        "SELECT ARRAY_AGG(name) FROM t",
        "SELECT COLLECT_LIST(name) FROM t",
        Dialect::Postgres,
        Dialect::Hive,
    );
}

#[test]
fn test_typed_variance_stddev() {
    validate_identity("SELECT VARIANCE(score) FROM t");
    validate_identity("SELECT STDDEV(score) FROM t");
}

// ── Window typed functions ──

#[test]
fn test_typed_row_number_with_over() {
    validate_identity("SELECT ROW_NUMBER() OVER (ORDER BY id) FROM t");
}

#[test]
fn test_typed_rank_dense_rank() {
    validate_identity("SELECT RANK() OVER (PARTITION BY dept ORDER BY salary) FROM t");
    validate_identity("SELECT DENSE_RANK() OVER (ORDER BY score DESC) FROM t");
}

#[test]
fn test_typed_ntile() {
    validate_identity("SELECT NTILE(4) OVER (ORDER BY id) FROM t");
}

#[test]
fn test_typed_lead_lag() {
    validate_identity("SELECT LEAD(price, 1) OVER (ORDER BY date) FROM t");
    validate_identity("SELECT LAG(price) OVER (ORDER BY date) FROM t");
    validate_identity("SELECT LAG(price, 1, 0) OVER (PARTITION BY category ORDER BY date) FROM t");
}

#[test]
fn test_typed_first_last_value() {
    validate_identity("SELECT FIRST_VALUE(name) OVER (ORDER BY id) FROM t");
    validate_identity("SELECT LAST_VALUE(name) OVER (ORDER BY id) FROM t");
}

#[test]
fn test_typed_window_with_filter() {
    validate_identity("SELECT COUNT(*) FILTER (WHERE active) FROM t");
    validate_identity("SELECT SUM(amount) FILTER (WHERE status = 'paid') FROM orders");
}

// ── Math typed functions ──

#[test]
fn test_typed_math_functions_identity() {
    let cases = [
        "SELECT ABS(x) FROM t",
        "SELECT CEIL(x) FROM t",
        "SELECT FLOOR(x) FROM t",
        "SELECT ROUND(x, 2) FROM t",
        "SELECT SQRT(x) FROM t",
        "SELECT LN(x) FROM t",
        "SELECT LOG(x) FROM t",
        "SELECT MOD(x, 3) FROM t",
    ];
    for sql in &cases {
        validate_identity(sql);
    }
}

#[test]
fn test_typed_pow_cross_dialect() {
    validate_with_dialect(
        "SELECT POW(x, 2) FROM t",
        "SELECT POWER(x, 2) FROM t",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_typed_ceil_cross_dialect() {
    validate_with_dialect(
        "SELECT CEIL(x) FROM t",
        "SELECT CEILING(x) FROM t",
        Dialect::Postgres,
        Dialect::Tsql,
    );
}

#[test]
fn test_typed_greatest_least() {
    validate_identity("SELECT GREATEST(a, b, c) FROM t");
    validate_identity("SELECT LEAST(a, b, c) FROM t");
}

// ── Array typed functions ──

#[test]
fn test_typed_array_size_cross_dialect() {
    validate_with_dialect(
        "SELECT ARRAY_SIZE(arr) FROM t",
        "SELECT ARRAY_LENGTH(arr) FROM t",
        Dialect::Snowflake,
        Dialect::Postgres,
    );
    validate_with_dialect(
        "SELECT ARRAY_SIZE(arr) FROM t",
        "SELECT SIZE(arr) FROM t",
        Dialect::Snowflake,
        Dialect::Hive,
    );
}

#[test]
fn test_typed_array_concat_cross_dialect() {
    validate_with_dialect(
        "SELECT ARRAY_CONCAT(a, b) FROM t",
        "SELECT ARRAY_CAT(a, b) FROM t",
        Dialect::BigQuery,
        Dialect::Postgres,
    );
}

#[test]
fn test_typed_generate_series() {
    validate_identity("SELECT GENERATE_SERIES(1, 10)");
    validate_identity("SELECT GENERATE_SERIES(1, 100, 5)");
}

#[test]
fn test_typed_flatten_identity() {
    validate_identity("SELECT FLATTEN(arr) FROM t");
}

#[test]
fn test_typed_explode_identity() {
    validate_identity("SELECT EXPLODE(arr) FROM t");
}

// ── JSON typed functions ──

#[test]
fn test_typed_json_extract_cross_dialect() {
    validate_with_dialect(
        "SELECT JSON_EXTRACT(data, '$.name') FROM t",
        "SELECT JSON_VALUE(data, '$.name') FROM t",
        Dialect::Mysql,
        Dialect::Tsql,
    );
}

#[test]
fn test_typed_json_extract_scalar_identity() {
    validate_identity("SELECT JSON_EXTRACT_SCALAR(data, '$.name') FROM t");
}

#[test]
fn test_typed_json_format_cross_dialect() {
    validate_with_dialect(
        "SELECT JSON_FORMAT(data) FROM t",
        "SELECT TO_JSON_STRING(data) FROM t",
        Dialect::Ansi,
        Dialect::BigQuery,
    );
}

// ── Conversion typed functions ──

#[test]
fn test_typed_hex_cross_dialect() {
    validate_with_dialect(
        "SELECT HEX(data) FROM t",
        "SELECT TO_HEX(data) FROM t",
        Dialect::Mysql,
        Dialect::Presto,
    );
}

#[test]
fn test_typed_unhex_cross_dialect() {
    validate_with_dialect(
        "SELECT UNHEX(data) FROM t",
        "SELECT FROM_HEX(data) FROM t",
        Dialect::Mysql,
        Dialect::Trino,
    );
}

#[test]
fn test_typed_md5_identity() {
    validate_identity("SELECT MD5(password) FROM t");
}

#[test]
fn test_typed_sha_cross_dialect() {
    validate_with_dialect(
        "SELECT SHA(data) FROM t",
        "SELECT SHA1(data) FROM t",
        Dialect::Postgres,
        Dialect::Mysql,
    );
}

// ── Generic function fallback ──

#[test]
fn test_generic_function_fallback() {
    // Unrecognized functions should still work via Expr::Function
    validate_identity("SELECT MY_CUSTOM_FUNC(a, b) FROM t");
    validate_identity("SELECT SOME_UDF(x) FROM t");
}

// ── Complex expressions with typed functions ──

#[test]
fn test_typed_functions_in_complex_expressions() {
    validate_identity("SELECT COUNT(*), SUM(amount), AVG(price) FROM orders GROUP BY category");
    validate_identity(
        "SELECT ROW_NUMBER() OVER (PARTITION BY dept ORDER BY salary DESC) AS rn FROM emp",
    );
    validate_identity("SELECT UPPER(SUBSTRING(name, 1, 1)) FROM t");
    validate_identity("SELECT GREATEST(a, LEAST(b, c)) FROM t");
    validate_identity("SELECT ROUND(AVG(score), 2) FROM t");
}

#[test]
fn test_typed_functions_in_where_clause() {
    validate_identity("SELECT * FROM t WHERE LENGTH(name) > 5");
    validate_identity("SELECT * FROM t WHERE ABS(score) < 10");
    validate_identity("SELECT * FROM t WHERE UPPER(status) = 'ACTIVE'");
}

#[test]
fn test_typed_functions_nested() {
    validate_identity("SELECT ROUND(SQRT(ABS(x)), 2) FROM t");
    validate_identity("SELECT UPPER(REVERSE(TRIM(name))) FROM t");
}

#[test]
fn test_typed_functions_with_aliases() {
    validate_identity("SELECT COUNT(*) AS total, MAX(price) AS max_price FROM t");
    validate_identity("SELECT ROW_NUMBER() OVER (ORDER BY id) AS rn FROM t");
}

// ═════════════════════════════════════════════════════════════════════════════
// PIVOT / UNPIVOT
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_pivot_basic() {
    validate_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2', 'Q3', 'Q4'))",
    );
}

#[test]
fn test_pivot_with_alias() {
    validate_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2', 'Q3', 'Q4')) AS pvt",
    );
}

#[test]
fn test_pivot_with_aliased_values() {
    validate_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1' AS q1, 'Q2' AS q2))",
    );
}

#[test]
fn test_pivot_with_count() {
    validate_identity(
        "SELECT * FROM orders PIVOT (COUNT(*) FOR status IN ('open', 'closed', 'pending'))",
    );
}

#[test]
fn test_pivot_subquery_source() {
    validate_identity(
        "SELECT * FROM (SELECT * FROM sales) AS s PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2'))",
    );
}

#[test]
fn test_unpivot_basic() {
    validate_identity("SELECT * FROM quarterly UNPIVOT (amount FOR quarter IN (Q1, Q2, Q3, Q4))");
}

#[test]
fn test_unpivot_with_alias() {
    validate_identity(
        "SELECT * FROM quarterly UNPIVOT (amount FOR quarter IN (Q1, Q2, Q3, Q4)) AS unpvt",
    );
}

#[test]
fn test_unpivot_with_aliased_columns() {
    validate_identity(
        "SELECT * FROM quarterly UNPIVOT (amount FOR quarter IN (Q1 AS q1, Q2 AS q2))",
    );
}

#[test]
fn test_pivot_with_where() {
    validate_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2')) AS pvt WHERE pvt.Q1 > 100",
    );
}

#[test]
fn test_pivot_with_join() {
    validate_identity(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2')) AS pvt INNER JOIN regions ON pvt.region_id = regions.id",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Time Format Mapping Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_time_format_mysql_to_postgres() {
    // MySQL DATE_FORMAT should transpile to PostgreSQL TO_CHAR with format conversion
    validate_with_dialect(
        "SELECT DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s')",
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DD HH24:MI:SS')",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_time_format_postgres_to_mysql() {
    // PostgreSQL TO_CHAR should transpile to MySQL DATE_FORMAT with format conversion
    validate_with_dialect(
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DD HH24:MI:SS')",
        "SELECT DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s')",
        Dialect::Postgres,
        Dialect::Mysql,
    );
}

#[test]
fn test_time_format_mysql_to_sqlite_strftime() {
    validate_with_dialect(
        "SELECT DATE_FORMAT(d, '%Y-%m-%d') FROM t",
        "SELECT STRFTIME('%Y-%m-%d', d) FROM t",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_sqlite_time_function_parity_batch() {
    validate_with_dialect(
        "SELECT MAKETIME(15, 30, 00)",
        "SELECT TIME_FROM_PARTS(15, 30, 00)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT MAKE_TIME(15, 30, 00)",
        "SELECT TIME_FROM_PARTS(15, 30, 00)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT UTC_TIME(6), UTC_TIMESTAMP()",
        "SELECT CURRENT_TIME, CURRENT_TIMESTAMP",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TIME_STR_TO_TIME(x), FROM_UNIXTIME(col)",
        "SELECT x, UNIX_TO_TIME(col)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TO_TIMESTAMP(col)",
        "SELECT UNIX_TO_TIME(col)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_time_format_ambiguous_tokens_to_sqlite() {
    validate_with_dialect(
        "SELECT DATE_FORMAT('2009-10-04 22:23:00', '%W %M %Y')",
        "SELECT STRFTIME('%A %B %Y', '2009-10-04 22:23:00')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT STR_TO_DATE(x, '%M')",
        "SELECT STR_TO_DATE(x, '%B')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT STR_TO_DATE(x, '%Y-%m-%dT%T')",
        "SELECT STR_TO_TIME(x, '%Y-%m-%dT%H:%M:%S')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_sqlite_time_report_parity_batch() {
    validate_with_dialect(
        "TIMESTAMPDIFF(month, b, a)",
        "TIMESTAMPDIFF(a, b, MONTH)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT FROM_UNIXTIME(1711366265, '%Y %D %M')",
        "SELECT UNIX_TO_TIME(1711366265, '%Y %D %B')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TO_DATE('01/01/2000', 'MM/DD/YYYY')",
        "SELECT STR_TO_DATE('01/01/2000', '%m/%d/%Y')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TO_TIMESTAMP('05 Dec 2000', 'DD Mon YYYY')",
        "SELECT STR_TO_TIME('05 Dec 2000', '%d Mon %Y')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TO_TIMESTAMP('05 Dec 2000 10:00 AM', 'DD Mon YYYY HH:MI AM')",
        "SELECT STR_TO_TIME('05 Dec 2000 10:00 AM', '%d Mon %Y HH:%M AM')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TO_CHAR(CAST('2020-02-03 04:05:06.789' AS TIMESTAMP), 'YY-DDD HH24:MI:SS.US TZ')",
        "SELECT STRFTIME('%y-%j %H:%M:%S.%f %Z', CAST('2020-02-03 04:05:06.789' AS TIMESTAMP))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "GENERATE_SERIES('2019-01-01'::TIMESTAMP, NOW(), '1day')",
        "UNNEST(GENERATE_SERIES(CAST('2019-01-01' AS TIMESTAMP), CURRENT_TIMESTAMP, INTERVAL '1' DAY))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_time_rust_error_report_batch_to_sqlite() {
    validate_with_dialect(
        "SELECT CAST('2025-02-01 00:00:00' AS TIMESTAMP) - MAKE_INTERVAL(years => 1)",
        "SELECT CAST('2025-02-01 00:00:00' AS TIMESTAMP) - MAKE_INTERVAL(years => 1)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT NOW() + MAKE_INTERVAL(years => 1, months => 2, days => 3)",
        "SELECT CURRENT_TIMESTAMP + MAKE_INTERVAL(years => 1, months => 2, days => 3)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT XMLELEMENT(NAME foo, XMLATTRIBUTES(CURRENT_DATE AS bar), 'cont', 'ent')",
        "SELECT XMLELEMENT(NAME foo, XMLATTRIBUTES(CURRENT_DATE AS bar), 'cont', 'ent')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "INSERT INTO book (isbn, title) VALUES ($1, $2) ON CONFLICT(isbn) WHERE deleted_at IS NULL DO UPDATE SET title = EXCLUDED.title RETURNING id, isbn",
        "INSERT INTO book (isbn, title) VALUES (@1, @2) ON CONFLICT(isbn) WHERE deleted_at IS NULL DO UPDATE SET title = EXCLUDED.title RETURNING id, isbn",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "INSERT INTO x VALUES (1, 'a', 2.0) ON CONFLICT ON CONSTRAINT pkey DO UPDATE SET x.id = 1 RETURNING *",
        "INSERT INTO x VALUES (1, 'a', 2.0) ON CONFLICT ON CONSTRAINT pkey DO UPDATE SET x.id = 1 RETURNING *",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_postgres_join_rust_error_report_batch_to_sqlite() {
    validate_with_dialect(
        "SELECT id, name FROM xml_data AS t, XMLTABLE('/root/user' PASSING t.xml COLUMNS id INT PATH '@id', name TEXT PATH 'name/text()') AS x",
        "SELECT id, name FROM xml_data AS t, XMLTABLE('/root/user' PASSING t.xml COLUMNS id INTEGER PATH '@id', name TEXT PATH 'name/text()') AS x",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT * FROM ROWS FROM (FUNC1(col1, col2))",
        "SELECT * FROM ROWS FROM (FUNC1(col1, col2))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT * FROM ROWS FROM (FUNC1(col1) AS alias1(\"col1\" TEXT), FUNC2(col2) AS alias2(\"col2\" INT)) WITH ORDINALITY",
        "SELECT * FROM ROWS FROM (FUNC1(col1) AS alias1, FUNC2(col2) AS alias2) WITH ORDINALITY",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT * FROM UNNEST(ARRAY[1, 2], ARRAY['foo', 'bar', 'baz']) AS x(a, b)",
        "SELECT * FROM UNNEST(ARRAY(1, 2), ARRAY('foo', 'bar', 'baz')) AS x",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(ARRAY_TO_STRING(ARRAY(SELECT val FROM UNNEST(ARRAY['a', 'b']) WITH ORDINALITY AS u(val, rn)), ' '))",
        "SELECT TRIM(ARRAY_TO_STRING(ARRAY(SELECT val FROM UNNEST(ARRAY('a', 'b')) WITH ORDINALITY AS u), ' '))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT MLEAST(VARIADIC ARRAY[10, -1, 5, 4.4])",
        "SELECT MLEAST(VARIADIC ARRAY(10, -1, 5, 4.4))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT * FROM schema_name.table_name st WHERE JSON_EXTRACT_PATH_TEXT((st.data)::json, variadic array['test'::text]) = 'test'::text",
        "SELECT * FROM schema_name.table_name AS st WHERE CAST((st.data) AS JSON) ->> VARIADIC ARRAY(CAST('test' AS TEXT)) = CAST('test' AS TEXT)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(BOTH ' XXX ')",
        "SELECT TRIM(' XXX ')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(LEADING ' XXX ' COLLATE \"de_DE\")",
        "SELECT LTRIM(' XXX ' COLLATE \"de_DE\")",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(TRAILING ' XXX ' COLLATE \"de_DE\")",
        "SELECT RTRIM(' XXX ' COLLATE \"de_DE\")",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT SUBSTRING('Thomas' FOR 3 FROM 2)",
        "SELECT SUBSTRING('Thomas', 2, 3)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT SUBSTRING('afafa' for 1)",
        "SELECT SUBSTRING('afafa', 1, 1)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect("|/ x", "SQRT(x)", Dialect::Postgres, Dialect::Sqlite);
    validate_with_dialect("||/ x", "CBRT(x)", Dialect::Postgres, Dialect::Sqlite);
    validate_with_dialect(
        "SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY x)",
        "SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY x NULLS LAST)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY x) OVER ()",
        "SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY x NULLS LAST) OVER ()",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY a) FILTER(WHERE CAST(b AS BOOLEAN)) AS mean_value FROM (VALUES (0, 't')) AS fake_data(a, b)",
        "SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY a NULLS LAST) FILTER(WHERE CAST(b AS INTEGER)) AS mean_value FROM (VALUES (0, 't')) AS fake_data",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 'testa 1' NOT LIKE ALL (ARRAY['testa%', 'testb%'])",
        "SELECT 'testa 1' NOT LIKE ALL (ARRAY('testa%', 'testb%'))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 'testa 1' NOT ILIKE ALL (ARRAY['testa%', 'testb%'])",
        "SELECT LOWER('testa 1') NOT LIKE LOWER(ALL (ARRAY('testa%', 'testb%')))",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CAST('a' AS TEXT COLLATE \"de_DE\")",
        "CAST('a' AS TEXT COLLATE \"de_DE\")",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT CAST('a' AS VARCHAR COLLATE foo)",
        "SELECT CAST('a' AS TEXT COLLATE foo)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT NUMRANGE(1.1, 2.2) -|- NUMRANGE(2.2, 3.3)",
        "SELECT NUMRANGE(1.1, 2.2) -|- NUMRANGE(2.2, 3.3)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT p1.id, p2.id, v1, v2 FROM polygons AS p1, polygons AS p2, LATERAL VERTICES(p1.poly) v1, LATERAL VERTICES(p2.poly) v2 WHERE (v1 <-> v2) < 10 AND p1.id <> p2.id",
        "SELECT p1.id, p2.id, v1, v2 FROM polygons AS p1, polygons AS p2, LATERAL VERTICES(p1.poly) AS v1, LATERAL VERTICES(p2.poly) AS v2 WHERE (v1 <-> v2) < 10 AND p1.id <> p2.id",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "END WORK AND NO CHAIN",
        "COMMIT AND NO CHAIN",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "END AND CHAIN",
        "COMMIT AND CHAIN",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT OVERLAY(a PLACING b FROM 1)",
        "SELECT OVERLAY(a PLACING b FROM 1)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT OVERLAY(a PLACING b FROM 1 FOR 1)",
        "SELECT OVERLAY(a PLACING b FROM 1 FOR 1)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "x::JSON -> 'duration' ->> -1",
        "CAST(x AS JSON) -> '$.duration' ->> -1",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "WITH RECURSIVE search_tree(id, link, data) AS (SELECT t.id, t.link, t.data FROM tree AS t UNION ALL SELECT t.id, t.link, t.data FROM tree AS t, search_tree AS st WHERE t.id = st.link) SEARCH BREADTH FIRST BY id SET ordercol SELECT * FROM search_tree ORDER BY ordercol",
        "WITH RECURSIVE search_tree(id, link, data) AS (SELECT t.id, t.link, t.data FROM tree AS t UNION ALL SELECT t.id, t.link, t.data FROM tree AS t, search_tree AS st WHERE t.id = st.link) SEARCH BREADTH FIRST BY id SET ordercol SELECT * FROM search_tree ORDER BY ordercol NULLS LAST",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "WITH RECURSIVE search_graph(id, link, data, depth) AS (SELECT g.id, g.link, g.data, 1 FROM graph AS g UNION ALL SELECT g.id, g.link, g.data, sg.depth + 1 FROM graph AS g, search_graph AS sg WHERE g.id = sg.link) CYCLE id SET is_cycle USING path SELECT * FROM search_graph",
        "WITH RECURSIVE search_graph(id, link, data, depth) AS (SELECT g.id, g.link, g.data, 1 FROM graph AS g UNION ALL SELECT g.id, g.link, g.data, sg.depth + 1 FROM graph AS g, search_graph AS sg WHERE g.id = sg.link) CYCLE id SET is_cycle USING path SELECT * FROM search_graph",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CAST(x AS sch.udt)",
        "CAST(x AS sch.udt)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "COPY (SELECT * FROM t) TO 'file' WITH (FORMAT format, HEADER MATCH, FREEZE TRUE)",
        "COPY INTO (SELECT * FROM t) TO 'file' WITH (FORMAT format, HEADER MATCH, FREEZE TRUE)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT a <<->> b",
        "SELECT a <<->> b",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "select count() OVER(partition by a order by a range offset preceding exclude current row)",
        "SELECT COUNT() OVER (PARTITION BY a ORDER BY a NULLS LAST range BETWEEN offset preceding AND CURRENT ROW EXCLUDE CURRENT ROW)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT ARRAY[1, 2, 3] <@ ARRAY[1, 2]",
        "SELECT ARRAY(1, 2) @> ARRAY(1, 2, 3)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "MERGE INTO target_table USING source_table AS source ON target.id = source.id WHEN MATCHED THEN DO NOTHING WHEN NOT MATCHED THEN DO NOTHING RETURNING MERGE_ACTION(), *",
        "MERGE INTO target_table USING source_table AS source ON target.id = source.id WHEN MATCHED THEN DO NOTHING WHEN NOT MATCHED THEN DO NOTHING RETURNING MERGE_ACTION(), *",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 1 FROM ((VALUES (1)) AS vals(id) LEFT OUTER JOIN tbl ON vals.id = tbl.id)",
        "SELECT 1 FROM ((VALUES (1)) AS vals LEFT OUTER JOIN tbl ON vals.id = tbl.id)",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
}

#[test]
fn test_time_format_mysql_to_spark() {
    // MySQL format to Spark Java DateTimeFormatter style
    validate_with_dialect(
        "SELECT DATE_FORMAT(created_at, '%Y-%m-%d')",
        "SELECT DATE_FORMAT(created_at, 'yyyy-MM-dd')",
        Dialect::Mysql,
        Dialect::Spark,
    );
}

#[test]
fn test_time_format_postgres_to_snowflake() {
    // PostgreSQL TO_CHAR to Snowflake (which uses similar Postgres-style format)
    validate_with_dialect(
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DD')",
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DD')",
        Dialect::Postgres,
        Dialect::Snowflake,
    );
}

#[test]
fn test_time_format_spark_to_postgres() {
    // Spark Java format to PostgreSQL
    validate_with_dialect(
        "SELECT DATE_FORMAT(created_at, 'yyyy-MM-dd HH:mm:ss')",
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DD HH24:MI:SS')",
        Dialect::Spark,
        Dialect::Postgres,
    );
}

#[test]
fn test_time_format_with_12hour_clock() {
    // 12-hour clock format with AM/PM (MySQL uses %h for 12-hour)
    validate_with_dialect(
        "SELECT DATE_FORMAT(created_at, '%Y-%m-%d %h:%i %p')",
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DD HH12:MI AM')",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_time_format_mysql_to_bigquery() {
    // MySQL to BigQuery (BigQuery uses strftime-like format)
    validate_with_dialect(
        "SELECT DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s')",
        "SELECT FORMAT_TIMESTAMP(created_at, '%Y-%m-%d %H:%M:%S')",
        Dialect::Mysql,
        Dialect::BigQuery,
    );
}

#[test]
fn test_time_format_with_literals() {
    // Format with literal characters (like T in ISO format)
    validate_with_dialect(
        "SELECT DATE_FORMAT(created_at, '%Y-%m-%dT%H:%i:%s')",
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DDTHH24:MI:SS')",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_str_to_time_mysql_to_postgres() {
    // STR_TO_DATE to TO_TIMESTAMP conversion
    validate_with_dialect(
        "SELECT STR_TO_DATE(date_str, '%Y-%m-%d')",
        "SELECT TO_TIMESTAMP(date_str, 'YYYY-MM-DD')",
        Dialect::Mysql,
        Dialect::Postgres,
    );
}

#[test]
fn test_time_format_identity_mysql() {
    // Identity test - MySQL format should remain unchanged when transpiling to MySQL
    validate_with_dialect(
        "SELECT DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s')",
        "SELECT DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s')",
        Dialect::Mysql,
        Dialect::Mysql,
    );
}

#[test]
fn test_time_format_identity_postgres() {
    // Identity test - PostgreSQL format should remain unchanged
    validate_with_dialect(
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DD HH24:MI:SS')",
        "SELECT TO_CHAR(created_at, 'YYYY-MM-DD HH24:MI:SS')",
        Dialect::Postgres,
        Dialect::Postgres,
    );
}

#[test]
fn test_oracle_omits_as_in_table_alias() {
    // Oracle forbids AS between a table reference and its alias
    validate_with_dialect(
        "SELECT * FROM users AS u WHERE u.id = 1",
        "SELECT * FROM users u WHERE u.id = 1",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_oracle_omits_as_in_join_table_alias() {
    validate_with_dialect(
        "SELECT a.name, b.email FROM accounts AS a INNER JOIN users AS b ON a.user_id = b.id",
        "SELECT a.name, b.email FROM accounts a INNER JOIN users b ON a.user_id = b.id",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_oracle_omits_as_in_subquery_alias() {
    validate_with_dialect(
        "SELECT * FROM (SELECT id, name FROM users) AS sub WHERE sub.id > 10",
        "SELECT * FROM (SELECT id, name FROM users) sub WHERE sub.id > 10",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_oracle_preserves_column_alias_as() {
    // Column aliases should still use AS even for Oracle
    validate_with_dialect(
        "SELECT first_name AS fname, last_name AS lname FROM employees",
        "SELECT first_name AS fname, last_name AS lname FROM employees",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_oracle_catalog_query_no_spurious_as() {
    // Catalog query that already has no AS should not gain one
    validate_with_dialect(
        "SELECT U.* FROM ALL_USERS U WHERE (U.USERNAME IS NOT NULL)",
        "SELECT U.* FROM ALL_USERS U WHERE (U.USERNAME IS NOT NULL)",
        Dialect::Postgres,
        Dialect::Oracle,
    );
}

#[test]
fn test_non_oracle_keeps_table_alias_as() {
    // Non-Oracle dialects should still emit AS
    validate_with_dialect(
        "SELECT * FROM users AS u",
        "SELECT * FROM users AS u",
        Dialect::Postgres,
        Dialect::Postgres,
    );
}

#[test]
fn test_sqlite_window_exclude_parses_without_crashing() {
    let result = transpile(
        "SELECT SUM(X) OVER (ORDER BY X ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW EXCLUDE NO OTHERS) FROM t",
        Dialect::Sqlite,
        Dialect::Sqlite,
    )
    .expect("window EXCLUDE should parse");
    assert!(!result.is_empty());
}

#[test]
fn test_mysql_parser_carriers_to_sqlite() {
    validate_with_dialect(
        "SELECT 0xCC",
        "SELECT x'CC'",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 0b1011",
        "SELECT 11",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT b'1011'",
        "SELECT 11",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT x'CC'",
        "SELECT x'CC'",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT x'0000CC'",
        "SELECT x'0000CC'",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 0b102",
        "SELECT \"0b102\"",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    assert!(transpile("SELECT b'1012'", Dialect::Mysql, Dialect::Sqlite).is_err());
    validate_with_dialect(
        "SELECT @var1 := 1, @var2",
        "SELECT @var1 := 1, @var2",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "INSERT INTO x VALUES (1, 'a', 2.0) ON DUPLICATE KEY UPDATE x.id = 1",
        "INSERT INTO x VALUES (1, 'a', 2.0) ON DUPLICATE KEY UPDATE SET x.id = 1",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "CHAR(77, 77.3, '77.3' USING utf8mb4)",
        "CHAR(77, 77.3, '77.3', utf8mb4)",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "DELETE FROM t1, t2 USING t1 INNER JOIN t2 INNER JOIN t3 WHERE t1.id = t2.id AND t2.id = t3.id",
        "DELETE FROM t1, t2 USING t1 INNER JOIN t2 INNER JOIN t3 WHERE t1.id = t2.id AND t2.id = t3.id",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_trim_variants_to_sqlite() {
    validate_with_dialect(
        "SELECT TRIM(LEADING 'bla' FROM ' XXX ')",
        "SELECT LTRIM(' XXX ', 'bla')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(TRAILING 'bla' FROM ' XXX ')",
        "SELECT RTRIM(' XXX ', 'bla')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(BOTH 'bla' FROM ' XXX ')",
        "SELECT TRIM(' XXX ', 'bla')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM('bla' FROM ' XXX ')",
        "SELECT TRIM(' XXX ', 'bla')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(LEADING FROM ' XXX ')",
        "SELECT LTRIM(' XXX ')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(TRAILING FROM ' XXX ')",
        "SELECT RTRIM(' XXX ')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT TRIM(BOTH FROM ' XXX ')",
        "SELECT TRIM(' XXX ')",
        Dialect::Mysql,
        Dialect::Sqlite,
    );
}

#[test]
fn test_mysql_json_table_carrier_to_sqlite() {
    let result = transpile(
        "SELECT * FROM source, JSON_TABLE(source.links, '$.org[*]' COLUMNS(row_id FOR ORDINALITY, link VARCHAR(255) PATH '$.link')) AS links",
        Dialect::Mysql,
        Dialect::Sqlite,
    )
    .expect("JSON_TABLE table source should parse");
    assert_eq!(
        result,
        "SELECT * FROM source, JSON_TABLE(source.links, '$.org[*]' COLUMNS(row_id FOR ORDINALITY, link TEXT(255) PATH '$.link')) AS links"
    );
}

#[test]
fn test_postgres_parser_carriers_to_sqlite() {
    validate_with_dialect(
        "SELECT * FROM (VALUES (1)) AS t1",
        "SELECT * FROM (VALUES (1)) AS t1",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT $$Dianne's horse$$",
        "SELECT 'Dianne''s horse'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT * FROM foo WHERE id = %s",
        "SELECT * FROM foo WHERE id = ?",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT x ~ 'y' FROM t",
        "SELECT REGEXP_LIKE(x, 'y') FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT x ~* 'y' FROM t",
        "SELECT REGEXP_I_LIKE(x, 'y') FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT x !~ 'y' FROM t",
        "SELECT NOT REGEXP_LIKE(x, 'y') FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT x !~* 'y' FROM t",
        "SELECT NOT REGEXP_I_LIKE(x, 'y') FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect("x ~~ 'y'", "x LIKE 'y'", Dialect::Postgres, Dialect::Sqlite);
    validate_with_dialect(
        "x ~~* 'y'",
        "LOWER(x) LIKE LOWER('y')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "x !~~ 'y'",
        "x NOT LIKE 'y'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "x !~~* 'y'",
        "LOWER(x) NOT LIKE LOWER('y')",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT ~ ~x",
        "SELECT ~~x",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT '%' SIMILAR TO '^%' ESCAPE '^'",
        "SELECT '%' SIMILAR TO '^%' ESCAPE '^'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT a SIMILAR TO b ESCAPE '#' FROM t",
        "SELECT a SIMILAR TO b ESCAPE '#' FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 'abc' SIMILAR TO 'a_c' ESCAPE '_'",
        "SELECT 'abc' SIMILAR TO 'a_c' ESCAPE '_'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 'a_c' SIMILAR TO 'a#_c' ESCAPE '#'",
        "SELECT 'a_c' SIMILAR TO 'a#_c' ESCAPE '#'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 'abc' SIMILAR TO '(abc|def)'",
        "SELECT 'abc' SIMILAR TO '(abc|def)'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 'abbbc' SIMILAR TO 'ab{2,3}c'",
        "SELECT 'abbbc' SIMILAR TO 'ab{2,3}c'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 'a*c' SIMILAR TO 'a#*c' ESCAPE '#'",
        "SELECT 'a*c' SIMILAR TO 'a#*c' ESCAPE '#'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT 'abc' SIMILAR TO '[ab]%'",
        "SELECT 'abc' SIMILAR TO '[ab]%'",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    validate_with_dialect(
        "SELECT a NOT SIMILAR TO b FROM t",
        "SELECT NOT a SIMILAR TO b FROM t",
        Dialect::Postgres,
        Dialect::Sqlite,
    );
    assert!(transpile("SELECT ~~x", Dialect::Postgres, Dialect::Sqlite).is_err());
}

#[test]
fn test_upstream_sqlglot_similar_to_identity_cases() {
    validate_with_dialect(
        "SELECT '%' SIMILAR TO '^%' ESCAPE '^'",
        "SELECT '%' SIMILAR TO '^%' ESCAPE '^'",
        Dialect::Postgres,
        Dialect::Postgres,
    );
    validate_with_dialect(
        "'abc' SIMILAR TO '(b|c)%'",
        "'abc' SIMILAR TO '(b|c)%'",
        Dialect::Redshift,
        Dialect::Redshift,
    );
    validate_with_dialect(
        "'%' SIMILAR TO '^%' ESCAPE '^'",
        "'%' SIMILAR TO '^%' ESCAPE '^'",
        Dialect::Redshift,
        Dialect::Redshift,
    );
}

#[test]
fn test_large_similar_to_sqlglot_shaped_corpus() {
    let cases = [
        (
            "SELECT 'abc' SIMILAR TO 'abc'",
            "SELECT 'abc' SIMILAR TO 'abc'",
        ),
        (
            "SELECT 'abc' SIMILAR TO 'a_c'",
            "SELECT 'abc' SIMILAR TO 'a_c'",
        ),
        (
            "SELECT 'axyzc' SIMILAR TO 'a%c'",
            "SELECT 'axyzc' SIMILAR TO 'a%c'",
        ),
        (
            "SELECT 'abc' NOT SIMILAR TO 'z%'",
            "SELECT NOT 'abc' SIMILAR TO 'z%'",
        ),
        (
            "SELECT 'def' SIMILAR TO '(abc|def)'",
            "SELECT 'def' SIMILAR TO '(abc|def)'",
        ),
        (
            "SELECT 'abbc' SIMILAR TO 'ab+c'",
            "SELECT 'abbc' SIMILAR TO 'ab+c'",
        ),
        (
            "SELECT 'ac' SIMILAR TO 'ab?c'",
            "SELECT 'ac' SIMILAR TO 'ab?c'",
        ),
        (
            "SELECT 'abbbbbc' SIMILAR TO 'ab*c'",
            "SELECT 'abbbbbc' SIMILAR TO 'ab*c'",
        ),
        (
            "SELECT 'zbc' SIMILAR TO '[^z]%'",
            "SELECT 'zbc' SIMILAR TO '[^z]%'",
        ),
        (
            "SELECT 'a.c' SIMILAR TO 'a.c'",
            "SELECT 'a.c' SIMILAR TO 'a.c'",
        ),
        (
            "SELECT 'abc' SIMILAR TO 'a.c'",
            "SELECT 'abc' SIMILAR TO 'a.c'",
        ),
        (
            "SELECT 'a%c' SIMILAR TO 'a#%c' ESCAPE '#'",
            "SELECT 'a%c' SIMILAR TO 'a#%c' ESCAPE '#'",
        ),
        (
            "SELECT 'abc' SIMILAR TO 'a\\_c'",
            "SELECT 'abc' SIMILAR TO 'a\\_c'",
        ),
        (
            "SELECT 'a_c' SIMILAR TO 'a\\_c'",
            "SELECT 'a_c' SIMILAR TO 'a\\_c'",
        ),
        (
            "SELECT col NOT SIMILAR TO '(foo|bar)%' FROM t",
            "SELECT NOT col SIMILAR TO '(foo|bar)%' FROM t",
        ),
        (
            "SELECT 'abc' SIMILAR TO '[a-c]%'",
            "SELECT 'abc' SIMILAR TO '[a-c]%'",
        ),
        (
            "SELECT '1bc' SIMILAR TO '[[:digit:]]%'",
            "SELECT '1bc' SIMILAR TO '[[:digit:]]%'",
        ),
        (
            "SELECT 'abc' SIMILAR TO '(a|b|c)+'",
            "SELECT 'abc' SIMILAR TO '(a|b|c)+'",
        ),
    ];

    for (sql, expected) in cases {
        validate_with_dialect(sql, expected, Dialect::Postgres, Dialect::Sqlite);
    }
}
