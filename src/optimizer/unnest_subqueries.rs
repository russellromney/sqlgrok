/// Unnest and decorrelate subqueries.
///
/// Transforms correlated subqueries in WHERE clauses into equivalent JOINs
/// when safe to do so. This is a common optimization because most query engines
/// execute joins more efficiently than correlated subqueries.
///
/// ## Supported rewrites
///
/// | Pattern                               | Rewrite                             |
/// |---------------------------------------|-------------------------------------|
/// | `WHERE EXISTS (SELECT … WHERE a = b)` | `INNER JOIN (SELECT …) ON a = b`   |
/// | `WHERE NOT EXISTS (…)`                | `LEFT JOIN … WHERE _u.col IS NULL`  |
/// | `WHERE x IN (SELECT col FROM …)`      | `INNER JOIN (SELECT DISTINCT …)`    |
/// | `WHERE x NOT IN (SELECT col FROM …)`  | `LEFT JOIN … WHERE _u.col IS NULL`  |
///
/// ## Safety
///
/// The pass bails out (leaves the subquery unchanged) when:
/// - The subquery has no correlation predicates (equality on outer columns).
/// - The subquery is inside a function/expression in the SELECT list rather
///   than in a WHERE predicate — matching the fix for Python sqlglot#7295.
/// - The correlated predicate involves non-equality operators that would
///   require LATERAL / APPLY for correctness.
use crate::ast::*;

/// Apply subquery unnesting to a statement.
///
/// Returns the statement unchanged if no subqueries can be safely unnested.
pub fn unnest_subqueries(statement: Statement) -> Statement {
    match statement {
        Statement::Select(sel) => Statement::Select(unnest_select(sel)),
        other => other,
    }
}

/// Counter for generating unique aliases for unnested subquery tables.
struct AliasGen {
    counter: usize,
}

impl AliasGen {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn next(&mut self) -> String {
        let alias = format!("_u{}", self.counter);
        self.counter += 1;
        alias
    }
}

fn unnest_select(mut sel: SelectStatement) -> SelectStatement {
    let mut alias_gen = AliasGen::new();

    // Only unnest subqueries found in the WHERE clause (safe position).
    // Subqueries in SELECT columns that are inside functions (COALESCE, etc.)
    // are NOT unnested to avoid the Python sqlglot#7295 crash scenario.
    if let Some(where_clause) = sel.where_clause.take() {
        let (new_where, new_joins) = unnest_where(where_clause, &mut alias_gen);
        sel.where_clause = new_where;
        sel.joins.extend(new_joins);
    }

    sel
}

/// Process a WHERE clause expression, extracting subqueries that can be
/// rewritten as joins. Returns the residual WHERE clause (if any) and
/// a list of new JoinClauses.
fn unnest_where(expr: Expr, alias_gen: &mut AliasGen) -> (Option<Expr>, Vec<JoinClause>) {
    let mut joins = Vec::new();
    let residual = unnest_expr(expr, &mut joins, alias_gen);
    (residual, joins)
}

