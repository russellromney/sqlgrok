#![allow(
    dead_code,
    reason = "private internal parser is enabled incrementally before production hot-path wiring"
)]

use crate::ast::{BinaryOperator, QuoteStyle};
use crate::dialects::Dialect;
use crate::errors::{Result, SqlglotError};
use crate::parser::internal_ast::{
    InternalExpr, InternalOrderBy, InternalSelect, InternalSelectItem, InternalStatement,
    InternalTableRef,
};
use crate::parser::text::SqlText;
use crate::tokens::{Token, TokenType, Tokenizer};

pub(crate) fn parse_internal(sql: &str, dialect: Dialect) -> Result<Option<InternalStatement<'_>>> {
    let mut tokenizer = tokenizer_for_dialect(sql, dialect);
    let tokens = tokenizer.tokenize()?;
    let mut parser = InternalParser {
        sql,
        tokens,
        pos: 0,
    };
    match parser.parse_statement() {
        Err(SqlglotError::UnsupportedDialectFeature(message))
            if message.starts_with(INTERNAL_UNSUPPORTED_PREFIX) =>
        {
            Ok(None)
        }
        other => other,
    }
}

const INTERNAL_UNSUPPORTED_PREFIX: &str = "internal parser fallback:";

struct InternalParser<'sql> {
    sql: &'sql str,
    tokens: Vec<Token>,
    pos: usize,
}

impl<'sql> InternalParser<'sql> {
    fn parse_statement(&mut self) -> Result<Option<InternalStatement<'sql>>> {
        if !self.match_token(TokenType::Select) {
            return Ok(None);
        }

        let columns = self.parse_select_items()?;
        let from = if self.match_token(TokenType::From) {
            Some(self.parse_table_ref()?)
        } else {
            None
        };

        if self.peek_is_join_start() {
            return Ok(None);
        }

        let where_clause = if self.match_token(TokenType::Where) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        let group_by = if self.match_token(TokenType::Group) {
            self.expect(TokenType::By)?;
            self.parse_expr_list()?
        } else {
            vec![]
        };

        let order_by = if self.match_token(TokenType::Order) {
            self.expect(TokenType::By)?;
            self.parse_order_by_items()?
        } else {
            vec![]
        };

        let limit = if self.match_token(TokenType::Limit) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        let offset = if self.match_token(TokenType::Offset) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.consume_optional_semicolon();
        if !self.is_eof() {
            return Ok(None);
        }

