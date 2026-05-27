use sqlgrok::optimizer::optimize;
/// Tests ported from Python sqlglot's `test_optimizer.py`.
///
/// Covers constant folding (arithmetic, string concat, comparisons),
/// boolean simplification (AND/OR with TRUE/FALSE, double NOT, WHERE TRUE),
/// subquery unnesting / decorrelation (EXISTS/IN → JOINs),
/// and predicate pushdown (WHERE → derived tables / JOINs).
use sqlgrok::{Dialect, generate, parse};

/// Parse → optimise → generate, compare with expected output.
fn validate_optimized(input: &str, expected: &str) {
    let stmt = parse(input, Dialect::Ansi)
        .unwrap_or_else(|e| panic!("Parse failed for '{}': {}", input, e));
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert_eq!(output, expected, "\n  Input: {}", input);
}

/// Ensures the optimized output equals the original (i.e. no-op optimization).
fn validate_no_op(sql: &str) {
    validate_optimized(sql, sql);
}

// ═════════════════════════════════════════════════════════════════════════════
// Constant Folding – Arithmetic
// (from Python test_optimizer.py::test_simplify)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_fold_addition() {
    validate_optimized("SELECT 1 + 2", "SELECT 3");
    validate_optimized("SELECT 10 + 20 + 30", "SELECT 60");
}

#[test]
fn test_fold_subtraction() {
    validate_optimized("SELECT 10 - 3", "SELECT 7");
}

#[test]
fn test_fold_multiplication() {
    validate_optimized("SELECT 3 * 4", "SELECT 12");
    validate_optimized("SELECT 2 * 3 * 4", "SELECT 24");
}

#[test]
fn test_fold_division() {
    validate_optimized("SELECT 10 / 2", "SELECT 5");
}

#[test]
fn test_fold_modulo() {
    validate_optimized("SELECT 10 % 3", "SELECT 1");
}

#[test]
fn test_fold_mixed_arithmetic() {
    validate_optimized("SELECT 1 + 2 * 3", "SELECT 7");
}

// ═════════════════════════════════════════════════════════════════════════════
// Constant Folding – String Concatenation
// (from Python test_optimizer.py::test_simplify)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_fold_string_concat() {
    validate_optimized("SELECT 'hello' || ' ' || 'world'", "SELECT 'hello world'");
    validate_optimized("SELECT 'foo' || 'bar'", "SELECT 'foobar'");
}

// ═════════════════════════════════════════════════════════════════════════════
// Constant Folding – Comparisons
// (from Python test_optimizer.py::test_simplify)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_fold_comparisons() {
    validate_optimized("SELECT 1 < 2", "SELECT TRUE");
    validate_optimized("SELECT 1 > 2", "SELECT FALSE");
    validate_optimized("SELECT 1 = 1", "SELECT TRUE");
    validate_optimized("SELECT 1 <> 1", "SELECT FALSE");
    validate_optimized("SELECT 1 <= 1", "SELECT TRUE");
    validate_optimized("SELECT 1 >= 1", "SELECT TRUE");
    validate_optimized("SELECT 2 <= 1", "SELECT FALSE");
    validate_optimized("SELECT 0 >= 1", "SELECT FALSE");
}

// ═════════════════════════════════════════════════════════════════════════════
// Boolean Simplification – AND / OR with TRUE / FALSE
// (from Python test_optimizer.py::test_simplify)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_simplify_and_true() {
    validate_optimized("SELECT TRUE AND x", "SELECT x");
    validate_optimized("SELECT x AND TRUE", "SELECT x");
}

#[test]
fn test_simplify_and_false() {
    validate_optimized("SELECT FALSE AND x", "SELECT FALSE");
    validate_optimized("SELECT x AND FALSE", "SELECT FALSE");
}

#[test]
fn test_simplify_or_true() {
    validate_optimized("SELECT TRUE OR x", "SELECT TRUE");
    validate_optimized("SELECT x OR TRUE", "SELECT TRUE");
}

