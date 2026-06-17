//! # sqlgrok
//!
//! A SQL parser, optimizer, and transpiler library written in Rust,
//! targeting behavioral parity with Python's [SQLGlot](https://github.com/tobymao/sqlglot).
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
//! - AST diff for semantic SQL comparison
//!
//! ## Quick Start
//!
//! ```rust
//! use sqlgrok::{parse, generate, transpile, Dialect};
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
pub mod compat;
pub mod dialects;
pub mod diff;
pub mod errors;
pub mod executor;
pub mod ffi;
pub mod generator;
pub mod optimizer;
pub mod parser;
pub mod planner;
pub mod schema;
pub mod tokens;

pub use ast::{CommentType, Expr, MergeClauseKind, QuoteStyle, Statement};
pub use builder::{
    ConditionBuilder,
    SelectBuilder,
    // Arithmetic helpers
    add,
    alias,
    and_all,
    between,
    boolean,
    // Operators and expressions
    cast,
    // Expression factory functions
    column,
    // Builders
    condition,
    condition_dialect,
    div,
    // Comparison helpers
    eq,
    exists,
    func,
    func_distinct,
    gt,
    gte,
    in_list,
    in_subquery,
    is_not_null,
    is_null,
    like,
    literal,
    lt,
    lte,
    mul,
    neq,
    not,
    not_in_list,
    null,
    or_all,
    parse_condition,
    parse_condition_dialect,
    // Parse helpers
    parse_expr,
    parse_expr_dialect,
    qualified_star,
    select,
    select_all,
    select_distinct,
    // Other helpers
    star,
    string_literal,
    sub,
    subquery,
    table,
    table_full,
};
pub use compat::{ScalarSelectProjection, scalar_select_projection};
pub use dialects::Dialect;
pub use dialects::plugin::{
    DialectPlugin, DialectRef, DialectRegistry, register_dialect, resolve_dialect, transpile_ext,
    transpile_statements_ext,
};
pub use dialects::time::{
    FormatConversionResult, TimeFormatStyle, TsqlStyleCode, format_time, format_time_dialect,
    format_time_with_warnings,
};
pub use diff::{AstNode, ChangeAction, diff as diff_ast, diff_sql};
pub use errors::SqlglotError;
pub use generator::{generate, generate_pretty};
pub use optimizer::annotate_types::{TypeAnnotations, annotate_types};
pub use optimizer::lineage::{
    LineageConfig, LineageError, LineageGraph, LineageNode, lineage, lineage_sql,
};
pub use optimizer::pushdown_predicates::pushdown_predicates;
pub use optimizer::scope_analysis::{Scope, ScopeType, build_scope, find_all_in_scope};
pub use parser::parse;
pub use parser::{parse_statements_with_comments, parse_with_comments};
pub use planner::{Plan, Projection, Step, StepId, plan};

/// Transpile a SQL string from one dialect to another.
///
/// This is the primary high-level API, corresponding to Python sqlglot's
/// `sqlglot.transpile()`.
///
/// # Example
///
/// ```rust
/// use sqlgrok::{transpile, Dialect};
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
    Ok(generate(&ast, write_dialect))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InternalTranspileDecision {
    UsedInternal,
    FellBackToPublic,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalFastPathStatus {
    Used,
    ParseDeclined,
    TransformDeclined,
    GenerateDeclined,
    OutputMismatch,
}

#[allow(
    dead_code,
    reason = "private guarded internal transpile experiment is enabled incrementally"
)]
pub(crate) fn transpile_guarded_internal_experiment(
    sql: &str,
    read_dialect: Dialect,
    write_dialect: Dialect,
) -> errors::Result<(String, InternalTranspileDecision)> {
    let ast = parse(sql, read_dialect)?;
    let public_output = generate(&ast, write_dialect);

    let Ok(Some(internal)) = parser::parse_internal(sql, read_dialect) else {
        return Ok((public_output, InternalTranspileDecision::FellBackToPublic));
    };

    let Some(internal_output) = parser::generate_internal(&internal, write_dialect) else {
        return Ok((public_output, InternalTranspileDecision::FellBackToPublic));
    };
    if internal_output == public_output {
        Ok((internal_output, InternalTranspileDecision::UsedInternal))
    } else {
        Ok((public_output, InternalTranspileDecision::FellBackToPublic))
    }
}

