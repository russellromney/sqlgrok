#![allow(
    dead_code,
    reason = "private internal AST scaffold is enabled incrementally"
)]

use crate::ast::{
    BinaryOperator, Cte, DataType, Expr, FromClause, JoinClause, JoinType, OrderByItem, QuoteStyle,
    SelectItem, SelectStatement, Statement, TableRef, TableSource, WindowSpec,
};
use crate::parser::text::SqlText;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InternalStatement<'sql> {
    Select(Box<InternalSelect<'sql>>),
    RawIdentity(SqlText<'sql>),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct InternalSelect<'sql> {
    pub(crate) ctes: Vec<InternalCte<'sql>>,
    pub(crate) columns: Vec<InternalSelectItem<'sql>>,
    pub(crate) from: Vec<InternalTableRef<'sql>>,
    pub(crate) where_clause: Option<InternalExpr<'sql>>,
    pub(crate) group_by: Vec<InternalExpr<'sql>>,
    pub(crate) order_by: Vec<InternalOrderBy<'sql>>,
    pub(crate) limit: Option<InternalExpr<'sql>>,
    pub(crate) offset: Option<InternalExpr<'sql>>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct InternalCte<'sql> {
    pub(crate) name: SqlText<'sql>,
    pub(crate) name_quote_style: QuoteStyle,
    pub(crate) query: Box<InternalStatement<'sql>>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InternalSelectItem<'sql> {
    Wildcard,
    Expr {
        expr: InternalExpr<'sql>,
        alias: Option<SqlText<'sql>>,
        alias_quote_style: QuoteStyle,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct InternalTableRef<'sql> {
    pub(crate) name: SqlText<'sql>,
    pub(crate) alias: Option<SqlText<'sql>>,
    pub(crate) name_quote_style: QuoteStyle,
    pub(crate) alias_quote_style: QuoteStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct InternalOrderBy<'sql> {
    pub(crate) expr: InternalExpr<'sql>,
    pub(crate) ascending: bool,
    pub(crate) explicit_direction: bool,
    pub(crate) nulls_first: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InternalExpr<'sql> {
    Column {
        table: Option<SqlText<'sql>>,
        name: SqlText<'sql>,
        quote_style: QuoteStyle,
        table_quote_style: QuoteStyle,
    },
    Number(SqlText<'sql>),
    StringLiteral(SqlText<'sql>),
    Boolean(bool),
    Null,
    Function {
        name: SqlText<'sql>,
        args: Vec<InternalExpr<'sql>>,
        distinct: bool,
        over: Option<InternalWindowSpec<'sql>>,
    },
    BinaryOp {
        left: Box<InternalExpr<'sql>>,
        op: BinaryOperator,
        right: Box<InternalExpr<'sql>>,
    },
    Cast {
        expr: Box<InternalExpr<'sql>>,
        data_type: DataType,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct InternalWindowSpec<'sql> {
    pub(crate) partition_by: Vec<InternalExpr<'sql>>,
    pub(crate) order_by: Vec<InternalOrderBy<'sql>>,
}

impl<'sql> InternalStatement<'sql> {
    pub(crate) fn into_public(self) -> Statement {
        match self {
            Self::Select(select) => Statement::Select(select.into_public()),
            Self::RawIdentity(sql) => Statement::Raw(crate::ast::RawStatement {
                comments: vec![],
                sql: sql.into_owned(),
            }),
        }
    }
}

impl<'sql> InternalSelect<'sql> {
    pub(crate) fn into_public(self) -> SelectStatement {
        let mut from = self.from.into_iter();
        SelectStatement {
            comments: vec![],
            ctes: self
                .ctes
                .into_iter()
                .map(InternalCte::into_public)
                .collect(),
            distinct: false,
            distinct_on: vec![],
            top: None,
            columns: self
                .columns
                .into_iter()
                .map(InternalSelectItem::into_public)
                .collect(),
            from: from.next().map(|table| FromClause {
                source: TableSource::Table(table.into_public()),
            }),
            joins: from
                .map(|table| JoinClause {
                    join_type: JoinType::Cross,
                    table: TableSource::Table(table.into_public()),
                    on: None,
                    using: vec![],
                })
                .collect(),
            where_clause: self.where_clause.map(InternalExpr::into_public),
            group_by: self
                .group_by
                .into_iter()
                .map(InternalExpr::into_public)
                .collect(),
            having: None,
            order_by: self
                .order_by
                .into_iter()
                .map(InternalOrderBy::into_public)
                .collect(),
            limit: self.limit.map(InternalExpr::into_public),
            offset: self.offset.map(InternalExpr::into_public),
            limit_by: vec![],
            fetch_first: None,
            qualify: None,
            window_definitions: vec![],
            lock: None,
        }
    }
}

impl<'sql> InternalCte<'sql> {
    fn into_public(self) -> Cte {
        Cte {
            name: self.name.into_owned(),
            name_quote_style: self.name_quote_style,
            columns: vec![],
            query: Box::new(self.query.into_public()),
            materialized: None,
            recursive: false,
        }
    }
}

impl<'sql> InternalSelectItem<'sql> {
    fn into_public(self) -> SelectItem {
        match self {
            Self::Wildcard => SelectItem::Wildcard,
            Self::Expr {
                expr,
                alias,
                alias_quote_style,
            } => SelectItem::Expr {
                expr: expr.into_public(),
                alias: alias.map(SqlText::into_owned),
                alias_quote_style,
            },
        }
    }
}

impl<'sql> InternalTableRef<'sql> {
    fn into_public(self) -> TableRef {
        TableRef {
            catalog: None,
            schema: None,
            name: self.name.into_owned(),
            alias: self.alias.map(SqlText::into_owned),
            name_quote_style: self.name_quote_style,
            alias_quote_style: self.alias_quote_style,
        }
    }
}

impl<'sql> InternalOrderBy<'sql> {
    fn into_public(self) -> OrderByItem {
        OrderByItem {
            expr: self.expr.into_public(),
            ascending: self.ascending,
            explicit_direction: self.explicit_direction,
            nulls_first: self.nulls_first,
        }
    }
}

impl<'sql> InternalExpr<'sql> {
    pub(crate) fn into_public(self) -> Expr {
        match self {
            Self::Column {
                table,
                name,
                quote_style,
                table_quote_style,
            } => Expr::Column {
                table: table.map(SqlText::into_owned),
                name: name.into_owned(),
                quote_style,
                table_quote_style,
            },
            Self::Number(value) => Expr::Number(value.into_owned()),
            Self::StringLiteral(value) => Expr::StringLiteral(value.into_owned()),
            Self::Boolean(value) => Expr::Boolean(value),
            Self::Null => Expr::Null,
            Self::Function {
                name,
                args,
                distinct,
                over,
            } => {
                let name = name.into_owned();
                let args: Vec<Expr> = args.into_iter().map(Self::into_public).collect();
                if name.eq_ignore_ascii_case("COALESCE") && !distinct && over.is_none() {
                    Expr::Coalesce {
                        items: args,
                        is_nvl: false,
                        is_null: false,
                        source_name: None,
                    }
                } else {
                    Expr::Function {
                        name,
                        args,
                        distinct,
                        filter: None,
                        over: over.map(InternalWindowSpec::into_public),
                    }
                }
            }
            Self::BinaryOp { left, op, right } => Expr::BinaryOp {
                left: Box::new(left.into_public()),
                op,
                right: Box::new(right.into_public()),
            },
            Self::Cast { expr, data_type } => Expr::Cast {
                expr: Box::new(expr.into_public()),
                data_type,
            },
        }
    }
}

impl<'sql> InternalWindowSpec<'sql> {
    fn into_public(self) -> WindowSpec {
        WindowSpec {
            window_ref: None,
            partition_by: self
                .partition_by
                .into_iter()
                .map(InternalExpr::into_public)
                .collect(),
            order_by: self
                .order_by
                .into_iter()
                .map(InternalOrderBy::into_public)
                .collect(),
            frame: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialects::Dialect;
    use crate::generator::generate;
    use crate::parser::parse;

    #[test]
    fn internal_select_subset_converts_to_public_ast() {
        let internal = InternalStatement::Select(Box::new(InternalSelect {
            ctes: vec![],
            columns: vec![
                InternalSelectItem::Expr {
                    expr: InternalExpr::Column {
                        table: Some(SqlText::borrowed("t")),
                        name: SqlText::borrowed("a"),
                        quote_style: QuoteStyle::None,
                        table_quote_style: QuoteStyle::None,
                    },
                    alias: Some(SqlText::borrowed("a1")),
                    alias_quote_style: QuoteStyle::None,
                },
                InternalSelectItem::Expr {
                    expr: InternalExpr::Function {
                        name: SqlText::borrowed("COALESCE"),
                        args: vec![
                            InternalExpr::Column {
                                table: None,
                                name: SqlText::borrowed("b"),
                                quote_style: QuoteStyle::None,
                                table_quote_style: QuoteStyle::None,
                            },
                            InternalExpr::Number(SqlText::borrowed("0")),
                        ],
                        distinct: false,
                        over: None,
                    },
                    alias: None,
                    alias_quote_style: QuoteStyle::None,
                },
            ],
            from: vec![InternalTableRef {
                name: SqlText::borrowed("tbl"),
                alias: Some(SqlText::borrowed("t")),
                name_quote_style: QuoteStyle::None,
                alias_quote_style: QuoteStyle::None,
            }],
            where_clause: Some(InternalExpr::BinaryOp {
                left: Box::new(InternalExpr::Column {
                    table: Some(SqlText::borrowed("t")),
                    name: SqlText::borrowed("a"),
                    quote_style: QuoteStyle::None,
                    table_quote_style: QuoteStyle::None,
                }),
                op: BinaryOperator::Gt,
                right: Box::new(InternalExpr::Number(SqlText::borrowed("10"))),
            }),
            group_by: vec![],
            order_by: vec![InternalOrderBy {
                expr: InternalExpr::Column {
                    table: None,
                    name: SqlText::borrowed("a1"),
                    quote_style: QuoteStyle::None,
                    table_quote_style: QuoteStyle::None,
                },
                ascending: false,
                explicit_direction: true,
                nulls_first: None,
            }],
            limit: Some(InternalExpr::Number(SqlText::borrowed("5"))),
            offset: None,
        }));

        let public = internal.into_public();
        let expected = parse(
            "SELECT t.a AS a1, COALESCE(b, 0) FROM tbl AS t WHERE t.a > 10 ORDER BY a1 DESC LIMIT 5",
            Dialect::Ansi,
        )
        .unwrap();

        assert_eq!(public, expected);
        assert_eq!(
            generate(&public, Dialect::Ansi),
            "SELECT t.a AS a1, COALESCE(b, 0) FROM tbl AS t WHERE t.a > 10 ORDER BY a1 DESC LIMIT 5"
        );
    }

    #[test]
    fn internal_expr_can_mix_borrowed_and_owned_text() {
        let expr = InternalExpr::Cast {
            expr: Box::new(InternalExpr::StringLiteral(SqlText::borrowed("42"))),
            data_type: DataType::Unknown(SqlText::owned("INTEGER".to_string()).into_owned()),
        };

        assert_eq!(
            expr.into_public(),
            Expr::Cast {
                expr: Box::new(Expr::StringLiteral("42".to_string())),
                data_type: DataType::Unknown("INTEGER".to_string()),
            }
        );
    }
}
