use sqlglot_rust::optimizer::optimize;
/// Tests ported from Python sqlglot's `test_optimizer.py`.
///
/// Covers constant folding (arithmetic, string concat, comparisons),
/// boolean simplification (AND/OR with TRUE/FALSE, double NOT, WHERE TRUE),
/// and subquery unnesting / decorrelation (EXISTS/IN → JOINs).
use sqlglot_rust::{Dialect, generate, parse};

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