#[test]
fn test_simplify_or_false() {
    validate_optimized("SELECT FALSE OR x", "SELECT x");
    validate_optimized("SELECT x OR FALSE", "SELECT x");
}

// ═════════════════════════════════════════════════════════════════════════════
// Boolean Simplification – NOT
// (from Python test_optimizer.py::test_simplify)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_simplify_double_not() {
    validate_optimized("SELECT NOT NOT x", "SELECT x");
}

// ═════════════════════════════════════════════════════════════════════════════
// Boolean Simplification – WHERE TRUE elimination
// (from Python test_optimizer.py::test_simplify)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_simplify_where_true() {
    validate_optimized("SELECT x FROM t WHERE TRUE", "SELECT x FROM t");
}

#[test]
fn test_simplify_where_true_and_condition() {
    validate_optimized(
        "SELECT x FROM t WHERE TRUE AND a > 1",
        "SELECT x FROM t WHERE a > 1",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// No-op cases – optimizer should leave these unchanged
// (from Python test_optimizer.py checking idempotence)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_no_op_plain_select() {
    validate_no_op("SELECT a FROM t WHERE a > 1");
}

#[test]
fn test_no_op_column_only() {
    validate_no_op("SELECT a + b FROM t");
}

#[test]
fn test_no_op_complex_query() {
    validate_no_op("SELECT a, b FROM t INNER JOIN t2 ON t.id = t2.id WHERE t.x > 0 ORDER BY a");
}

// ═════════════════════════════════════════════════════════════════════════════
// Combined constant folding + boolean simplification
// (from Python test_optimizer.py multi-pass tests)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_combined_fold_and_simplify() {
    // 1 + 2 folds to 3, 3 > 0 folds to TRUE, TRUE AND x → x
    validate_optimized("SELECT 1 + 2 > 0 AND x", "SELECT x");
}

#[test]
fn test_fold_in_where() {
    validate_optimized(
        "SELECT a FROM t WHERE 1 + 1 = 2",
        "SELECT a FROM t", // WHERE TRUE → removed
    );
}

#[test]
fn test_fold_leaves_non_const_untouched() {
    // Cannot fold a + 1 because a is a column reference
    validate_no_op("SELECT a + 1 FROM t");
}

// ═════════════════════════════════════════════════════════════════════════════
// Optimizer preserves correct structure
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_optimizer_preserves_joins() {
    validate_no_op("SELECT a.x FROM a INNER JOIN b ON a.id = b.id");
}

#[test]
fn test_optimizer_preserves_group_by() {
    validate_no_op("SELECT a, COUNT(*) FROM t GROUP BY a");
}

#[test]
fn test_optimizer_preserves_cte() {
    validate_no_op("WITH cte AS (SELECT 1 AS x) SELECT * FROM cte");
}

// ═════════════════════════════════════════════════════════════════════════════
// Subquery unnesting – EXISTS → JOIN (via full optimize pipeline)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_optimize_exists_to_inner_join() {
    let input = "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("INNER JOIN"),
        "optimize() should unnest EXISTS: {output}"
    );
    assert!(
        !output.contains("EXISTS"),
        "EXISTS should be removed: {output}"
    );
}

#[test]
fn test_optimize_not_exists_to_left_join() {
    let input = "SELECT a.id FROM a WHERE NOT EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("LEFT JOIN"),
        "optimize() should unnest NOT EXISTS: {output}"
    );
    assert!(
        output.contains("IS NULL"),
        "Anti-join needs IS NULL check: {output}"
    );
}

#[test]
fn test_optimize_in_subquery_to_join() {
    let input = "SELECT a.id FROM a WHERE a.id IN (SELECT b.id FROM b)";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("INNER JOIN"),
        "optimize() should unnest IN: {output}"
    );
    assert!(!output.contains(" IN "), "IN should be removed: {output}");
}

