//! Compatibility-analysis helpers for protocol shims.
//!
//! These APIs expose small, stable structural queries over sqlgrok's AST so
//! downstream protocol layers do not have to infer SQL shape with substring
//! scans. They intentionally do not execute SQL or know anything about a storage
//! backend.

use crate::ast::{SelectItem, Statement};
use crate::dialects::Dialect;
use crate::errors::Result;
use crate::generator::Generator;
use crate::parser::parse_statements;

/// A single scalar expression selected without any row source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarSelectProjection {
    /// The projection expression rendered back to SQL.
    pub expression_sql: String,
    /// Optional projection alias.
    pub alias: Option<String>,
}

/// Return the single projected expression for `SELECT <expr>` statements that do
/// not read from a table, CTE, subquery, or table function.
///
/// This is deliberately narrow: `SELECT <expr> FROM ...` returns `Ok(None)` so
/// callers cannot accidentally route table-backed user queries through scalar
/// demo compatibility paths.
///
/// # Errors
///
/// Returns a sqlgrok parser error when the input is not parseable SQL.
pub fn scalar_select_projection(
    sql: &str,
    dialect: Dialect,
) -> Result<Option<ScalarSelectProjection>> {
    let mut statements = parse_statements(sql, dialect)?;
    if statements.len() != 1 {
        return Ok(None);
    }
    let Statement::Select(select) = statements.remove(0) else {
        return Ok(None);
    };
    if !select.ctes.is_empty()
        || select.from.is_some()
        || !select.joins.is_empty()
        || select.where_clause.is_some()
        || !select.group_by.is_empty()
        || select.having.is_some()
        || !select.order_by.is_empty()
        || select.limit.is_some()
        || select.offset.is_some()
        || select.fetch_first.is_some()
        || select.qualify.is_some()
        || !select.window_definitions.is_empty()
        || select.columns.len() != 1
    {
        return Ok(None);
    }

    let SelectItem::Expr { expr, alias, .. } = &select.columns[0] else {
        return Ok(None);
    };
    Ok(Some(ScalarSelectProjection {
        expression_sql: Generator::expr_to_sql(expr),
        alias: alias.clone(),
    }))
}
