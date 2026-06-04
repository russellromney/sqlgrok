#![allow(
    dead_code,
    reason = "private internal generator is enabled incrementally before production hot-path wiring"
)]

use crate::ast::{BinaryOperator, QuoteStyle};
use crate::dialects::Dialect;
use crate::parser::internal_ast::{
    InternalExpr, InternalOrderBy, InternalSelect, InternalSelectItem, InternalStatement,
    InternalTableRef,
};
use crate::parser::text::SqlText;

pub(crate) fn generate_internal(
    statement: &InternalStatement<'_>,
    dialect: Dialect,
) -> Option<String> {
    let mut generator = InternalGenerator {
        output: String::new(),
        dialect,
    };
    generator.gen_statement(statement)?;
    Some(trim_generator_output(generator.output))
}

struct InternalGenerator {
    output: String,
    dialect: Dialect,
}

impl InternalGenerator {
    fn gen_statement(&mut self, statement: &InternalStatement<'_>) -> Option<()> {
        match statement {
            InternalStatement::Select(select) => self.gen_select(select),
        }
    }

    fn gen_select(&mut self, select: &InternalSelect<'_>) -> Option<()> {
        self.write_keyword("SELECT");
        if !select.columns.is_empty() {
            self.write(" ");
            self.gen_select_items(&select.columns)?;
        }

        if let Some(table) = &select.from {
            self.write(" ");
            self.write_keyword("FROM");
            self.write(" ");
            self.gen_table_ref(table);
        }

        if let Some(expr) = &select.where_clause {
            self.write(" ");
            self.write_keyword("WHERE");
            self.write(" ");
            self.gen_expr(expr)?;
        }

        if !select.group_by.is_empty() {
            self.write(" ");
            self.write_keyword("GROUP BY");
            self.write(" ");
            self.gen_exprs(&select.group_by)?;
        }

        if !select.order_by.is_empty() {
            self.write(" ");
            self.write_keyword("ORDER BY");
            self.write(" ");
            self.gen_order_by_items(&select.order_by)?;
        }

        if let Some(limit) = &select.limit {
            self.write(" ");
            self.write_keyword("LIMIT ");
            self.gen_expr(limit)?;
        }

        if let Some(offset) = &select.offset {
            self.write(" ");
            self.write_keyword("OFFSET ");
            self.gen_expr(offset)?;
        }

        Some(())
    }