#[test]
fn test_optimize_not_in_subquery_to_left_join() {
    let input = "SELECT a.id FROM a WHERE a.id NOT IN (SELECT b.id FROM b)";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("LEFT JOIN"),
        "optimize() should unnest NOT IN: {output}"
    );
    assert!(
        output.contains("IS NULL"),
        "Anti-join needs IS NULL: {output}"
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Subquery unnesting – safety: uncorrelated and non-equality correlations
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_optimize_uncorrelated_exists_unchanged() {
    // No correlation → optimizer should leave EXISTS alone
    let input = "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.x > 10)";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("EXISTS"),
        "Uncorrelated EXISTS should remain: {output}"
    );
}

#[test]
fn test_optimize_non_eq_correlation_unchanged() {
    // Non-equality correlation → unsafe to unnest
    let input =
        "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.val < a.val AND b.id = a.id)";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("EXISTS"),
        "Non-eq correlation should remain: {output}"
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Python sqlglot#7295 exact reproducer through full pipeline
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_optimize_sqlglot_issue_7295_no_crash() {
    // Exact reproducer from Python sqlglot#7295: correlated scalar subquery
    // inside COALESCE in SELECT list with non-equality correlation.
    // Must NOT crash and must NOT modify the query structure.
    let input = "SELECT COALESCE((SELECT MAX(b.val) FROM t AS b WHERE b.val < a.val AND b.id = a.id), a.val) AS result FROM t AS a";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        !output.contains("JOIN"),
        "Issue #7295: should NOT add JOINs: {output}"
    );
    assert!(
        output.contains("COALESCE"),
        "COALESCE should remain: {output}"
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Combined optimizations: constant folding + boolean simplification + unnesting
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_optimize_combined_bool_simplify_and_unnest() {
    // TRUE AND EXISTS(...) → EXISTS is simplified to just EXISTS, then unnested
    let input = "SELECT a.id FROM a WHERE TRUE AND EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("INNER JOIN"),
        "Should unnest after boolean simplification: {output}"
    );
    assert!(
        !output.contains("TRUE"),
        "TRUE should be simplified away: {output}"
    );
    assert!(
        !output.contains("EXISTS"),
        "EXISTS should be unnested: {output}"
    );
}

#[test]
fn test_optimize_preserves_existing_joins_with_unnest() {
    // Query already has a join, and also has an EXISTS in WHERE → should add another join
    let input = "SELECT a.id FROM a INNER JOIN c ON a.cid = c.id WHERE EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("a.cid = c.id"),
        "Original join should be preserved: {output}"
    );
    assert!(
        !output.contains("EXISTS"),
        "EXISTS should be unnested: {output}"
    );
}

// ═══════════════════════════════════════════════════════════════════════
// qualify_columns tests
// ═══════════════════════════════════════════════════════════════════════

use sqlgrok::ast::DataType;
use sqlgrok::optimizer::qualify_columns::qualify_columns;
use sqlgrok::schema::{MappingSchema, Schema};

fn test_schema() -> MappingSchema {
    let mut s = MappingSchema::new(Dialect::Ansi);
    s.add_table(
        &["users"],
        vec![
            ("id".to_string(), DataType::Int),
            ("name".to_string(), DataType::Varchar(Some(255))),
            ("email".to_string(), DataType::Text),
            ("dept_id".to_string(), DataType::Int),
        ],
    )
    .unwrap();
    s.add_table(
        &["orders"],
        vec![
            ("id".to_string(), DataType::Int),
            ("user_id".to_string(), DataType::Int),
            (
                "amount".to_string(),
                DataType::Decimal {
                    precision: Some(10),
                    scale: Some(2),
                },
            ),
            ("status".to_string(), DataType::Varchar(Some(50))),
        ],
    )
    .unwrap();
    s.add_table(
        &["departments"],
        vec![
            ("id".to_string(), DataType::Int),
            ("dept_name".to_string(), DataType::Varchar(Some(100))),
        ],
    )
    .unwrap();
    s
}

