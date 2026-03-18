//! # sqlglot-rust
//!
//! A SQL parser, optimizer, and transpiler library written in Rust,
//! inspired by Python's [sqlglot](https://github.com/tobymao/sqlglot).
//!
//! ## Features
//!
//! - Parse SQL strings into a structured AST
//! - Generate SQL from AST nodes
//! - Transpile between SQL dialects (30 dialects including MySQL, PostgreSQL, BigQuery, Snowflake, DuckDB, Hive, Spark, Presto, Trino, T-SQL, Oracle, ClickHouse, Redshift, and more)
//! - Optimize SQL queries
//! - CTEs, subqueries, UNION/INTERSECT/EXCEPT
//! - Window functions, CAST, EXTRACT, EXISTS
//! - Pretty-print SQL output
//! - AST traversal (walk, find, transform)
//!
//! ## Quick Start
//!
//! ```rust
//! use sqlglot_rust::{parse, generate, transpile, Dialect};
//!
//! // Parse a SQL query
//! let ast = parse("SELECT a, b FROM t WHERE a > 1", Dialect::Ansi).unwrap();
//!
//! // Generate SQL for a specific dialect
//! let sql = generate(&ast, Dialect::Postgres);
//! assert_eq!(sql, "SELECT a, b FROM t WHERE a > 1");
//!
//! // One-step transpile between dialects
//! let result = transpile("SELECT a, b FROM t", Dialect::Ansi, Dialect::Postgres).unwrap();
//! ```

pub mod ast;
pub mod builder;
pub mod dialects;
pub mod errors;
pub mod generator;
pub mod optimizer;
pub mod parser;
pub mod schema;
pub mod tokens;

pub use ast::{Expr, MergeClauseKind, QuoteStyle, Statement};
pub use builder::{
    // Expression factory functions
    column, table, table_full, literal, string_literal, boolean, null,
    // Operators and expressions
    cast, and_all, or_all, not, func, func_distinct,
    // Comparison helpers
    eq, neq, lt, lte, gt, gte, is_null, is_not_null, between, in_list, not_in_list, in_subquery, like,
    // Arithmetic helpers
    add, sub, mul, div,
    // Other helpers
    star, qualified_star, subquery, exists, alias,
    // Parse helpers
    parse_expr, parse_expr_dialect, parse_condition, parse_condition_dialect,
    // Builders
    condition, condition_dialect, select, select_all, select_distinct,
    ConditionBuilder, SelectBuilder,
};
pub use dialects::Dialect;
pub use dialects::time::{TimeFormatStyle, format_time, format_time_dialect, format_time_with_warnings, FormatConversionResult, TsqlStyleCode};
pub use errors::SqlglotError;
pub use generator::{generate, generate_pretty};
pub use optimizer::annotate_types::{TypeAnnotations, annotate_types};
pub use optimizer::lineage::{lineage, lineage_sql, LineageConfig, LineageError, LineageGraph, LineageNode};
pub use optimizer::pushdown_predicates::pushdown_predicates;
pub use optimizer::scope_analysis::{Scope, ScopeType, build_scope, find_all_in_scope};
pub use parser::parse;

/// Transpile a SQL string from one dialect to another.
///
/// This is the primary high-level API, corresponding to Python sqlglot's
/// `sqlglot.transpile()`.
///
/// # Example
///
/// ```rust
/// use sqlglot_rust::{transpile, Dialect};
///
/// let result = transpile(
///     "SELECT CAST(x AS INT) FROM t",
///     Dialect::Ansi,
///     Dialect::Postgres,
/// ).unwrap();
/// ```
///
/// # Errors
///
/// Returns a [`SqlglotError`] if parsing fails.
pub fn transpile(
    sql: &str,
    read_dialect: Dialect,
    write_dialect: Dialect,
) -> errors::Result<String> {
    let ast = parse(sql, read_dialect)?;
    let transformed = dialects::transform(&ast, read_dialect, write_dialect);
    Ok(generate(&transformed, write_dialect))
}

/// Transpile a SQL string, returning multiple statements if the input
/// contains semicolons.
///
/// # Errors
///
/// Returns a [`SqlglotError`] if parsing fails.
pub fn transpile_statements(
    sql: &str,
    read_dialect: Dialect,
    write_dialect: Dialect,
) -> errors::Result<Vec<String>> {
    let stmts = parser::parse_statements(sql, read_dialect)?;
    let mut results = Vec::with_capacity(stmts.len());
    for stmt in &stmts {
        let transformed = dialects::transform(stmt, read_dialect, write_dialect);
        results.push(generate(&transformed, write_dialect));
    }
    Ok(results)
}