        Ok(Some(InternalStatement::Select(InternalSelect {
            columns,
            from,
            where_clause,
            group_by,
            order_by,
            limit,
            offset,
        })))
    }

    fn parse_select_items(&mut self) -> Result<Vec<InternalSelectItem<'sql>>> {
        let mut items = vec![self.parse_select_item()?];
        while self.match_token(TokenType::Comma) {
            if matches!(self.peek_type(), TokenType::From | TokenType::Eof) {
                return Ok(items);
            }
            items.push(self.parse_select_item()?);
        }
        Ok(items)
    }

    fn parse_select_item(&mut self) -> Result<InternalSelectItem<'sql>> {
        if self.match_token(TokenType::Star) {
            return Ok(InternalSelectItem::Wildcard);
        }

        let expr = self.parse_expr()?;
        let (alias, alias_quote_style) = if self.match_token(TokenType::As) {
            let token = self.expect_identifier_like()?;
            (
                Some(self.token_text_at(token)),
                quote_style_from_char(self.token_at(token).quote_char),
            )
        } else {
            (None, QuoteStyle::None)
        };

        Ok(InternalSelectItem::Expr {
            expr,
            alias,
            alias_quote_style,
        })
    }

    fn parse_table_ref(&mut self) -> Result<InternalTableRef<'sql>> {
        let name = self.expect_identifier_like()?;
        let name_quote_style = quote_style_from_char(self.token_at(name).quote_char);
        let name = self.token_text_at(name);

        let (alias, alias_quote_style) = if self.match_token(TokenType::As) {
            let token = self.expect_identifier_like()?;
            (
                Some(self.token_text_at(token)),
                quote_style_from_char(self.token_at(token).quote_char),
            )
        } else if self.peek_is_identifier_like() {
            let token = self.advance();
            (
                Some(self.token_text_at(token)),
                quote_style_from_char(self.token_at(token).quote_char),
            )
        } else {
            (None, QuoteStyle::None)
        };

        Ok(InternalTableRef {
            name,
            alias,
            name_quote_style,
            alias_quote_style,
        })
    }

    fn parse_order_by_items(&mut self) -> Result<Vec<InternalOrderBy<'sql>>> {
        self.parse_comma_list(Self::parse_order_by_item)
    }

    fn parse_order_by_item(&mut self) -> Result<InternalOrderBy<'sql>> {
        let expr = self.parse_expr()?;
        let (ascending, explicit_direction) = if self.match_token(TokenType::Asc) {
            (true, true)
        } else if self.match_token(TokenType::Desc) {
            (false, true)
        } else {
            (true, false)
        };

        Ok(InternalOrderBy {
            expr,
            ascending,
            explicit_direction,
            nulls_first: None,
        })
    }

    fn parse_expr(&mut self) -> Result<InternalExpr<'sql>> {
        let mut expr = self.parse_primary()?;

        while let Some(op) = self.peek_binary_operator() {
            self.advance();
            let right = self.parse_primary()?;
            if self.peek_binary_operator().is_some() {
                return self.unsupported("operator precedence");
            }
            expr = InternalExpr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<InternalExpr<'sql>> {
        let token = self.advance();
        match self.token_at(token).token_type {
            TokenType::Identifier
            | TokenType::Any
            | TokenType::Some
            | TokenType::Year
            | TokenType::Month
            | TokenType::Day
            | TokenType::Hour
            | TokenType::Minute
            | TokenType::Second => self.parse_identifier_primary(token),
            TokenType::Number => Ok(InternalExpr::Number(self.token_text_at(token))),
            TokenType::String | TokenType::EscapedString => Ok(InternalExpr::StringLiteral(
                SqlText::owned(self.token_at(token).value.clone()),
            )),
            TokenType::True => Ok(InternalExpr::Boolean(true)),
            TokenType::False => Ok(InternalExpr::Boolean(false)),
            TokenType::Null => Ok(InternalExpr::Null),
            _ => Err(SqlglotError::UnexpectedToken {
                token: self.token_at(token).clone(),
            }),
        }
    }

    fn parse_identifier_primary(&mut self, token: usize) -> Result<InternalExpr<'sql>> {
        let first_name = self.token_text_at(token);
        let first_quote_style = quote_style_from_char(self.token_at(token).quote_char);

        if self.match_token(TokenType::LParen) {
            let (distinct, args) = self.parse_function_args()?;
            return Ok(InternalExpr::Function {
                name: first_name,
                args,
                distinct,
            });
        }

        if self.match_token(TokenType::Dot) {
            if self.peek_type() == &TokenType::Star {
                return self.unsupported("qualified wildcard");
            }
            let column = self.expect_identifier_like()?;
            return Ok(InternalExpr::Column {
                table: Some(first_name),
                name: self.token_text_at(column),
                quote_style: quote_style_from_char(self.token_at(column).quote_char),
                table_quote_style: first_quote_style,
            });
        }

        Ok(InternalExpr::Column {
            table: None,
            name: first_name,
            quote_style: first_quote_style,
            table_quote_style: QuoteStyle::None,
        })
    }

    fn parse_function_args(&mut self) -> Result<(bool, Vec<InternalExpr<'sql>>)> {
        if self.match_token(TokenType::RParen) {
            return Ok((false, vec![]));
        }

        let distinct = self.match_token(TokenType::Distinct);
        let mut args = vec![self.parse_expr()?];
        while self.match_token(TokenType::Comma) {
            args.push(self.parse_expr()?);
        }
        self.expect(TokenType::RParen)?;
        Ok((distinct, args))
    }

    fn parse_expr_list(&mut self) -> Result<Vec<InternalExpr<'sql>>> {
        self.parse_comma_list(Self::parse_expr)
    }

    fn parse_comma_list<T>(&mut self, parse_item: fn(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let mut items = vec![parse_item(self)?];
        while self.match_token(TokenType::Comma) {
            items.push(parse_item(self)?);
        }
        Ok(items)
    }

    fn peek_binary_operator(&self) -> Option<BinaryOperator> {
        match self.peek_type() {
            TokenType::Plus => Some(BinaryOperator::Plus),
            TokenType::Minus => Some(BinaryOperator::Minus),
            TokenType::Star => Some(BinaryOperator::Multiply),
            TokenType::Slash => Some(BinaryOperator::Divide),
            TokenType::Percent2 => Some(BinaryOperator::Modulo),
            TokenType::Eq => Some(BinaryOperator::Eq),
            TokenType::Neq => Some(BinaryOperator::Neq),
            TokenType::Lt => Some(BinaryOperator::Lt),
            TokenType::Gt => Some(BinaryOperator::Gt),
            TokenType::LtEq => Some(BinaryOperator::LtEq),
            TokenType::GtEq => Some(BinaryOperator::GtEq),
            TokenType::And => Some(BinaryOperator::And),
            TokenType::Or => Some(BinaryOperator::Or),
            TokenType::Concat => Some(BinaryOperator::Concat),
            TokenType::BitwiseAnd => Some(BinaryOperator::BitwiseAnd),
            TokenType::BitwiseOr => Some(BinaryOperator::BitwiseOr),
            TokenType::BitwiseXor => Some(BinaryOperator::BitwiseXor),
            _ => None,
        }
    }

    fn expect(&mut self, expected: TokenType) -> Result<()> {
        if self.match_token(expected.clone()) {
            Ok(())
        } else {
            Err(SqlglotError::ParserError {
                message: format!("Expected {:?}, got {:?}", expected, self.peek_type()),
            })
        }
    }

    fn unsupported<T>(&self, feature: &str) -> Result<T> {
        Err(SqlglotError::UnsupportedDialectFeature(format!(
            "{INTERNAL_UNSUPPORTED_PREFIX} {feature}"
        )))
    }

    fn expect_identifier_like(&mut self) -> Result<usize> {
        if self.peek_is_identifier_like() {
            Ok(self.advance())
        } else {
            Err(SqlglotError::UnexpectedToken {
                token: self.peek().clone(),
            })
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.peek_type() == &token_type {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> usize {
        let token = self.pos;
        self.pos += 1;
        token
    }

    fn token_at(&self, pos: usize) -> &Token {
        &self.tokens[pos]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn peek_type(&self) -> &TokenType {
        &self.peek().token_type
    }

    fn is_eof(&self) -> bool {
        self.peek_type() == &TokenType::Eof
    }

    fn consume_optional_semicolon(&mut self) {
        self.match_token(TokenType::Semicolon);
    }

    fn peek_is_join_start(&self) -> bool {
        matches!(
            self.peek_type(),
            TokenType::Join
                | TokenType::Inner
                | TokenType::Left
                | TokenType::Right
                | TokenType::Full
                | TokenType::Cross
                | TokenType::Lateral
        )
    }

    fn peek_is_identifier_like(&self) -> bool {
        matches!(
            self.peek_type(),
            TokenType::Identifier
                | TokenType::Any
                | TokenType::Some
                | TokenType::Year
                | TokenType::Month
                | TokenType::Day
                | TokenType::Hour
                | TokenType::Minute
                | TokenType::Second
        )
    }

    fn token_text_at(&self, token: usize) -> SqlText<'sql> {
        let token = self.token_at(token);
        if token.quote_char == '\0'
            && let Some(text) = self.sql.get(token.position..token.end)
            && text == token.value
        {
            SqlText::borrowed(text)
        } else {
            SqlText::owned(token.value.clone())
        }
    }
}

fn tokenizer_for_dialect(sql: &str, dialect: Dialect) -> Tokenizer<'_> {
    Tokenizer::with_options(sql, false, dialect_supports_hash_comments(dialect))
        .with_bit_literals_as_numbers(matches!(dialect, Dialect::Mysql | Dialect::Postgres))
        .with_digit_letter_is_identifier(matches!(
            dialect,
            Dialect::Mysql | Dialect::SingleStore | Dialect::Doris | Dialect::StarRocks
        ))
        .with_interpret_string_escapes(!matches!(dialect, Dialect::Sqlite | Dialect::Postgres))
}

fn dialect_supports_hash_comments(dialect: Dialect) -> bool {
    matches!(
        dialect,
        Dialect::Ansi | Dialect::Mysql | Dialect::Doris | Dialect::SingleStore | Dialect::StarRocks
    )
}

fn quote_style_from_char(c: char) -> QuoteStyle {
    match c {
        '"' => QuoteStyle::DoubleQuote,
        '`' => QuoteStyle::Backtick,
        '[' => QuoteStyle::Bracket,
        _ => QuoteStyle::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::generate;
    use crate::parser::{generate_internal, parse};

    fn assert_internal_matches_public(sql: &str, dialect: Dialect) {
        let internal = parse_internal(sql, dialect).unwrap().unwrap().into_public();
        let public = parse(sql, dialect).unwrap();

        assert_eq!(internal, public);
        assert_eq!(generate(&internal, dialect), generate(&public, dialect));
    }

    fn assert_internal_generates_like_public(sql: &str, dialect: Dialect) {
        let internal = parse_internal(sql, dialect).unwrap().unwrap();
        let public = parse(sql, dialect).unwrap();
        let public_sql = generate(&public, dialect);

        assert_eq!(
            generate_internal(&internal, dialect).as_deref(),
            Some(public_sql.as_str())
        );
    }

    #[test]
    fn parses_simple_select_from() {
        assert_internal_matches_public("SELECT a FROM t", Dialect::Ansi);
    }

    #[test]
    fn parses_where_order_by_and_limit() {
        assert_internal_matches_public(
            "SELECT t.a AS a1, COALESCE(b, 0) FROM tbl AS t WHERE t.a > 10 ORDER BY a1 DESC LIMIT 5",
            Dialect::Ansi,
        );
    }

    #[test]
    fn parses_group_by_limit_offset_and_explicit_asc() {
        assert_internal_matches_public(
            "SELECT a, name FROM t WHERE a > 1 GROUP BY a, name ORDER BY a ASC LIMIT 10 OFFSET 5",
            Dialect::Sqlite,
        );
    }

    #[test]
    fn parses_count_distinct_group_by() {
        assert_internal_generates_like_public(
            "SELECT COUNT(DISTINCT name) FROM users GROUP BY account_id",
            Dialect::Sqlite,
        );
    }

    #[test]
    fn parses_literals_and_wildcards() {
        assert_internal_matches_public("SELECT *, TRUE, FALSE, NULL, 'x' FROM t", Dialect::Ansi);
    }

    #[test]
    fn borrows_unquoted_identifiers_and_numbers() {
        let Some(InternalStatement::Select(select)) =
            parse_internal("SELECT a, 42 FROM t", Dialect::Ansi).unwrap()
        else {
            panic!("expected internal select");
        };

        let InternalSelectItem::Expr {
            expr: InternalExpr::Column { name, .. },
            ..
        } = &select.columns[0]
        else {
            panic!("expected column");
        };
        assert!(name.is_borrowed());

        let InternalSelectItem::Expr {
            expr: InternalExpr::Number(value),
            ..
        } = &select.columns[1]
        else {
            panic!("expected number");
        };
        assert!(value.is_borrowed());

        let table = select.from.unwrap();
        assert!(table.name.is_borrowed());
    }

    #[test]
    fn owns_decoded_string_literals() {
        let Some(InternalStatement::Select(select)) =
            parse_internal("SELECT 'x' FROM t", Dialect::Ansi).unwrap()
        else {
            panic!("expected internal select");
        };

        let InternalSelectItem::Expr {
            expr: InternalExpr::StringLiteral(value),
            ..
        } = &select.columns[0]
        else {
            panic!("expected string literal");
        };
        assert_eq!(value.as_str(), "x");
        assert!(!value.is_borrowed());
    }

    #[test]
    fn falls_back_for_joins() {
        assert!(
            parse_internal("SELECT a FROM t JOIN u ON t.id = u.id", Dialect::Ansi)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn parses_group_by() {
        assert_internal_matches_public("SELECT a FROM t GROUP BY a", Dialect::Ansi);
    }

    #[test]
    fn falls_back_for_operator_precedence() {
        assert!(
            parse_internal("SELECT a + b * c FROM t", Dialect::Ansi)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn falls_back_for_qualified_wildcard() {
        assert!(
            parse_internal("SELECT t.* FROM t", Dialect::Ansi)
                .unwrap()
                .is_none()
        );
    }
}