fn qualify_sql(input: &str, schema: &MappingSchema) -> String {
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let qualified = qualify_columns(stmt, schema);
    generate(&qualified, Dialect::Ansi)
}

#[test]
fn test_qc_expand_star_single_table() {
    let s = test_schema();
    assert_eq!(
        qualify_sql("SELECT * FROM users", &s),
        "SELECT id, name, email, dept_id FROM users"
    );
}

#[test]
fn test_qc_expand_star_multi_table() {
    let s = test_schema();
    let result = qualify_sql(
        "SELECT * FROM users JOIN departments ON users.dept_id = departments.id",
        &s,
    );
    assert_eq!(
        result,
        "SELECT id, name, email, dept_id, id, dept_name FROM users JOIN departments ON users.dept_id = departments.id"
    );
}

#[test]
fn test_qc_expand_qualified_wildcard() {
    let s = test_schema();
    assert_eq!(
        qualify_sql("SELECT users.* FROM users", &s),
        "SELECT users.id, users.name, users.email, users.dept_id FROM users"
    );
}

#[test]
fn test_qc_qualify_single_source() {
    let s = test_schema();
    assert_eq!(
        qualify_sql("SELECT id, name FROM users WHERE email = 'a@b.com'", &s),
        "SELECT users.id, users.name FROM users WHERE users.email = 'a@b.com'"
    );
}

#[test]
fn test_qc_qualify_with_alias() {
    let s = test_schema();
    assert_eq!(
        qualify_sql("SELECT id, name FROM users AS u", &s),
        "SELECT u.id, u.name FROM users AS u"
    );
}

#[test]
fn test_qc_already_qualified_noop() {
    let s = test_schema();
    assert_eq!(
        qualify_sql("SELECT users.id FROM users", &s),
        "SELECT users.id FROM users"
    );
}

#[test]
fn test_qc_qualify_join_unique_columns() {
    let s = test_schema();
    assert_eq!(
        qualify_sql(
            "SELECT name, amount FROM users JOIN orders ON users.id = orders.user_id",
            &s
        ),
        "SELECT users.name, orders.amount FROM users JOIN orders ON users.id = orders.user_id"
    );
}

#[test]
fn test_qc_ambiguous_column_left_unqualified() {
    let s = test_schema();
    // 'id' exists in both — stays unqualified
    assert_eq!(
        qualify_sql(
            "SELECT id FROM users JOIN orders ON users.id = orders.user_id",
            &s
        ),
        "SELECT id FROM users JOIN orders ON users.id = orders.user_id"
    );
}

#[test]
fn test_qc_qualify_where_order_group_having() {
    let s = test_schema();
    assert_eq!(
        qualify_sql(
            "SELECT status, COUNT(*) FROM orders WHERE amount > 100 GROUP BY status HAVING COUNT(*) > 5 ORDER BY status",
            &s
        ),
        "SELECT orders.status, COUNT(*) FROM orders WHERE orders.amount > 100 GROUP BY orders.status HAVING COUNT(*) > 5 ORDER BY orders.status"
    );
}

#[test]
fn test_qc_cte_resolution() {
    let s = test_schema();
    assert_eq!(
        qualify_sql(
            "WITH active AS (SELECT id, name FROM users) SELECT id, name FROM active",
            &s
        ),
        "WITH active AS (SELECT users.id, users.name FROM users) SELECT active.id, active.name FROM active"
    );
}

#[test]
fn test_qc_derived_table() {
    let s = test_schema();
    assert_eq!(
        qualify_sql("SELECT sub.id FROM (SELECT id, name FROM users) AS sub", &s),
        "SELECT sub.id FROM (SELECT users.id, users.name FROM users) AS sub"
    );
}