#[doc(hidden)]
pub fn transpile_internal_fast_experiment(
    sql: &str,
    read_dialect: Dialect,
    write_dialect: Dialect,
) -> errors::Result<Option<String>> {
    let (status, output) =
        transpile_internal_fast_experiment_status(sql, read_dialect, write_dialect)?;
    Ok(if status == InternalFastPathStatus::Used {
        output
    } else {
        None
    })
}

#[doc(hidden)]
pub fn transpile_internal_fast_experiment_status(
    sql: &str,
    read_dialect: Dialect,
    write_dialect: Dialect,
) -> errors::Result<(InternalFastPathStatus, Option<String>)> {
    let Some(internal) = parser::parse_internal(sql, read_dialect)? else {
        return Ok((InternalFastPathStatus::ParseDeclined, None));
    };
    let Some(transformed) = parser::transform_internal(internal, read_dialect, write_dialect)
    else {
        return Ok((InternalFastPathStatus::TransformDeclined, None));
    };
    let Some(output) = parser::generate_internal(&transformed, write_dialect) else {
        return Ok((InternalFastPathStatus::GenerateDeclined, None));
    };
    Ok((InternalFastPathStatus::Used, Some(output)))
}

#[doc(hidden)]
pub fn transpile_internal_fast_guarded_status(
    sql: &str,
    read_dialect: Dialect,
    write_dialect: Dialect,
) -> errors::Result<(InternalFastPathStatus, Option<String>)> {
    let public = transpile(sql, read_dialect, write_dialect)?;
    let (status, output) =
        transpile_internal_fast_experiment_status(sql, read_dialect, write_dialect)?;
    if status != InternalFastPathStatus::Used {
        return Ok((status, output));
    }
    if output.as_deref() == Some(public.as_str()) {
        Ok((InternalFastPathStatus::Used, output))
    } else {
        Ok((InternalFastPathStatus::OutputMismatch, output))
    }
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
    for stmt in stmts {
        results.push(generate(&stmt, write_dialect));
    }
    Ok(results)
}

/// Transpile a SQL string with pretty-printed output.
///
/// # Errors
///
/// Returns a [`SqlglotError`] if parsing fails.
pub fn transpile_statements_pretty(
    sql: &str,
    read_dialect: Dialect,
    write_dialect: Dialect,
) -> errors::Result<Vec<String>> {
    let stmts = parser::parse_statements(sql, read_dialect)?;
    let mut results = Vec::with_capacity(stmts.len());
    for stmt in stmts {
        results.push(generate_pretty(&stmt, write_dialect));
    }
    Ok(results)
}

/// A single request for [`transpile_many`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranspileRequest {
    pub sql: String,
    pub read: Dialect,
    pub write: Dialect,
    pub pretty: bool,
}

/// The result for one [`transpile_many`] request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranspileManyResult {
    pub ok: bool,
    pub sql: Option<Vec<String>>,
    pub error: Option<String>,
}

/// Transpile many SQL strings in one API call.
///
/// This is primarily useful for language bindings, where crossing the host
/// language boundary once per query can dominate benchmark and application
/// runtime.
#[must_use]
pub fn transpile_many(requests: &[TranspileRequest]) -> Vec<TranspileManyResult> {
    requests
        .iter()
        .map(|request| {
            let result = if request.pretty {
                transpile_statements_pretty(&request.sql, request.read, request.write)
            } else {
                transpile_statements(&request.sql, request.read, request.write)
            };
            match result {
                Ok(sql) => TranspileManyResult {
                    ok: true,
                    sql: Some(sql),
                    error: None,
                },
                Err(err) => TranspileManyResult {
                    ok: false,
                    sql: None,
                    error: Some(err.to_string()),
                },
            }
        })
        .collect()
}

