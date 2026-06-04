#![allow(
    dead_code,
    reason = "private internal transform is enabled incrementally before production hot-path wiring"
)]

use crate::dialects::Dialect;
use crate::parser::internal_ast::{
    InternalCte, InternalExpr, InternalOrderBy, InternalSelect, InternalSelectItem,
    InternalStatement, InternalTableRef, InternalWindowSpec,
};

pub(crate) fn transform_internal<'sql>(
    statement: InternalStatement<'sql>,
    read: Dialect,
    write: Dialect,
) -> Option<InternalStatement<'sql>> {
    if read != write || !supports_internal_identity_dialect(write) {
        return None;
    }
    if !identity_safe_statement(&statement) {
        return None;
    }
    Some(statement)
}

fn supports_internal_identity_dialect(dialect: Dialect) -> bool {
    matches!(
        dialect,
        Dialect::Ansi | Dialect::Postgres | Dialect::Mysql | Dialect::Sqlite
    )
}

fn identity_safe_statement(statement: &InternalStatement<'_>) -> bool {
    match statement {
        InternalStatement::Select(select) => identity_safe_select(select),
        InternalStatement::RawIdentity(_) => true,
    }
}

fn identity_safe_select(select: &InternalSelect<'_>) -> bool {
    select.ctes.iter().all(identity_safe_cte)
        && select.columns.iter().all(identity_safe_select_item)
        && select.from.iter().all(identity_safe_table_ref)
        && select.where_clause.as_ref().is_none_or(identity_safe_expr)
        && select.group_by.iter().all(identity_safe_expr)
        && select.order_by.iter().all(identity_safe_order_by)
        && select.limit.as_ref().is_none_or(identity_safe_expr)
        && select.offset.as_ref().is_none_or(identity_safe_expr)
}

fn identity_safe_cte(cte: &InternalCte<'_>) -> bool {
    identity_safe_statement(&cte.query)
}

fn identity_safe_select_item(item: &InternalSelectItem<'_>) -> bool {
    match item {
        InternalSelectItem::Wildcard => true,
        InternalSelectItem::Expr { expr, .. } => identity_safe_expr(expr),
    }
}

fn identity_safe_table_ref(_table: &InternalTableRef<'_>) -> bool {
    true
}

fn identity_safe_order_by(item: &InternalOrderBy<'_>) -> bool {
    identity_safe_expr(&item.expr)
}

fn identity_safe_expr(expr: &InternalExpr<'_>) -> bool {
    match expr {
        InternalExpr::Column {
            table,
            name,
            quote_style,
            ..
        } => {
            table.is_some()
                || quote_style.is_quoted()
                || !public_generator_canonicalizes_bare_column(name.as_str())
        }
        InternalExpr::Number(_)
        | InternalExpr::StringLiteral(_)
        | InternalExpr::Boolean(_)
        | InternalExpr::Null => true,
        InternalExpr::Function { args, over, .. } => {
            args.iter().all(identity_safe_expr)
                && over.as_ref().is_none_or(identity_safe_window_spec)
        }
        InternalExpr::BinaryOp { left, right, .. } => {
            identity_safe_expr(left) && identity_safe_expr(right)
        }
        InternalExpr::Cast { .. } => false,
    }
}

fn identity_safe_window_spec(spec: &InternalWindowSpec<'_>) -> bool {
    spec.partition_by.iter().all(identity_safe_expr)
        && spec.order_by.iter().all(identity_safe_order_by)
}

fn public_generator_canonicalizes_bare_column(name: &str) -> bool {
    matches!(
        name.to_ascii_uppercase().as_str(),
        "CURRENT_TIME"
            | "CURRENT_DATE"
            | "CURRENT_TIMESTAMP"
            | "CURRENT_USER"
            | "CURRENT_ROLE"
            | "CURRENT_SCHEMA"
            | "CURRENT_CATALOG"
            | "CURRENT_PATH"
            | "LOCALTIME"
            | "LOCALTIMESTAMP"
            | "SESSION_USER"
            | "SYSTEM_USER"
            | "USER"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_internal;

    #[test]
    fn preserves_supported_identity_dialects() {
        let statement = parse_internal("SELECT a FROM t", Dialect::Sqlite)
            .unwrap()
            .unwrap();

        assert!(transform_internal(statement, Dialect::Sqlite, Dialect::Sqlite).is_some());
    }

    #[test]
    fn rejects_cross_dialect_transforms() {
        let statement = parse_internal("SELECT a FROM t", Dialect::Mysql)
            .unwrap()
            .unwrap();

        assert!(transform_internal(statement, Dialect::Mysql, Dialect::Sqlite).is_none());
    }

    #[test]
    fn rejects_identity_dialects_with_different_table_alias_rules() {
        let statement = parse_internal("SELECT a FROM t AS x", Dialect::Oracle)
            .unwrap()
            .unwrap();

        assert!(transform_internal(statement, Dialect::Oracle, Dialect::Oracle).is_none());
    }

    #[test]
    fn rejects_bare_pseudo_columns_canonicalized_by_public_generator() {
        let statement = parse_internal("SELECT current_user FROM t", Dialect::Sqlite)
            .unwrap()
            .unwrap();

        assert!(transform_internal(statement, Dialect::Sqlite, Dialect::Sqlite).is_none());
    }
}