#[test]
fn test_qc_preserve_aliases() {
    let s = test_schema();
    assert_eq!(
        qualify_sql("SELECT name AS user_name, email AS contact FROM users", &s),
        "SELECT users.name AS user_name, users.email AS contact FROM users"
    );
}

#[test]
fn test_qc_three_table_join() {
    let s = test_schema();
    assert_eq!(
        qualify_sql(
            "SELECT name, amount, dept_name FROM users JOIN orders ON users.id = orders.user_id JOIN departments ON users.dept_id = departments.id",
            &s
        ),
        "SELECT users.name, orders.amount, departments.dept_name FROM users JOIN orders ON users.id = orders.user_id JOIN departments ON users.dept_id = departments.id"
    );
}

#[test]
fn test_qc_unknown_table_passthrough() {
    let s = test_schema();
    // Table not in schema, columns pass through unchanged
    assert_eq!(
        qualify_sql("SELECT x, y FROM unknown_table", &s),
        "SELECT x, y FROM unknown_table"
    );
}

#[test]
fn test_qc_subquery_in_where() {
    let s = test_schema();
    // Columns in the inner subquery should get qualified too
    let result = qualify_sql(
        "SELECT name FROM users WHERE id IN (SELECT user_id FROM orders)",
        &s,
    );
    assert_eq!(
        result,
        "SELECT users.name FROM users WHERE users.id IN (SELECT orders.user_id FROM orders)"
    );
}

#[test]
fn test_qc_nested_cte_with_derived_table() {
    let s = test_schema();
    let result = qualify_sql(
        "WITH cte AS (SELECT id, name FROM users) SELECT id FROM (SELECT id FROM cte) AS sub",
        &s,
    );
    assert_eq!(
        result,
        "WITH cte AS (SELECT users.id, users.name FROM users) SELECT sub.id FROM (SELECT cte.id FROM cte) AS sub"
    );
}

#[test]
fn test_qc_non_select_passthrough() {
    let s = test_schema();
    // Non-SELECT statements pass through unchanged
    assert_eq!(
        qualify_sql("INSERT INTO users VALUES (1, 'a', 'b', 1)", &s),
        "INSERT INTO users VALUES (1, 'a', 'b', 1)"
    );
}

#[test]
fn test_qc_mixed_qualified_unqualified() {
    let s = test_schema();
    assert_eq!(
        qualify_sql("SELECT users.id, name FROM users", &s),
        "SELECT users.id, users.name FROM users"
    );
}