/// Recursively process an expression in the WHERE clause.
/// Returns `None` if the expression was fully consumed (replaced by a join).
fn unnest_expr(expr: Expr, joins: &mut Vec<JoinClause>, alias_gen: &mut AliasGen) -> Option<Expr> {
    match expr {
        // AND: try to unnest each side independently
        Expr::BinaryOp {
            left,
            op: BinaryOperator::And,
            right,
        } => {
            let left_result = unnest_expr(*left, joins, alias_gen);
            let right_result = unnest_expr(*right, joins, alias_gen);
            match (left_result, right_result) {
                (Some(l), Some(r)) => Some(Expr::BinaryOp {
                    left: Box::new(l),
                    op: BinaryOperator::And,
                    right: Box::new(r),
                }),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        }

        // EXISTS (SELECT ... WHERE outer.col = inner.col)
        Expr::Exists { subquery, negated } => {
            let subquery_inner = *subquery;
            if let Some((join, residual)) =
                try_unnest_exists(subquery_inner.clone(), negated, alias_gen)
            {
                joins.push(join);
                residual
            } else {
                Some(Expr::Exists {
                    subquery: Box::new(subquery_inner),
                    negated,
                })
            }
        }

        // NOT EXISTS parsed as UnaryOp(Not, Exists { negated: false })
        Expr::UnaryOp {
            op: UnaryOperator::Not,
            expr,
        } if matches!(expr.as_ref(), Expr::Exists { negated: false, .. }) => {
            if let Expr::Exists { subquery, .. } = *expr {
                let subquery_inner = *subquery;
                if let Some((join, residual)) =
                    try_unnest_exists(subquery_inner.clone(), true, alias_gen)
                {
                    joins.push(join);
                    residual
                } else {
                    Some(Expr::UnaryOp {
                        op: UnaryOperator::Not,
                        expr: Box::new(Expr::Exists {
                            subquery: Box::new(subquery_inner),
                            negated: false,
                        }),
                    })
                }
            } else {
                unreachable!()
            }
        }

        // col IN (SELECT ...)
        Expr::InSubquery {
            expr: lhs,
            subquery,
            negated,
        } => {
            let lhs_inner = *lhs;
            let subquery_inner = *subquery;
            if let Some((join, residual)) = try_unnest_in_subquery(
                lhs_inner.clone(),
                subquery_inner.clone(),
                negated,
                alias_gen,
            ) {
                joins.push(join);
                residual
            } else {
                Some(Expr::InSubquery {
                    expr: Box::new(lhs_inner),
                    subquery: Box::new(subquery_inner),
                    negated,
                })
            }
        }

        // Nested parenthesized expression — unwrap, try to unnest, re-wrap if needed
        Expr::Nested(inner) => {
            let result = unnest_expr(*inner, joins, alias_gen);
            result.map(|e| {
                if e.is_literal() || matches!(e, Expr::Column { .. }) {
                    e
                } else {
                    Expr::Nested(Box::new(e))
                }
            })
        }

        // Everything else: leave as-is
        other => Some(other),
    }
}

// ─────────────────────────────────────────────────────────────────────
// EXISTS / NOT EXISTS → JOIN
// ─────────────────────────────────────────────────────────────────────

/// Try to rewrite `[NOT] EXISTS (SELECT ... FROM t WHERE correlations)`
/// into a join. Returns `None` if the subquery cannot be safely unnested.
fn try_unnest_exists(
    subquery: Statement,
    negated: bool,
    alias_gen: &mut AliasGen,
) -> Option<(JoinClause, Option<Expr>)> {
    let inner_select = match &subquery {
        Statement::Select(sel) => sel,
        _ => return None,
    };

    // Extract correlation predicates from the inner WHERE clause
    let inner_where = inner_select.where_clause.as_ref()?;
    let (eq_preds, non_eq_preds) = extract_correlation_predicates(inner_where);

    // Must have at least one equality correlation
    if eq_preds.is_empty() {
        return None;
    }

    // Bail out if there are non-equality correlations (would need LATERAL)
    if !non_eq_preds.is_empty() {
        return None;
    }

    let alias = alias_gen.next();

    // Build the ON condition from equality predicates
    let on_condition = build_join_on(&eq_preds, &alias);

    // Build the derived table from the inner SELECT, stripping correlation predicates
    let derived = build_derived_table_from_exists(subquery, &eq_preds, &alias);

    if negated {
        // NOT EXISTS → LEFT JOIN ... WHERE _u.col IS NULL
        // Pick any column from the inner select as the NULL-check sentinel
        let null_check_col = sentinel_column(&alias);

        let join = JoinClause {
            join_type: JoinType::Left,
            table: derived,
            on: Some(on_condition),
            using: vec![],
        };
        let residual = Some(Expr::IsNull {
            expr: Box::new(null_check_col),
            negated: false,
        });
        Some((join, residual))
    } else {
        // EXISTS → INNER JOIN (deduplicated via GROUP BY or DISTINCT in the derived table)
        let join = JoinClause {
            join_type: JoinType::Inner,
            table: derived,
            on: Some(on_condition),
            using: vec![],
        };
        Some((join, None))
    }
}

// ─────────────────────────────────────────────────────────────────────
// IN / NOT IN subquery → JOIN
// ─────────────────────────────────────────────────────────────────────

/// Try to rewrite `col [NOT] IN (SELECT x FROM t WHERE ...)` into a join.
fn try_unnest_in_subquery(
    lhs: Expr,
    subquery: Statement,
    negated: bool,
    alias_gen: &mut AliasGen,
) -> Option<(JoinClause, Option<Expr>)> {
    let inner_select = match &subquery {
        Statement::Select(sel) => sel,
        _ => return None,
    };

    // The inner SELECT must project exactly one column
    if inner_select.columns.len() != 1 {
        return None;
    }

    let alias = alias_gen.next();
    let inner_col_alias = "_col0".to_string();

    // Build ON condition: outer_col = _u._col0
    let on_condition = Expr::BinaryOp {
        left: Box::new(lhs),
        op: BinaryOperator::Eq,
        right: Box::new(Expr::Column {
            table: Some(alias.clone()),
            name: inner_col_alias.clone(),
            quote_style: QuoteStyle::None,
            table_quote_style: QuoteStyle::None,
        }),
    };

    // Build a derived table: (SELECT DISTINCT <col> AS _col0 FROM ... WHERE ...) AS _uN
    let derived = build_derived_table_from_in(subquery, &inner_col_alias, &alias);

    if negated {
        // NOT IN → LEFT JOIN ... WHERE _uN._col0 IS NULL
        let null_check = Expr::IsNull {
            expr: Box::new(Expr::Column {
                table: Some(alias.clone()),
                name: inner_col_alias,
                quote_style: QuoteStyle::None,
                table_quote_style: QuoteStyle::None,
            }),
            negated: false,
        };

        let join = JoinClause {
            join_type: JoinType::Left,
            table: derived,
            on: Some(on_condition),
            using: vec![],
        };
        Some((join, Some(null_check)))
    } else {
        // IN → INNER JOIN
        let join = JoinClause {
            join_type: JoinType::Inner,
            table: derived,
            on: Some(on_condition),
            using: vec![],
        };
        Some((join, None))
    }
}

// ─────────────────────────────────────────────────────────────────────
// Helpers: correlation detection
// ─────────────────────────────────────────────────────────────────────

/// A correlation predicate extracted from a subquery's WHERE clause.
/// Represents `outer_col = inner_col`.
#[derive(Debug, Clone)]
struct CorrelationPredicate {
    /// The column reference from the outer query
    outer_col: Expr,
    /// The column reference from the inner query
    inner_col: Expr,
}

/// Examine a WHERE expression and extract equality correlation predicates
/// (where one side references an outer table and the other an inner table).
///
/// Returns (equality_correlations, non_equality_predicates_referencing_outer_tables).
///
/// A predicate is classified as "correlated" if it contains column references
/// with table qualifiers that likely come from the outer scope. Since we don't
/// have full scope analysis, we use a heuristic: columns with a table qualifier
/// that appears as a correlation candidate.
fn extract_correlation_predicates(expr: &Expr) -> (Vec<CorrelationPredicate>, Vec<Expr>) {
    let mut eq_preds = Vec::new();
    let mut non_eq_preds = Vec::new();

    collect_correlation_predicates(expr, &mut eq_preds, &mut non_eq_preds);

    (eq_preds, non_eq_preds)
}

fn collect_correlation_predicates(
    expr: &Expr,
    eq_preds: &mut Vec<CorrelationPredicate>,
    non_eq_preds: &mut Vec<Expr>,
) {
    match expr {
        Expr::BinaryOp {
            left,
            op: BinaryOperator::And,
            right,
        } => {
            collect_correlation_predicates(left, eq_preds, non_eq_preds);
            collect_correlation_predicates(right, eq_preds, non_eq_preds);
        }

        Expr::BinaryOp {
            left,
            op: BinaryOperator::Eq,
            right,
        } => {
            // Check if this is a correlation: one side has a table qualifier,
            // and they reference different tables
            if let (Some((l_table, _l_name)), Some((r_table, _r_name))) =
                (extract_column_ref(left), extract_column_ref(right))
            {
                if l_table == r_table {
                    // Same table — not a correlation
                } else {
                    eq_preds.push(CorrelationPredicate {
                        outer_col: *left.clone(),
                        inner_col: *right.clone(),
                    });
                    return;
                }
            }
            // Not a correlation — check if it references outer tables
            if has_potential_outer_reference(expr) {
                non_eq_preds.push(expr.clone());
            }
        }

        // Non-equality comparisons that reference outer columns
        Expr::BinaryOp {
            op:
                BinaryOperator::Lt
                | BinaryOperator::Gt
                | BinaryOperator::LtEq
                | BinaryOperator::GtEq
                | BinaryOperator::Neq,
            ..
        } => {
            if is_cross_table_predicate(expr) {
                non_eq_preds.push(expr.clone());
            }
        }

        _ => {}
    }
}

/// Extract (table, column_name) from a column reference if it has a table qualifier.
fn extract_column_ref(expr: &Expr) -> Option<(String, String)> {
    match expr {
        Expr::Column {
            table: Some(t),
            name,
            ..
        } => Some((t.clone(), name.clone())),
        _ => None,
    }
}

/// Check whether an expression contains column references from different tables
/// (heuristic for cross-scope references without full scope analysis).
fn is_cross_table_predicate(expr: &Expr) -> bool {
    let mut tables = Vec::new();
    expr.walk(&mut |e| {
        if let Expr::Column { table: Some(t), .. } = e
            && !tables.iter().any(|existing: &String| existing == t)
        {
            tables.push(t.clone());
        }
        true
    });
    tables.len() > 1
}

/// Heuristic: does this expression reference columns from more than one table?
fn has_potential_outer_reference(expr: &Expr) -> bool {
    is_cross_table_predicate(expr)
}

// ─────────────────────────────────────────────────────────────────────
// Helpers: building derived tables and join conditions
// ─────────────────────────────────────────────────────────────────────

/// Build a JOIN ON clause from equality correlation predicates,
/// rewriting inner column references to use the new alias.
fn build_join_on(preds: &[CorrelationPredicate], alias: &str) -> Expr {
    let conditions: Vec<Expr> = preds
        .iter()
        .map(|p| {
            let rewritten_inner = rewrite_column_table(&p.inner_col, alias);
            Expr::BinaryOp {
                left: Box::new(p.outer_col.clone()),
                op: BinaryOperator::Eq,
                right: Box::new(rewritten_inner),
            }
        })
        .collect();

    and_all(conditions)
}

/// Combine multiple expressions with AND.
fn and_all(mut exprs: Vec<Expr>) -> Expr {
    assert!(
        !exprs.is_empty(),
        "and_all requires at least one expression"
    );
    if exprs.len() == 1 {
        return exprs.remove(0);
    }
    let first = exprs.remove(0);
    exprs.into_iter().fold(first, |acc, e| Expr::BinaryOp {
        left: Box::new(acc),
        op: BinaryOperator::And,
        right: Box::new(e),
    })
}

/// Rewrite a column's table qualifier to the given alias.
fn rewrite_column_table(expr: &Expr, new_table: &str) -> Expr {
    match expr {
        Expr::Column {
            name, quote_style, ..
        } => Expr::Column {
            table: Some(new_table.to_string()),
            name: name.clone(),
            quote_style: *quote_style,
            table_quote_style: QuoteStyle::None,
        },
        other => other.clone(),
    }
}

/// Build a derived table source from an EXISTS subquery.
///
/// Strips the correlation predicates from the inner WHERE clause and wraps
/// the result as `(SELECT DISTINCT 1 AS _sentinel, <join_keys> FROM ... WHERE <residual>) AS _uN`.
fn build_derived_table_from_exists(
    subquery: Statement,
    eq_preds: &[CorrelationPredicate],
    alias: &str,
) -> TableSource {
    let mut inner_select = match subquery {
        Statement::Select(sel) => sel,
        _ => unreachable!("Caller ensures this is a SELECT"),
    };

    // Remove correlation predicates from the inner WHERE
    if let Some(where_clause) = inner_select.where_clause.take() {
        inner_select.where_clause = strip_correlation_predicates(where_clause, eq_preds);
    }

    // Replace the SELECT list with DISTINCT 1 AS _sentinel
    // to deduplicate rows for EXISTS semantics
    inner_select.distinct = true;
    inner_select.columns = vec![SelectItem::Expr {
        expr: Expr::Number("1".to_string()),
        alias: Some("_sentinel".to_string()),
        alias_quote_style: QuoteStyle::None,
    }];

    TableSource::Subquery {
        query: Box::new(Statement::Select(inner_select)),
        alias: Some(alias.to_string()),
        alias_quote_style: QuoteStyle::None,
    }
}

/// Build a derived table from an IN-subquery.
///
/// Wraps the inner query as `(SELECT DISTINCT <col> AS _col0 FROM ...) AS _uN`.
fn build_derived_table_from_in(
    subquery: Statement,
    col_alias: &str,
    table_alias: &str,
) -> TableSource {
    let mut inner_select = match subquery {
        Statement::Select(sel) => sel,
        _ => unreachable!("Caller ensures this is a SELECT"),
    };

    // Ensure DISTINCT to avoid row multiplication
    inner_select.distinct = true;

    // Alias the single projected column
    if let Some(SelectItem::Expr { alias, .. }) = inner_select.columns.first_mut() {
        *alias = Some(col_alias.to_string());
    }

    TableSource::Subquery {
        query: Box::new(Statement::Select(inner_select)),
        alias: Some(table_alias.to_string()),
        alias_quote_style: QuoteStyle::None,
    }
}

/// Create a sentinel column reference for NULL-checks in anti-joins.
fn sentinel_column(alias: &str) -> Expr {
    Expr::Column {
        table: Some(alias.to_string()),
        name: "_sentinel".to_string(),
        quote_style: QuoteStyle::None,
        table_quote_style: QuoteStyle::None,
    }
}

/// Strip the given correlation predicates from a WHERE expression,
/// returning the residual expression (or None if the entire WHERE was correlations).
fn strip_correlation_predicates(expr: Expr, eq_preds: &[CorrelationPredicate]) -> Option<Expr> {
    match expr {
        Expr::BinaryOp {
            left,
            op: BinaryOperator::And,
            right,
        } => {
            let left_result = strip_correlation_predicates(*left, eq_preds);
            let right_result = strip_correlation_predicates(*right, eq_preds);
            match (left_result, right_result) {
                (Some(l), Some(r)) => Some(Expr::BinaryOp {
                    left: Box::new(l),
                    op: BinaryOperator::And,
                    right: Box::new(r),
                }),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        }

        Expr::BinaryOp {
            ref left,
            op: BinaryOperator::Eq,
            ref right,
        } => {
            // Check if this matches any of the correlation predicates
            for pred in eq_preds {
                if (*left.as_ref() == pred.outer_col && *right.as_ref() == pred.inner_col)
                    || (*left.as_ref() == pred.inner_col && *right.as_ref() == pred.outer_col)
                {
                    return None; // Strip this predicate
                }
            }
            Some(expr)
        }

        other => Some(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialects::Dialect;
    use crate::generator::generate;
    use crate::parser::Parser;

    fn parse_and_unnest(sql: &str) -> String {
        let stmt = Parser::new(sql).unwrap().parse_statement().unwrap();
        let unnested = unnest_subqueries(stmt);
        generate(&unnested, Dialect::Ansi)
    }

    #[test]
    fn test_exists_to_inner_join() {
        let sql = "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
        let result = parse_and_unnest(sql);
        // Should be rewritten to an INNER JOIN
        assert!(
            result.contains("INNER JOIN"),
            "Expected INNER JOIN in: {result}"
        );
        assert!(
            result.contains("_u0"),
            "Expected derived alias _u0 in: {result}"
        );
        assert!(
            !result.contains("EXISTS"),
            "Should not contain EXISTS: {result}"
        );
    }

    #[test]
    fn test_not_exists_to_left_join() {
        let sql = "SELECT a.id FROM a WHERE NOT EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("LEFT JOIN"),
            "Expected LEFT JOIN in: {result}"
        );
        assert!(
            result.contains("IS NULL"),
            "Expected IS NULL check in: {result}"
        );
        assert!(
            !result.contains("NOT EXISTS"),
            "Should not contain NOT EXISTS: {result}"
        );
    }

    #[test]
    fn test_in_subquery_to_inner_join() {
        let sql = "SELECT a.id FROM a WHERE a.id IN (SELECT b.id FROM b)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("INNER JOIN"),
            "Expected INNER JOIN in: {result}"
        );
        assert!(!result.contains(" IN "), "Should not contain IN: {result}");
    }

    #[test]
    fn test_not_in_subquery_to_left_join() {
        let sql = "SELECT a.id FROM a WHERE a.id NOT IN (SELECT b.id FROM b)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("LEFT JOIN"),
            "Expected LEFT JOIN in: {result}"
        );
        assert!(
            result.contains("IS NULL"),
            "Expected IS NULL check in: {result}"
        );
    }

    #[test]
    fn test_no_correlation_not_unnested() {
        // Subquery without correlation should not be unnested
        let sql = "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.x > 10)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("EXISTS"),
            "Uncorrelated EXISTS should remain: {result}"
        );
    }

    #[test]
    fn test_non_equality_correlation_not_unnested() {
        // Subquery with non-equality correlation (< instead of =) should NOT be unnested
        // This is the scenario described in Python sqlglot#7295
        let sql =
            "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.val < a.val AND b.id = a.id)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("EXISTS"),
            "Subquery with non-eq correlation should not be unnested: {result}"
        );
    }

    #[test]
    fn test_subquery_in_select_not_unnested() {
        // Correlated scalar subquery inside COALESCE in SELECT list
        // This is the exact scenario from Python sqlglot#7295 — must NOT be touched
        let sql =
            "SELECT COALESCE((SELECT MAX(b.val) FROM b WHERE b.id = a.id), a.val) AS result FROM a";
        let result = parse_and_unnest(sql);
        // The subquery should remain in the SELECT, NOT be moved to a JOIN
        assert!(
            !result.contains("JOIN"),
            "Subquery in SELECT should not become a JOIN: {result}"
        );
    }

    #[test]
    fn test_exists_with_additional_where() {
        // EXISTS with both correlation and a local predicate
        let sql = "SELECT a.id FROM a WHERE a.x > 5 AND EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("INNER JOIN"),
            "Expected INNER JOIN in: {result}"
        );
        assert!(
            result.contains("a.x > 5") || result.contains("a.x >"),
            "Should keep non-subquery predicate: {result}"
        );
    }

    #[test]
    fn test_non_select_statement_unchanged() {
        let sql = "INSERT INTO t (a) VALUES (1)";
        let stmt = Parser::new(sql).unwrap().parse_statement().unwrap();
        let result = unnest_subqueries(stmt.clone());
        assert_eq!(
            format!("{result:?}"),
            format!("{stmt:?}"),
            "Non-SELECT statements should pass through unchanged"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // Multiple correlation predicates
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_exists_multiple_correlations() {
        // Two equality correlations → JOIN ON with AND
        let sql =
            "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.id = a.id AND b.org = a.org)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("INNER JOIN"),
            "Expected INNER JOIN in: {result}"
        );
        assert!(
            !result.contains("EXISTS"),
            "Should not contain EXISTS: {result}"
        );
        // The ON clause must have both correlation columns joined with AND
        assert!(
            result.contains(" AND "),
            "ON clause should have AND for multiple correlations: {result}"
        );
        assert!(result.contains(".id"), "ON should reference id: {result}");
        assert!(result.contains(".org"), "ON should reference org: {result}");
    }

    // ─────────────────────────────────────────────────────────────────
    // Multiple subqueries in WHERE
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_multiple_subqueries_in_where() {
        // Two different subqueries in the same WHERE, both should be unnested
        let sql = "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.id = a.id) AND a.id IN (SELECT c.id FROM c)";
        let result = parse_and_unnest(sql);
        // Both should become joins
        assert!(
            !result.contains("EXISTS"),
            "EXISTS should be unnested: {result}"
        );
        assert!(!result.contains(" IN "), "IN should be unnested: {result}");
        // Should have two joins with different aliases
        assert!(result.contains("_u0"), "Expected first alias _u0: {result}");
        assert!(
            result.contains("_u1"),
            "Expected second alias _u1: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // Residual inner WHERE after stripping correlations
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_exists_with_inner_residual_where() {
        // Inner subquery has a correlation AND a local (non-correlated) predicate
        // The local predicate should remain in the derived table's WHERE
        let sql =
            "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.id = a.id AND b.active = 1)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("INNER JOIN"),
            "Expected INNER JOIN in: {result}"
        );
        assert!(
            !result.contains("EXISTS"),
            "Should not contain EXISTS: {result}"
        );
        // The non-correlated predicate b.active = 1 should be preserved in the subquery
        assert!(
            result.contains("active") && result.contains("1"),
            "Inner residual WHERE should be preserved: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // Nested parenthesized subqueries
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_parenthesized_exists() {
        // EXISTS wrapped in extra parentheses
        let sql = "SELECT a.id FROM a WHERE (EXISTS (SELECT 1 FROM b WHERE b.id = a.id))";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("INNER JOIN"),
            "Expected INNER JOIN in: {result}"
        );
        assert!(
            !result.contains("EXISTS"),
            "Should not contain EXISTS: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // IN subquery with multiple projected columns (bail out)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_in_subquery_multi_column_not_unnested() {
        // IN (SELECT col1, col2 ...) — multi-column, should NOT be unnested
        let sql = "SELECT a.id FROM a WHERE a.id IN (SELECT b.id, b.name FROM b)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains(" IN "),
            "Multi-column IN should remain: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // OR with subqueries (should NOT unnest — not safe)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_or_with_exists_not_unnested() {
        // EXISTS in an OR branch — cannot safely unnest
        let sql = "SELECT a.id FROM a WHERE a.x > 1 OR EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("EXISTS"),
            "EXISTS in OR should remain: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // Scalar subquery in WHERE (not EXISTS / IN — should stay)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_scalar_subquery_in_where_not_unnested() {
        // WHERE col = (SELECT ...) — scalar subquery comparison, not handled
        let sql = "SELECT a.id FROM a WHERE a.val = (SELECT MAX(b.val) FROM b WHERE b.id = a.id)";
        let result = parse_and_unnest(sql);
        assert!(
            !result.contains("JOIN"),
            "Scalar subquery in WHERE should not become JOIN: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // Exact reproducer from Python sqlglot#7295
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_sqlglot_issue_7295_exact_reproducer() {
        // This is the exact SQL from Python sqlglot issue #7295.
        // The correlated subquery is inside COALESCE in the SELECT list
        // with a non-equality correlation (b.val < a.val).
        // Must NOT crash and must NOT modify the query.
        let sql = "SELECT COALESCE((SELECT MAX(b.val) FROM t AS b WHERE b.val < a.val AND b.id = a.id), a.val) AS result FROM t AS a";
        let result = parse_and_unnest(sql);
        assert!(
            !result.contains("JOIN"),
            "Issue #7295 query must NOT be rewritten to JOIN: {result}"
        );
        assert!(
            result.contains("COALESCE"),
            "COALESCE should remain: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // No WHERE clause at all
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_no_where_clause_unchanged() {
        let sql = "SELECT a.id FROM a";
        let result = parse_and_unnest(sql);
        assert_eq!(result, "SELECT a.id FROM a", "No WHERE should be unchanged");
    }

    // ─────────────────────────────────────────────────────────────────
    // WHERE with no subqueries
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_where_without_subqueries_unchanged() {
        let sql = "SELECT a.id FROM a WHERE a.x > 1 AND a.y = 2";
        let result = parse_and_unnest(sql);
        assert!(
            !result.contains("JOIN"),
            "No subqueries, no joins should be added: {result}"
        );
        assert!(
            result.contains("a.x > 1"),
            "Original predicates should remain: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // EXISTS with no inner WHERE (uncorrelated — bail out)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_exists_no_where_not_unnested() {
        let sql = "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("EXISTS"),
            "EXISTS without inner WHERE should remain: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // Inner subquery has only same-table predicates (no cross-table correlation)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_exists_same_table_predicate_not_unnested() {
        // Inner WHERE only references columns from b — no correlation to outer a
        let sql = "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.x = b.y)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("EXISTS"),
            "Same-table predicate is not correlation: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // DISTINCT behaviour in derived tables
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_exists_produces_distinct_derived_table() {
        let sql = "SELECT a.id FROM a WHERE EXISTS (SELECT 1 FROM b WHERE b.id = a.id)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("DISTINCT"),
            "Derived table should use DISTINCT: {result}"
        );
    }

    #[test]
    fn test_in_produces_distinct_derived_table() {
        let sql = "SELECT a.id FROM a WHERE a.id IN (SELECT b.id FROM b)";
        let result = parse_and_unnest(sql);
        assert!(
            result.contains("DISTINCT"),
            "IN-derived table should use DISTINCT: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // NOT IN with inner WHERE (should propagate inner WHERE into derived table)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_not_in_preserves_inner_where() {
        let sql = "SELECT a.id FROM a WHERE a.id NOT IN (SELECT b.id FROM b WHERE b.active = 1)";
        let result = parse_and_unnest(sql);
        assert!(result.contains("LEFT JOIN"), "Expected LEFT JOIN: {result}");
        assert!(result.contains("IS NULL"), "Expected IS NULL: {result}");
        assert!(
            result.contains("active"),
            "Inner WHERE should be preserved: {result}"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // AliasGen produces sequential aliases
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_alias_gen_sequential() {
        let mut alias_gen = AliasGen::new();
        assert_eq!(alias_gen.next(), "_u0");
        assert_eq!(alias_gen.next(), "_u1");
        assert_eq!(alias_gen.next(), "_u2");
    }
}