    fn gen_select_items(&mut self, items: &[InternalSelectItem<'_>]) -> Option<()> {
        for (index, item) in items.iter().enumerate() {
            if index > 0 {
                self.write(", ");
            }
            self.gen_select_item(item)?;
        }
        Some(())
    }

    fn gen_select_item(&mut self, item: &InternalSelectItem<'_>) -> Option<()> {
        match item {
            InternalSelectItem::Wildcard => self.write("*"),
            InternalSelectItem::Expr {
                expr,
                alias,
                alias_quote_style,
            } => {
                self.gen_expr(expr)?;
                if let Some(alias) = alias {
                    self.write(" ");
                    self.write_keyword("AS ");
                    self.write_quoted(alias, *alias_quote_style);
                }
            }
        }
        Some(())
    }

    fn gen_table_ref(&mut self, table: &InternalTableRef<'_>) {
        self.write_quoted(&table.name, table.name_quote_style);
        if let Some(alias) = &table.alias {
            self.write(" ");
            self.write_keyword("AS ");
            self.write_quoted(alias, table.alias_quote_style);
        }
    }

    fn gen_order_by_items(&mut self, items: &[InternalOrderBy<'_>]) -> Option<()> {
        for (index, item) in items.iter().enumerate() {
            if index > 0 {
                self.write(", ");
            }
            self.gen_expr(&item.expr)?;
            if item.explicit_direction {
                self.write(" ");
                self.write_keyword(if item.ascending { "ASC" } else { "DESC" });
            }
            if let Some(nulls_first) = item.nulls_first {
                self.write(" ");
                self.write_keyword(if nulls_first {
                    "NULLS FIRST"
                } else {
                    "NULLS LAST"
                });
            }
        }
        Some(())
    }

    fn gen_exprs(&mut self, exprs: &[InternalExpr<'_>]) -> Option<()> {
        for (index, expr) in exprs.iter().enumerate() {
            if index > 0 {
                self.write(", ");
            }
            self.gen_expr(expr)?;
        }
        Some(())
    }

    fn gen_expr(&mut self, expr: &InternalExpr<'_>) -> Option<()> {
        match expr {
            InternalExpr::Column {
                table,
                name,
                quote_style,
                table_quote_style,
            } => {
                if let Some(table) = table {
                    self.write_quoted(table, *table_quote_style);
                    self.write(".");
                }
                self.write_quoted(name, *quote_style);
            }
            InternalExpr::Number(value) => self.write_text(value),
            InternalExpr::StringLiteral(value) => {
                self.write("'");
                self.write(&value.as_str().replace('\'', "''"));
                self.write("'");
            }
            InternalExpr::Boolean(value) => self.write(if *value { "TRUE" } else { "FALSE" }),
            InternalExpr::Null => self.write("NULL"),
            InternalExpr::Function {
                name,
                args,
                distinct,
            } => {
                self.write_text(name);
                self.write("(");
                if *distinct {
                    self.write_keyword("DISTINCT ");
                }
                for (index, arg) in args.iter().enumerate() {
                    if index > 0 {
                        self.write(", ");
                    }
                    self.gen_expr(arg)?;
                }
                self.write(")");
            }
            InternalExpr::BinaryOp { left, op, right } => {
                self.gen_expr(left)?;
                self.write(binary_op_str(op)?);
                self.gen_expr(right)?;
            }
            InternalExpr::Cast { .. } => return None,
        }
        Some(())
    }

    fn write_quoted(&mut self, text: &SqlText<'_>, style: QuoteStyle) {
        let effective_style = if style.is_quoted() {
            QuoteStyle::for_dialect(self.dialect)
        } else {
            style
        };
        match effective_style {
            QuoteStyle::None => self.write_text(text),
            QuoteStyle::DoubleQuote => {
                self.write("\"");
                self.write(&text.as_str().replace('"', "\"\""));
                self.write("\"");
            }
            QuoteStyle::Backtick => {
                self.write("`");
                self.write(&text.as_str().replace('`', "``"));
                self.write("`");
            }
            QuoteStyle::Bracket => {
                self.write("[");
                self.write(&text.as_str().replace(']', "]]"));
                self.write("]");
            }
        }
    }

    fn write_text(&mut self, text: &SqlText<'_>) {
        self.write(text.as_str());
    }

    fn write_keyword(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }
}

fn binary_op_str(op: &BinaryOperator) -> Option<&'static str> {
    match op {
        BinaryOperator::Plus => Some(" + "),
        BinaryOperator::Minus => Some(" - "),
        BinaryOperator::Multiply => Some(" * "),
        BinaryOperator::Divide => Some(" / "),
        BinaryOperator::Modulo => Some(" % "),
        BinaryOperator::Eq => Some(" = "),
        BinaryOperator::Neq => Some(" <> "),
        BinaryOperator::Lt => Some(" < "),
        BinaryOperator::Gt => Some(" > "),
        BinaryOperator::LtEq => Some(" <= "),
        BinaryOperator::GtEq => Some(" >= "),
        BinaryOperator::And => Some(" AND "),
        BinaryOperator::Or => Some(" OR "),
        BinaryOperator::Concat => Some(" || "),
        BinaryOperator::BitwiseAnd => Some(" & "),
        BinaryOperator::BitwiseOr => Some(" | "),
        BinaryOperator::BitwiseXor => Some(" ^ "),
        _ => None,
    }
}

fn trim_generator_output(output: String) -> String {
    let trimmed = output.trim_matches(|c: char| matches!(c, ' ' | '\t' | '\n' | '\r'));
    if trimmed.len() == output.len() {
        output
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::DataType;
    use crate::generator::generate;
    use crate::parser::internal_parser::parse_internal;
    use crate::parser::parse;

    fn assert_internal_generator_matches_public(sql: &str, read: Dialect, write: Dialect) {
        let internal = parse_internal(sql, read).unwrap().unwrap();
        let internal_sql = generate_internal(&internal, write).unwrap();
        let public = parse(sql, read).unwrap();
        let public_sql = generate(&public, write);

        assert_eq!(internal_sql, public_sql);
    }

    #[test]
    fn generates_simple_select_like_public_generator() {
        assert_internal_generator_matches_public("SELECT a FROM t", Dialect::Ansi, Dialect::Ansi);
    }

    #[test]
    fn generates_supported_select_subset_like_public_generator() {
        assert_internal_generator_matches_public(
            "SELECT t.a AS a1, COALESCE(b, 0), TRUE, NULL FROM tbl AS t WHERE t.a > 10 ORDER BY a1 DESC LIMIT 5",
            Dialect::Ansi,
            Dialect::Ansi,
        );
    }

    #[test]
    fn generates_quoted_identifiers_with_target_dialect_quotes() {
        assert_internal_generator_matches_public(
            "SELECT \"a\" AS \"x\" FROM \"t\" AS \"u\"",
            Dialect::Postgres,
            Dialect::Mysql,
        );
    }

    #[test]
    fn returns_none_for_unsupported_internal_shape() {
        let statement = InternalStatement::Select(InternalSelect {
            columns: vec![InternalSelectItem::Expr {
                expr: InternalExpr::Cast {
                    expr: Box::new(InternalExpr::Number(SqlText::borrowed("1"))),
                    data_type: DataType::Int,
                },
                alias: None,
                alias_quote_style: QuoteStyle::None,
            }],
            from: None,
            where_clause: None,
            group_by: vec![],
            order_by: vec![],
            limit: None,
            offset: None,
        });

        assert_eq!(generate_internal(&statement, Dialect::Ansi), None);
    }

    #[test]
    fn generates_group_by_and_offset_like_public_generator() {
        assert_internal_generator_matches_public(
            "SELECT a, COUNT(name) FROM t GROUP BY a ORDER BY a ASC LIMIT 10 OFFSET 5",
            Dialect::Sqlite,
            Dialect::Sqlite,
        );
    }
}