#[test]
fn test_qc_qualify_join_on_clause() {
    let s = test_schema();
    // user_id is unique to orders, gets qualified
    // id is ambiguous, stays unqualified
    assert_eq!(
        qualify_sql("SELECT name FROM users JOIN orders ON id = user_id", &s),
        "SELECT users.name FROM users JOIN orders ON id = orders.user_id"
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Predicate Pushdown – derived table pushdown
// (from Python test_optimizer.py::test_pushdown_predicates)
// ═════════════════════════════════════════════════════════════════════════════

use sqlgrok::pushdown_predicates;

/// Parse → pushdown_predicates → generate, compare with expected output.
fn validate_pushdown(input: &str, expected: &str) {
    let stmt = parse(input, Dialect::Ansi)
        .unwrap_or_else(|e| panic!("Parse failed for '{}': {}", input, e));
    let pushed = pushdown_predicates(stmt);
    let output = generate(&pushed, Dialect::Ansi);
    assert_eq!(output, expected, "\n  Input: {}", input);
}

/// Ensures the pushdown output equals the original (i.e. no-op).
fn validate_pushdown_no_op(sql: &str) {
    validate_pushdown(sql, sql);
}

// ── Push into derived tables ─────────────────────────────────────────

#[test]
fn test_pushdown_into_derived_table() {
    validate_pushdown(
        "SELECT sub.id FROM (SELECT id, name FROM t) AS sub WHERE sub.id > 5",
        "SELECT sub.id FROM (SELECT id, name FROM t WHERE id > 5) AS sub",
    );
}

#[test]
fn test_pushdown_into_derived_table_equality() {
    validate_pushdown(
        "SELECT sub.name FROM (SELECT id, name FROM t) AS sub WHERE sub.name = 'foo'",
        "SELECT sub.name FROM (SELECT id, name FROM t WHERE name = 'foo') AS sub",
    );
}

#[test]
fn test_pushdown_multiple_predicates_same_derived_table() {
    validate_pushdown(
        "SELECT sub.id FROM (SELECT id, name FROM t) AS sub WHERE sub.id > 5 AND sub.name = 'a'",
        "SELECT sub.id FROM (SELECT id, name FROM t WHERE id > 5 AND name = 'a') AS sub",
    );
}

#[test]
fn test_pushdown_derived_table_with_existing_where() {
    validate_pushdown(
        "SELECT sub.id FROM (SELECT id, name FROM t WHERE name <> 'del') AS sub WHERE sub.id > 5",
        "SELECT sub.id FROM (SELECT id, name FROM t WHERE name <> 'del' AND id > 5) AS sub",
    );
}

// ── Push into JOIN ON ────────────────────────────────────────────────

#[test]
fn test_pushdown_into_inner_join_on() {
    validate_pushdown(
        "SELECT a.id FROM a INNER JOIN b ON a.id = b.aid WHERE b.x > 10",
        "SELECT a.id FROM a INNER JOIN b ON a.id = b.aid AND b.x > 10",
    );
}

#[test]
fn test_pushdown_split_predicates_to_different_joins() {
    // b.y = 10 should go to the JOIN ON, a.x > 5 stays in WHERE
    // (a is the FROM table, not a join target)
    validate_pushdown(
        "SELECT a.id FROM a INNER JOIN b ON a.id = b.aid WHERE a.x > 5 AND b.y = 10",
        "SELECT a.id FROM a INNER JOIN b ON a.id = b.aid AND b.y = 10 WHERE a.x > 5",
    );
}

// ── Safety: no push through LIMIT ────────────────────────────────────

#[test]
fn test_pushdown_blocked_by_limit() {
    validate_pushdown_no_op(
        "SELECT sub.id FROM (SELECT id FROM t LIMIT 10) AS sub WHERE sub.id > 5",
    );
}

// ── Safety: no push through OFFSET ───────────────────────────────────

#[test]
fn test_pushdown_blocked_by_offset() {
    validate_pushdown_no_op(
        "SELECT sub.id FROM (SELECT id FROM t OFFSET 5) AS sub WHERE sub.id > 5",
    );
}

// ── Safety: no push through DISTINCT ─────────────────────────────────

#[test]
fn test_pushdown_blocked_by_distinct() {
    validate_pushdown_no_op(
        "SELECT sub.id FROM (SELECT DISTINCT id FROM t) AS sub WHERE sub.id > 5",
    );
}

// ── Safety: no push into LEFT/RIGHT/FULL JOINs ──────────────────────

#[test]
fn test_pushdown_blocked_by_left_join() {
    validate_pushdown_no_op("SELECT a.id FROM a LEFT JOIN b ON a.id = b.aid WHERE b.x > 10");
}

#[test]
fn test_pushdown_blocked_by_right_join() {
    validate_pushdown_no_op("SELECT a.id FROM a RIGHT JOIN b ON a.id = b.aid WHERE b.x > 10");
}

#[test]
fn test_pushdown_blocked_by_full_join() {
    validate_pushdown_no_op("SELECT a.id FROM a FULL JOIN b ON a.id = b.aid WHERE b.x > 10");
}

// ── Safety: aggregate predicates not pushed ──────────────────────────

#[test]
fn test_pushdown_blocked_by_aggregate_in_predicate() {
    validate_pushdown_no_op("SELECT sub.x FROM (SELECT x FROM t) AS sub WHERE COUNT(*) > 5");
}

// ── Safety: window function predicates not pushed ────────────────────

#[test]
fn test_pushdown_blocked_by_window_in_predicate() {
    validate_pushdown_no_op(
        "SELECT sub.x FROM (SELECT x FROM t) AS sub WHERE ROW_NUMBER() OVER () > 1",
    );
}

// ── Safety: subquery predicates not pushed ───────────────────────────

#[test]
fn test_pushdown_blocked_by_subquery_in_predicate() {
    validate_pushdown_no_op(
        "SELECT sub.x FROM (SELECT x FROM t) AS sub WHERE sub.x IN (SELECT y FROM t2)",
    );
}

// ── Safety: cross-table predicates not pushed ────────────────────────

#[test]
fn test_pushdown_blocked_by_cross_table_predicate() {
    validate_pushdown_no_op("SELECT a.id FROM a INNER JOIN b ON a.id = b.aid WHERE a.x = b.y");
}

// ── Safety: unqualified columns stay unchanged ───────────────────────

#[test]
fn test_pushdown_unqualified_columns_no_op() {
    validate_pushdown_no_op("SELECT id FROM t WHERE id > 5");
}

// ── Safety: no push through window functions in derived table ────────

#[test]
fn test_pushdown_blocked_by_window_in_derived_table() {
    validate_pushdown_no_op(
        "SELECT sub.rn FROM (SELECT ROW_NUMBER() OVER (ORDER BY id) AS rn FROM t) AS sub WHERE sub.rn > 5",
    );
}

// ── No-op: plain query without derived tables or joins ───────────────

#[test]
fn test_pushdown_plain_query_no_op() {
    validate_pushdown_no_op("SELECT a, b FROM t WHERE a > 1");
}

// ── Integration: pushdown through full optimize pipeline ─────────────

#[test]
fn test_optimize_with_pushdown_derived_table() {
    let input = "SELECT sub.id FROM (SELECT id, name FROM t) AS sub WHERE sub.id > 5";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("WHERE id > 5) AS sub"),
        "Predicate should be pushed into derived table: {output}"
    );
    assert!(
        !output.contains("WHERE sub.id > 5"),
        "Outer WHERE should be removed: {output}"
    );
}

#[test]
fn test_optimize_with_pushdown_join_on() {
    let input = "SELECT a.id FROM a INNER JOIN b ON a.id = b.aid WHERE b.x > 10";
    let stmt = parse(input, Dialect::Ansi).unwrap();
    let optimized = optimize(stmt).unwrap();
    let output = generate(&optimized, Dialect::Ansi);
    assert!(
        output.contains("a.id = b.aid AND b.x > 10"),
        "Predicate should be moved to JOIN ON: {output}"
    );
}

// ── AND conjunction splitting ────────────────────────────────────────

#[test]
fn test_pushdown_and_splitting_partial() {
    // One predicate pushable (b.y = 10 → JOIN), one not (literal stays)
    validate_pushdown(
        "SELECT a.id FROM a INNER JOIN b ON a.id = b.aid WHERE b.y = 10 AND 1 = 1",
        "SELECT a.id FROM a INNER JOIN b ON a.id = b.aid AND b.y = 10 WHERE 1 = 1",
    );
}

// ── Non-deterministic function ───────────────────────────────────────

#[test]
fn test_pushdown_blocked_by_nondeterministic() {
    validate_pushdown_no_op("SELECT sub.x FROM (SELECT x FROM t) AS sub WHERE RANDOM() > 0.5");
}

// ── Non-SELECT statements pass through ───────────────────────────────

#[test]
fn test_pushdown_non_select_passthrough() {
    validate_pushdown(
        "INSERT INTO users VALUES (1, 'a', 'b', 1)",
        "INSERT INTO users VALUES (1, 'a', 'b', 1)",
    );
}