/// Transpile a SQL string preserving comments through the pipeline.
///
/// # Errors
///
/// Returns a [`SqlglotError`] if parsing fails.
pub fn transpile_with_comments(
    sql: &str,
    read_dialect: Dialect,
    write_dialect: Dialect,
) -> errors::Result<String> {
    let ast = parse_with_comments(sql, read_dialect)?;
    Ok(generate(&ast, write_dialect))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guarded_internal_transpile_uses_internal_when_output_matches() {
        let (sql, decision) = transpile_guarded_internal_experiment(
            "SELECT a FROM t WHERE a > 1 ORDER BY a LIMIT 5",
            Dialect::Ansi,
            Dialect::Ansi,
        )
        .unwrap();

        assert_eq!(sql, "SELECT a FROM t WHERE a > 1 ORDER BY a LIMIT 5");
        assert_eq!(decision, InternalTranspileDecision::UsedInternal);
    }

    #[test]
    fn guarded_internal_transpile_falls_back_when_transform_changes_output() {
        let (sql, decision) = transpile_guarded_internal_experiment(
            "SELECT IFNULL(a, 0) FROM t",
            Dialect::Mysql,
            Dialect::Sqlite,
        )
        .unwrap();

        assert_eq!(sql, "SELECT COALESCE(a, 0) FROM t");
        assert_eq!(decision, InternalTranspileDecision::FellBackToPublic);
    }

    #[test]
    fn guarded_internal_transpile_falls_back_for_unsupported_internal_shape() {
        let (sql, decision) = transpile_guarded_internal_experiment(
            "SELECT a FROM t JOIN u ON t.id = u.id",
            Dialect::Ansi,
            Dialect::Ansi,
        )
        .unwrap();

        assert_eq!(sql, "SELECT a FROM t JOIN u ON t.id = u.id");
        assert_eq!(decision, InternalTranspileDecision::FellBackToPublic);
    }

    #[test]
    fn internal_fast_experiment_returns_supported_identity_output_without_oracle() {
        let sql = transpile_internal_fast_experiment(
            "SELECT a FROM t WHERE a > 1 ORDER BY a LIMIT 5",
            Dialect::Sqlite,
            Dialect::Sqlite,
        )
        .unwrap();

        assert_eq!(
            sql.as_deref(),
            Some("SELECT a FROM t WHERE a > 1 ORDER BY a LIMIT 5")
        );
    }

    #[test]
    fn internal_fast_experiment_matches_public_output_for_supported_identity_dialects() {
        for dialect in [
            Dialect::Ansi,
            Dialect::Postgres,
            Dialect::Mysql,
            Dialect::Sqlite,
        ] {
            let sql = "SELECT a, 'x' AS y FROM t AS u WHERE a > 1 ORDER BY y DESC LIMIT 5";
            let internal = transpile_internal_fast_experiment(sql, dialect, dialect)
                .unwrap()
                .unwrap();
            let public = transpile(sql, dialect, dialect).unwrap();

            assert_eq!(internal, public, "dialect: {dialect}");
        }
    }

    #[test]
    fn internal_fast_experiment_declines_pseudo_columns() {
        assert_eq!(
            transpile_internal_fast_experiment(
                "SELECT current_user FROM t",
                Dialect::Sqlite,
                Dialect::Sqlite
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn internal_fast_experiment_declines_cross_dialect_rewrites() {
        assert_eq!(
            transpile_internal_fast_experiment(
                "SELECT IFNULL(a, 0) FROM t",
                Dialect::Mysql,
                Dialect::Sqlite
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn internal_fast_experiment_declines_unsupported_shapes() {
        assert_eq!(
            transpile_internal_fast_experiment(
                "SELECT a FROM t JOIN u ON t.id = u.id",
                Dialect::Sqlite,
                Dialect::Sqlite
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn internal_fast_guarded_status_reports_used_when_output_matches() {
        let (status, output) = transpile_internal_fast_guarded_status(
            "SELECT a FROM t",
            Dialect::Sqlite,
            Dialect::Sqlite,
        )
        .unwrap();

        assert_eq!(status, InternalFastPathStatus::Used);
        assert_eq!(output.as_deref(), Some("SELECT a FROM t"));
    }

    #[test]
    fn internal_fast_guarded_status_covers_sqlite_identity_workload() {
        for sql in [
            "SELECT a, b FROM t WHERE a > 10 ORDER BY b DESC LIMIT 10 OFFSET 5",
            "SELECT JSON_TYPE(data, '$.k') FROM events",
            "CREATE TABLE users (id INTEGER PRIMARY KEY, email TEXT NOT NULL UNIQUE, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "INSERT OR IGNORE INTO users (id, email) VALUES (1, 'a@example.com')",
            "SELECT user_id, ROW_NUMBER() OVER (PARTITION BY account_id ORDER BY created_at) FROM events",
            "WITH a AS (SELECT 1 AS x), b AS (SELECT 2 AS y) SELECT a.x + b.y FROM a, b",
            "SELECT COUNT(DISTINCT name) FROM users GROUP BY account_id",
            "ALTER TABLE users ADD COLUMN created_at TEXT DEFAULT CURRENT_TIMESTAMP",
        ] {
            let (status, output) =
                transpile_internal_fast_guarded_status(sql, Dialect::Sqlite, Dialect::Sqlite)
                    .unwrap();

            assert_eq!(status, InternalFastPathStatus::Used, "{sql}");
            assert!(output.is_some(), "{sql}");
        }
    }
}
