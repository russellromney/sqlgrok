use serde::{Deserialize, Serialize};

use crate::ast::*;

pub mod plugin;
pub(crate) mod rules;
pub mod time;

/// Supported SQL dialects.
///
/// Mirrors the full set of dialects supported by Python's sqlglot library.
/// Dialects are grouped into **Official** (core, higher-priority maintenance)
/// and **Community** (contributed, fully functional) tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dialect {
    // ── Core / base ──────────────────────────────────────────────────────
    /// ANSI SQL standard (default / base dialect)
    Ansi,

    // ── Official dialects ────────────────────────────────────────────────
    /// AWS Athena (Presto-based)
    Athena,
    /// Google BigQuery
    BigQuery,
    /// ClickHouse
    ClickHouse,
    /// Databricks (Spark-based)
    Databricks,
    /// DuckDB
    DuckDb,
    /// Apache Hive
    Hive,
    /// MySQL
    Mysql,
    /// Oracle Database
    Oracle,
    /// PostgreSQL
    Postgres,
    /// Presto
    Presto,
    /// Amazon Redshift (Postgres-based)
    Redshift,
    /// Snowflake
    Snowflake,
    /// Apache Spark SQL
    Spark,
    /// SQLite
    Sqlite,
    /// StarRocks (MySQL-compatible)
    StarRocks,
    /// Trino (Presto successor)
    Trino,
    /// Microsoft SQL Server (T-SQL)
    Tsql,

    // ── Community dialects ───────────────────────────────────────────────
    /// Apache Doris (MySQL-compatible)
    Doris,
    /// Dremio
    Dremio,
    /// Apache Drill
    Drill,
    /// Apache Druid
    Druid,
    /// Exasol
    Exasol,
    /// Microsoft Fabric (T-SQL variant)
    Fabric,
    /// Materialize (Postgres-compatible)
    Materialize,
    /// PRQL (Pipelined Relational Query Language)
    Prql,
    /// RisingWave (Postgres-compatible)
    RisingWave,
    /// SingleStore (MySQL-compatible)
    SingleStore,
    /// Tableau
    Tableau,
    /// Teradata
    Teradata,
}

impl std::fmt::Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dialect::Ansi => write!(f, "ANSI SQL"),
            Dialect::Athena => write!(f, "Athena"),
            Dialect::BigQuery => write!(f, "BigQuery"),
            Dialect::ClickHouse => write!(f, "ClickHouse"),
            Dialect::Databricks => write!(f, "Databricks"),
            Dialect::DuckDb => write!(f, "DuckDB"),
            Dialect::Hive => write!(f, "Hive"),
            Dialect::Mysql => write!(f, "MySQL"),
            Dialect::Oracle => write!(f, "Oracle"),
            Dialect::Postgres => write!(f, "PostgreSQL"),
            Dialect::Presto => write!(f, "Presto"),
            Dialect::Redshift => write!(f, "Redshift"),
            Dialect::Snowflake => write!(f, "Snowflake"),
            Dialect::Spark => write!(f, "Spark"),
            Dialect::Sqlite => write!(f, "SQLite"),
            Dialect::StarRocks => write!(f, "StarRocks"),
            Dialect::Trino => write!(f, "Trino"),
            Dialect::Tsql => write!(f, "T-SQL"),
            Dialect::Doris => write!(f, "Doris"),
            Dialect::Dremio => write!(f, "Dremio"),
            Dialect::Drill => write!(f, "Drill"),
            Dialect::Druid => write!(f, "Druid"),
            Dialect::Exasol => write!(f, "Exasol"),
            Dialect::Fabric => write!(f, "Fabric"),
            Dialect::Materialize => write!(f, "Materialize"),
            Dialect::Prql => write!(f, "PRQL"),
            Dialect::RisingWave => write!(f, "RisingWave"),
            Dialect::SingleStore => write!(f, "SingleStore"),
            Dialect::Tableau => write!(f, "Tableau"),
            Dialect::Teradata => write!(f, "Teradata"),
        }
    }
}

impl Dialect {
    /// Returns the support tier for this dialect.
    #[must_use]
    pub fn support_level(&self) -> &'static str {
        match self {
            Dialect::Ansi
            | Dialect::Athena
            | Dialect::BigQuery
            | Dialect::ClickHouse
            | Dialect::Databricks
            | Dialect::DuckDb
            | Dialect::Hive
            | Dialect::Mysql
            | Dialect::Oracle
            | Dialect::Postgres
            | Dialect::Presto
            | Dialect::Redshift
            | Dialect::Snowflake
            | Dialect::Spark
            | Dialect::Sqlite
            | Dialect::StarRocks
            | Dialect::Trino
            | Dialect::Tsql => "Official",

            Dialect::Doris
            | Dialect::Dremio
            | Dialect::Drill
            | Dialect::Druid
            | Dialect::Exasol
            | Dialect::Fabric
            | Dialect::Materialize
            | Dialect::Prql
            | Dialect::RisingWave
            | Dialect::SingleStore
            | Dialect::Tableau
            | Dialect::Teradata => "Community",
        }
    }

    /// Returns all dialect variants.
    #[must_use]
    pub fn all() -> &'static [Dialect] {
        &[
            Dialect::Ansi,
            Dialect::Athena,
            Dialect::BigQuery,
            Dialect::ClickHouse,
            Dialect::Databricks,
            Dialect::Doris,
            Dialect::Dremio,
            Dialect::Drill,
            Dialect::Druid,
            Dialect::DuckDb,
            Dialect::Exasol,
            Dialect::Fabric,
            Dialect::Hive,
            Dialect::Materialize,
            Dialect::Mysql,
            Dialect::Oracle,
            Dialect::Postgres,
            Dialect::Presto,
            Dialect::Prql,
            Dialect::Redshift,
            Dialect::RisingWave,
            Dialect::SingleStore,
            Dialect::Snowflake,
            Dialect::Spark,
            Dialect::Sqlite,
            Dialect::StarRocks,
            Dialect::Tableau,
            Dialect::Teradata,
            Dialect::Trino,
            Dialect::Tsql,
        ]
    }

    /// Parse a dialect name (case-insensitive) into a `Dialect`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Dialect> {
        match s.to_lowercase().as_str() {
            "" | "ansi" => Some(Dialect::Ansi),
            "athena" => Some(Dialect::Athena),
            "bigquery" => Some(Dialect::BigQuery),
            "clickhouse" => Some(Dialect::ClickHouse),
            "databricks" => Some(Dialect::Databricks),
            "doris" => Some(Dialect::Doris),
            "dremio" => Some(Dialect::Dremio),
            "drill" => Some(Dialect::Drill),
            "druid" => Some(Dialect::Druid),
            "duckdb" => Some(Dialect::DuckDb),
            "exasol" => Some(Dialect::Exasol),
            "fabric" => Some(Dialect::Fabric),
            "hive" => Some(Dialect::Hive),
            "materialize" => Some(Dialect::Materialize),
            "mysql" => Some(Dialect::Mysql),
            "oracle" => Some(Dialect::Oracle),
            "postgres" | "postgresql" => Some(Dialect::Postgres),
            "presto" => Some(Dialect::Presto),
            "prql" => Some(Dialect::Prql),
            "redshift" => Some(Dialect::Redshift),
            "risingwave" => Some(Dialect::RisingWave),
            "singlestore" => Some(Dialect::SingleStore),
            "snowflake" => Some(Dialect::Snowflake),
            "spark" => Some(Dialect::Spark),
            "sqlite" => Some(Dialect::Sqlite),
            "starrocks" => Some(Dialect::StarRocks),
            "tableau" => Some(Dialect::Tableau),
            "teradata" => Some(Dialect::Teradata),
            "trino" => Some(Dialect::Trino),
            "tsql" | "mssql" | "sqlserver" => Some(Dialect::Tsql),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Dialect families — helpers for grouping similar dialects
// ═══════════════════════════════════════════════════════════════════════════

/// Dialects in the MySQL family (use SUBSTR, IFNULL, similar type system).
pub(crate) fn is_mysql_family(d: Dialect) -> bool {
    matches!(
        d,
        Dialect::Mysql | Dialect::Doris | Dialect::SingleStore | Dialect::StarRocks
    )
}

/// Dialects in the Postgres family (support ILIKE, BYTEA, SUBSTRING).
pub(crate) fn is_postgres_family(d: Dialect) -> bool {
    matches!(
        d,
        Dialect::Postgres | Dialect::Redshift | Dialect::Materialize | Dialect::RisingWave
    )
}

/// Dialects in the Presto family (ANSI-like, VARCHAR oriented).
pub(crate) fn is_presto_family(d: Dialect) -> bool {
    matches!(d, Dialect::Presto | Dialect::Trino | Dialect::Athena)
}

/// Dialects in the Hive/Spark family (use STRING type, SUBSTR).
pub(crate) fn is_hive_family(d: Dialect) -> bool {
    matches!(d, Dialect::Hive | Dialect::Spark | Dialect::Databricks)
}

/// Dialects in the T-SQL family.
pub(crate) fn is_tsql_family(d: Dialect) -> bool {
    matches!(d, Dialect::Tsql | Dialect::Fabric)
}

/// Dialects that natively support ILIKE.
pub(crate) fn supports_ilike_builtin(d: Dialect) -> bool {
    matches!(
        d,
        Dialect::Postgres
            | Dialect::Redshift
            | Dialect::Materialize
            | Dialect::RisingWave
            | Dialect::DuckDb
            | Dialect::Snowflake
            | Dialect::ClickHouse
            | Dialect::Trino
            | Dialect::Presto
            | Dialect::Athena
            | Dialect::Databricks
            | Dialect::Spark
            | Dialect::Hive
            | Dialect::StarRocks
            | Dialect::Exasol
            | Dialect::Druid
            | Dialect::Dremio
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Statement / expression transforms
// ═══════════════════════════════════════════════════════════════════════════

/// Transform a statement from one dialect to another.
///
/// This applies dialect-specific rewrite rules such as:
/// - Type mapping (e.g., `TEXT` → `STRING` for BigQuery)
/// - Function name mapping (e.g., `NOW()` → `CURRENT_TIMESTAMP()`)
/// - ILIKE → LIKE with LOWER() wrapping for dialects that don't support ILIKE
#[must_use]
pub fn transform(statement: &Statement, from: Dialect, to: Dialect) -> Statement {
    transform_owned(statement.clone(), from, to)
}

/// Transform an owned statement from one dialect to another.
///
/// This is the hot path for `transpile`: parsing already returns an owned AST,
/// so callers that do not need to preserve the original statement can avoid a
/// whole-AST clone before dialect rewrites mutate the tree.
#[must_use]
pub fn transform_owned(statement: Statement, from: Dialect, to: Dialect) -> Statement {
    if from == to && !matches!(from, Dialect::Sqlite) {
        return statement;
    }
    let mut stmt = statement;
    transform_statement(&mut stmt, from, to);
    stmt
}

fn transform_statement(statement: &mut Statement, source: Dialect, target: Dialect) {
    match statement {
        Statement::Select(sel) => {
            // Transform LIMIT / TOP / FETCH FIRST for the target dialect
            transform_limit(sel, target);
            if matches!(target, Dialect::Sqlite) {
                sel.lock = None;
            }
            // Transform identifier quoting for the target dialect
            transform_quotes_in_select(sel, target);

            // Recurse into CTE bodies so inner SELECTs see the same
            // transforms (DATE 'x' → DATE('x'), ARRAY[…] → ARRAY(…), etc.).
            for cte in &mut sel.ctes {
                transform_statement(&mut cte.query, source, target);
            }

            for item in &mut sel.columns {
                if let SelectItem::Expr { expr, .. } = item {
                    transform_expr_in_place(expr, source, target);
                }
            }
            if let Some(wh) = &mut sel.where_clause {
                transform_expr_in_place(wh, source, target);
            }
            for gb in &mut sel.group_by {
                transform_expr_in_place(gb, source, target);
            }
            for expr in &mut sel.distinct_on {
                transform_expr_in_place(expr, source, target);
            }
            transform_order_by_items(&mut sel.order_by, source, target);
            for expr in &mut sel.limit_by {
                transform_expr_in_place(expr, source, target);
            }
            if let Some(having) = &mut sel.having {
                transform_expr_in_place(having, source, target);
            }
            if matches!(source, Dialect::Sqlite) && matches!(target, Dialect::Sqlite) {
                for join in &mut sel.joins {
                    if join.join_type == JoinType::Comma {
                        join.join_type = JoinType::Cross;
                    }
                    // SQLite parser synthesizes `ON TRUE` for outer/inner
                    // joins that don't carry an ON / USING clause.
                    if join.on.is_none()
                        && join.using.is_empty()
                        && matches!(
                            join.join_type,
                            JoinType::Inner
                                | JoinType::Left
                                | JoinType::LeftOuter
                                | JoinType::Right
                                | JoinType::RightOuter
                                | JoinType::Full
                                | JoinType::FullOuter
                        )
                    {
                        join.on = Some(Expr::Boolean(true));
                    }
                }
            }
            // Recurse into table sources to transform inner Expr nodes
            // (e.g. UNNEST(ARRAY_LITERAL) or UNNEST(GENERATE_DATE_ARRAY(
            // DATE 'x', INTERVAL 1 WEEK))) — these would otherwise miss
            // the per-expression rewrites.
            if let Some(from) = &mut sel.from {
                transform_exprs_in_table_source(&mut from.source, source, target);
            }
            for join in &mut sel.joins {
                transform_exprs_in_table_source(&mut join.table, source, target);
            }
            // Named WINDOW definitions' ORDER BY also needs NULLS
            // direction propagation for postgres source → sqlite.
            for wd in &mut sel.window_definitions {
                wd.spec = transform_window_spec(wd.spec.clone(), source, target);
            }
            // Rewrite SEMI/ANTI JOIN → WHERE EXISTS/NOT EXISTS subquery.
            rewrite_semi_anti_joins(sel);
            if let Some(rewritten) = rewrite_postgres_distinct_on(sel, source, target) {
                *sel = rewritten;
            }
        }
        Statement::Insert(ins) => {
            if let InsertSource::Values(rows) = &mut ins.source {
                for row in rows {
                    for val in row {
                        transform_expr_in_place(val, source, target);
                    }
                }
            }
            if let Some(on_conflict) = &mut ins.on_conflict {
                if let Some(where_expr) = &mut on_conflict.target_where {
                    transform_expr_in_place(where_expr, source, target);
                }
                if let ConflictAction::DoUpdate(assignments) = &mut on_conflict.action {
                    for (_, val) in assignments {
                        transform_expr_in_place(val, source, target);
                    }
                }
                if let Some(where_expr) = &mut on_conflict.action_where {
                    transform_expr_in_place(where_expr, source, target);
                }
            }
        }
        Statement::Update(upd) => {
            for (_, val) in &mut upd.assignments {
                transform_expr_in_place(val, source, target);
            }
            if let Some(wh) = &mut upd.where_clause {
                transform_expr_in_place(wh, source, target);
            }
        }
        Statement::Expression(expr) => {
            transform_expr_in_place(expr, source, target);
            // Statement-level REPLACE(...) is unsupported syntax in SQLGlot's
            // MySQL / SQLite parsers, so Python falls back to the Command
            // parser and re-renders with a space before the open paren. We
            // mirror that for non-postgres sources targeting SQLite to keep
            // parity. Postgres sources keep their REPLACE() form unchanged.
            if matches!(target, Dialect::Sqlite)
                && !is_postgres_family(source)
                && let Some(text) = replace_statement_to_command_form(expr)
            {
                *statement = Statement::Raw(RawStatement {
                    comments: vec![],
                    sql: text,
                    source_dialect: Some(source),
                });
            }
        }
        // DDL: map data types in CREATE TABLE column definitions
        Statement::CreateTable(ct) => {
            for col in &mut ct.columns {
                if col.name_quote_style.is_quoted() {
                    col.name_quote_style = QuoteStyle::for_dialect(target);
                }
                if let Some(default) = &mut col.default {
                    transform_expr_in_place(default, source, target);
                }
                if let Some(generated) = &mut col.generated_as {
                    transform_expr_in_place(generated, source, target);
                }
            }
            // Transform constraints (CHECK expressions)
            for constraint in &mut ct.constraints {
                if let TableConstraint::Check { expr, .. } = constraint {
                    transform_expr_in_place(expr, source, target);
                }
            }
            // Transform AS SELECT subquery
            if let Some(as_select) = &mut ct.as_select {
                transform_statement(as_select, source, target);
            }
        }
        // DDL: map data types in ALTER TABLE ADD COLUMN
        Statement::AlterTable(alt) => {
            for action in &mut alt.actions {
                match action {
                    AlterTableAction::AddColumn(col) => {
                        if let Some(default) = &mut col.default {
                            transform_expr_in_place(default, source, target);
                        }
                    }
                    AlterTableAction::ChangeColumn { new_column, .. } => {
                        if let Some(default) = &mut new_column.default {
                            transform_expr_in_place(default, source, target);
                        }
                    }
                    _ => {}
                }
            }
        }
        Statement::CreateIndex(idx) => {
            transform_order_by_items(&mut idx.columns, source, target);
            if let Some(predicate) = &mut idx.where_clause {
                transform_expr_in_place(predicate, source, target);
            }
        }
        Statement::CreateFunction(func) => {
            for param in &mut func.params {
                if let Some(default) = &mut param.default {
                    transform_expr_in_place(default, source, target);
                }
            }
        }
        Statement::Raw(raw) if raw.source_dialect.is_none() => {
            raw.source_dialect = Some(source);
        }
        _ => {}
    }
}

/// Returns true if a SHOW statement is one of the forms the mysql
/// parser in Python SQLGlot recognizes (and therefore drops when
/// transpiling to sqlite). Unrecognized SHOWs fall back to Command and
/// are passed through verbatim.
pub(crate) fn mysql_show_is_recognized(trimmed: &str) -> bool {
    // Skip past "SHOW".
    let rest = trimmed[4..].trim_start();
    // Strip leading FULL/EXTENDED/ALL/GLOBAL/SESSION/etc.
    let upper = rest.to_ascii_uppercase();
    let first_word = upper.split_whitespace().next().unwrap_or("");
    matches!(
        first_word,
        "TABLES"
            | "TABLE"
            | "DATABASES"
            | "SCHEMAS"
            | "COLUMNS"
            | "FIELDS"
            | "INDEX"
            | "INDEXES"
            | "INDICES"
            | "KEYS"
            | "VARIABLES"
            | "STATUS"
            | "PROCESSLIST"
            | "GRANTS"
            | "PRIVILEGES"
            | "ENGINE"
            | "ENGINES"
            | "EVENTS"
            | "FUNCTION"
            | "PROCEDURE"
            | "TRIGGER"
            | "TRIGGERS"
            | "WARNINGS"
            | "ERRORS"
            | "PLUGINS"
            | "CHARACTER"
            | "CHARSET"
            | "COLLATION"
            | "PROFILE"
            | "PROFILES"
            | "OPEN"
            | "MASTER"
            | "SLAVE"
            | "REPLICA"
            | "BINARY"
            | "BINLOG"
            | "RELAYLOG"
            | "CREATE"
            | "FULL"
            | "EXTENDED"
            | "GLOBAL"
            | "SESSION"
            | "LOCAL"
    )
}

fn replace_statement_to_command_form(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Function { name, args, .. } if name.eq_ignore_ascii_case("REPLACE") => {
            let mut buf = String::from("REPLACE (");
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    buf.push_str(", ");
                }
                render_expr_short(arg, &mut buf);
            }
            buf.push(')');
            Some(buf)
        }
        Expr::TypedFunction {
            func: TypedFunction::Replace { expr, from, to },
            ..
        } => {
            let mut buf = String::from("REPLACE (");
            render_expr_short(expr, &mut buf);
            buf.push_str(", ");
            render_expr_short(from, &mut buf);
            buf.push_str(", ");
            render_expr_short(to, &mut buf);
            buf.push(')');
            Some(buf)
        }
        _ => None,
    }
}

fn render_expr_short(expr: &Expr, out: &mut String) {
    match expr {
        Expr::Column { name, .. } => out.push_str(name),
        Expr::StringLiteral(s) => {
            out.push('\'');
            out.push_str(&s.replace('\'', "''"));
            out.push('\'');
        }
        Expr::Number(n) => out.push_str(n),
        _ => {
            // Fallback: best-effort literal preservation. Anything more
            // complex than a simple identifier / literal would have been
            // parsed differently anyway.
            out.push_str(&format!("{expr:?}"));
        }
    }
}

pub(crate) fn normalize_postgres_create_type_enum(sql: &str) -> Option<String> {
    let trimmed = sql.trim();
    let upper = trimmed.to_ascii_uppercase();
    let create_type = "CREATE TYPE ";
    let as_enum = " AS ENUM";
    if !upper.starts_with(create_type) {
        return None;
    }
    let as_enum_index = upper.find(as_enum)?;
    let name = trimmed[create_type.len()..as_enum_index].trim();
    if name.is_empty() {
        return None;
    }
    let values = trimmed[as_enum_index + as_enum.len()..].trim_start();
    if !values.starts_with('(') {
        return None;
    }
    Some(format!("CREATE TYPE {name} AS ENUM{values}"))
}

pub(crate) fn normalize_postgres_recursive_cte_raw(sql: &str) -> String {
    let upper = sql.to_ascii_uppercase();
    if !(upper.contains(" SEARCH ") && upper.contains(" ORDER BY ")) {
        return sql.to_string();
    }
    if upper.contains(" NULLS FIRST") || upper.contains(" NULLS LAST") {
        return sql.to_string();
    }
    let Some(order_index) = upper.rfind(" ORDER BY ") else {
        return sql.to_string();
    };
    let mut out = String::with_capacity(sql.len() + " NULLS LAST".len());
    out.push_str(&sql[..order_index]);
    out.push_str(" ORDER BY ");
    let order_expr_start = order_index + " ORDER BY ".len();
    out.push_str(sql[order_expr_start..].trim_end());
    out.push_str(" NULLS LAST");
    out
}

pub(crate) fn normalize_postgres_copy_raw(sql: &str) -> String {
    let trimmed = sql.trim_start();
    // SQLGlot renders a COPY statement as `COPY INTO ...` for the sqlite
    // target. Insert INTO after COPY unless it's already there.
    if trimmed
        .get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("COPY "))
        && !trimmed
            .get(..10)
            .is_some_and(|p| p.eq_ignore_ascii_case("COPY INTO "))
    {
        let leading_len = sql.len() - trimmed.len();
        let mut out = String::with_capacity(sql.len() + " INTO".len());
        out.push_str(&sql[..leading_len]);
        out.push_str("COPY INTO ");
        out.push_str(&trimmed["COPY ".len()..]);
        out
    } else {
        sql.to_string()
    }
}

/// `INSERT INTO [TABLE] FUNCTION <name>(...)` drops the optional TABLE
/// keyword and uppercases the table-function name for the sqlite target.
pub(crate) fn normalize_insert_into_function(sql: &str) -> String {
    let upper = sql.to_ascii_uppercase();
    let trimmed_upper = upper.trim_start();
    if !trimmed_upper.starts_with("INSERT INTO ") {
        return sql.to_string();
    }
    let lead = sql.len() - sql.trim_start().len();
    let after_into = lead + "INSERT INTO ".len();
    let mut rest = &sql[after_into..];
    let mut out = sql[..after_into].to_string();
    // Optional TABLE keyword before FUNCTION.
    if rest.trim_start().to_ascii_uppercase().starts_with("TABLE ") {
        let r = rest.trim_start();
        rest = &r["TABLE ".len()..];
    }
    let rest_trim = rest.trim_start();
    if !rest_trim.to_ascii_uppercase().starts_with("FUNCTION ") {
        return sql.to_string();
    }
    out.push_str("FUNCTION ");
    let after_fn = &rest_trim["FUNCTION ".len()..];
    // Uppercase the function name (up to the opening paren / whitespace).
    let name_end = after_fn
        .find(|c: char| c == '(' || c.is_whitespace())
        .unwrap_or(after_fn.len());
    out.push_str(&after_fn[..name_end].to_ascii_uppercase());
    out.push_str(&after_fn[name_end..]);
    out
}

pub(crate) fn uppercase_function_names_in_raw_sql(sql: &str) -> String {
    // Walk the string; whenever we see an unquoted identifier
    // immediately followed by `(`, uppercase the identifier — unless
    // the identifier is acting as an alias (preceded by `AS `).
    // Quoted identifiers and string literals are preserved verbatim.
    let bytes = sql.as_bytes();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0;
    let mut last_ident_upper = String::new();
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '\'' || c == '"' {
            let quote = c;
            out.push(c);
            i += 1;
            while i < bytes.len() {
                let cc = bytes[i] as char;
                out.push(cc);
                i += 1;
                if cc == quote {
                    if i < bytes.len() && bytes[i] as char == quote {
                        out.push(quote);
                        i += 1;
                        continue;
                    }
                    break;
                }
            }
            last_ident_upper.clear();
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < bytes.len() && {
                let cc = bytes[i] as char;
                cc.is_ascii_alphanumeric() || cc == '_'
            } {
                i += 1;
            }
            let ident = &sql[start..i];
            let mut j = i;
            while j < bytes.len() && (bytes[j] as char).is_whitespace() {
                j += 1;
            }
            let preceded_by_as = last_ident_upper == "AS";
            if j < bytes.len() && bytes[j] as char == '(' && !preceded_by_as {
                out.push_str(&ident.to_ascii_uppercase());
            } else {
                out.push_str(ident);
            }
            last_ident_upper = ident.to_ascii_uppercase();
            continue;
        }
        if !c.is_whitespace() {
            last_ident_upper.clear();
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Rewrites typed-literal patterns inside Raw table source text:
///   DATE 'literal'       → DATE('literal')
///   TIMESTAMP 'literal'  → CAST('literal' AS TIMESTAMP)
///   INTERVAL N UNIT      → INTERVAL 'N' UNIT  (number → string literal)
pub(crate) fn normalize_typed_literals_in_raw_sql(sql: &str) -> String {
    let mut out = String::with_capacity(sql.len() + 16);
    let bytes = sql.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        // Skip string literals.
        if c == '\'' {
            out.push(c);
            i += 1;
            while i < bytes.len() {
                let cc = bytes[i] as char;
                out.push(cc);
                i += 1;
                if cc == '\'' {
                    if i < bytes.len() && bytes[i] as char == '\'' {
                        out.push('\'');
                        i += 1;
                        continue;
                    }
                    break;
                }
            }
            continue;
        }
        // Identifier-like keywords.
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < bytes.len() && {
                let cc = bytes[i] as char;
                cc.is_ascii_alphanumeric() || cc == '_'
            } {
                i += 1;
            }
            let ident = &sql[start..i];
            let upper = ident.to_ascii_uppercase();
            // DATE 'literal' / TIMESTAMP 'literal'
            if matches!(upper.as_str(), "DATE" | "TIMESTAMP") {
                let mut j = i;
                while j < bytes.len() && (bytes[j] as char).is_whitespace() {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] as char == '\'' {
                    // Find closing quote.
                    let lit_start = j + 1;
                    let mut k = lit_start;
                    while k < bytes.len() {
                        if bytes[k] as char == '\'' {
                            if k + 1 < bytes.len() && bytes[k + 1] as char == '\'' {
                                k += 2;
                                continue;
                            }
                            break;
                        }
                        k += 1;
                    }
                    if k < bytes.len() && bytes[k] as char == '\'' {
                        let lit = &sql[lit_start..k];
                        if upper == "DATE" {
                            out.push_str("DATE('");
                            out.push_str(lit);
                            out.push_str("')");
                        } else {
                            out.push_str("CAST('");
                            out.push_str(lit);
                            out.push_str("' AS TIMESTAMP)");
                        }
                        i = k + 1;
                        continue;
                    }
                }
                out.push_str(ident);
                continue;
            }
            // INTERVAL N UNIT — quote the numeric N.
            if upper == "INTERVAL" {
                out.push_str(ident);
                // Skip whitespace.
                while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                    out.push(bytes[i] as char);
                    i += 1;
                }
                // If next is a bare number, quote it.
                if i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                    let num_start = i;
                    while i < bytes.len() && {
                        let cc = bytes[i] as char;
                        cc.is_ascii_digit() || cc == '.'
                    } {
                        i += 1;
                    }
                    out.push('\'');
                    out.push_str(&sql[num_start..i]);
                    out.push('\'');
                }
                continue;
            }
            out.push_str(ident);
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

/// For postgres/mysql source → sqlite target, `UNNEST([items])`
/// becomes `UNNEST(ARRAY(items))` — Python SQLGlot wraps the
/// bracketed list in an ARRAY function call. Handles nested brackets
/// inside the outer `[…]` by tracking depth.
pub(crate) fn rewrite_unnest_array_literal_to_array_call(sql: &str) -> Option<String> {
    let upper = sql.to_ascii_uppercase();
    let unnest_pos = upper.find("UNNEST(")?;
    let after_lparen = unnest_pos + "UNNEST".len() + 1;
    let body_start = sql[after_lparen..]
        .find(|c: char| !c.is_whitespace())
        .map(|o| after_lparen + o)?;
    if !sql[body_start..].starts_with('[') {
        return None;
    }
    // Find matching `]` at depth 0.
    let bytes = sql.as_bytes();
    let mut depth = 0i32;
    let mut close = body_start;
    let mut i = body_start;
    while i < sql.len() {
        match bytes[i] {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    close = i;
                    break;
                }
            }
            _ => {}
        }
        i += 1;
    }
    if depth != 0 {
        return None;
    }
    let inside = &sql[body_start + 1..close];
    let mut out = String::with_capacity(sql.len() + 6);
    out.push_str(&sql[..body_start]);
    out.push_str("ARRAY(");
    out.push_str(inside);
    out.push(')');
    out.push_str(&sql[close + 1..]);
    Some(out)
}

/// For sqlite source → sqlite target, `UNNEST([1, 2, 3])` becomes
/// `UNNEST("1, 2, 3")` (Python's array-literal-to-quoted-string
/// fallback applied inside UNNEST). Only the outermost `[…]`
/// immediately inside the UNNEST( … ) is converted.
pub(crate) fn rewrite_unnest_array_literal_sqlite(sql: &str) -> Option<String> {
    let upper = sql.to_ascii_uppercase();
    let unnest_pos = upper.find("UNNEST(")?;
    let lparen_abs = unnest_pos + "UNNEST".len();
    // Find the next non-whitespace char after `(`.
    let after_lparen = lparen_abs + 1;
    let body_start = sql[after_lparen..]
        .find(|c: char| !c.is_whitespace())
        .map(|o| after_lparen + o)?;
    if !sql[body_start..].starts_with('[') {
        return None;
    }
    // Find matching `]` (no nesting for arrays in this form).
    let close_bracket = sql[body_start + 1..]
        .find(']')
        .map(|o| body_start + 1 + o)?;
    let inside = &sql[body_start + 1..close_bracket];
    let mut out = String::with_capacity(sql.len());
    out.push_str(&sql[..body_start]);
    out.push('"');
    out.push_str(inside);
    out.push('"');
    out.push_str(&sql[close_bracket + 1..]);
    Some(out)
}

/// Rewrites BigQuery `UNNEST(expr) [AS alias] WITH OFFSET [AS pos]` into
/// Postgres-style `UNNEST(expr) WITH ORDINALITY AS alias` (Python
/// SQLGlot's sqlite output). The offset alias is dropped.
pub(crate) fn rewrite_unnest_with_offset(sql: &str) -> Option<String> {
    let upper = sql.to_ascii_uppercase();
    let unnest_pos = upper.find("UNNEST")?;
    let with_offset_pos = find_case_insensitive(sql, "WITH OFFSET")?;
    if with_offset_pos < unnest_pos {
        return None;
    }
    // Find closing paren of UNNEST(...).
    let after_unnest = unnest_pos + "UNNEST".len();
    let lparen = sql[after_unnest..].find('(')?;
    let lparen_abs = after_unnest + lparen;
    let mut depth = 0i32;
    let bytes = sql.as_bytes();
    let mut rparen_abs = lparen_abs;
    for (i, byte) in bytes.iter().enumerate().skip(lparen_abs) {
        match *byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    rparen_abs = i;
                    break;
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return None;
    }
    // Slice [unnest_pos .. rparen_abs+1] is `UNNEST(...)`. The tail
    // between rparen_abs+1 and with_offset_pos may carry `AS alias`.
    let unnest_call = &sql[unnest_pos..=rparen_abs];
    let tail_before = sql[rparen_abs + 1..with_offset_pos].trim();
    let alias_clause = if let Some(rest) = tail_before
        .strip_prefix("AS ")
        .or_else(|| tail_before.strip_prefix("as "))
    {
        format!(" AS {}", rest.trim())
    } else if tail_before.is_empty() {
        String::new()
    } else {
        format!(" AS {tail_before}")
    };
    // Drop any `AS pos` suffix after WITH OFFSET — Python SQLGlot's
    // sqlite output discards the offset column alias.
    let mut out = String::with_capacity(sql.len());
    out.push_str(&sql[..unnest_pos]);
    out.push_str(unnest_call);
    out.push_str(" WITH ORDINALITY");
    out.push_str(&alias_clause);
    Some(out)
}

pub(crate) fn strip_postgres_values_column_aliases(sql: &str) -> String {
    if find_case_insensitive(sql, "VALUES").is_none() {
        return sql.to_string();
    }

    let mut out = String::with_capacity(sql.len());
    let mut index = 0usize;
    while index < sql.len() {
        let remaining = &sql[index..];
        let Some(as_pos) = find_case_insensitive(remaining, " AS ") else {
            out.push_str(remaining);
            break;
        };
        let absolute_as = index + as_pos;
        out.push_str(&sql[index..absolute_as + 4]);
        let mut cursor = absolute_as + 4;
        while let Some(ch) = sql[cursor..].chars().next()
            && (ch.is_ascii_alphanumeric() || ch == '_' || ch == '"' || ch == '`')
        {
            out.push(ch);
            cursor += ch.len_utf8();
        }
        let alias_end = cursor;
        while let Some(ch) = sql[cursor..].chars().next()
            && ch.is_ascii_whitespace()
        {
            cursor += ch.len_utf8();
        }
        if sql[cursor..].starts_with('(') {
            let mut depth = 0usize;
            let mut scan = cursor;
            while let Some(ch) = sql[scan..].chars().next() {
                scan += ch.len_utf8();
                if ch == '(' {
                    depth += 1;
                } else if ch == ')' {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        cursor = scan;
                        break;
                    }
                }
            }
            index = cursor;
        } else {
            index = alias_end;
        }
    }
    out
}

fn find_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .as_bytes()
        .windows(needle.len())
        .position(|window| window.eq_ignore_ascii_case(needle.as_bytes()))
}

/// Transform an expression for the target dialect.
fn transform_expr(expr: Expr, source: Dialect, target: Dialect) -> Expr {
    match expr {
        // Map function names across dialects
        Expr::Function {
            name,
            args,
            distinct,
            filter,
            over,
        } => {
            let new_args: Vec<Expr> = args
                .into_iter()
                .map(|a| transform_expr(a, source, target))
                .collect();
            // Postgres source aggregate calls parsed with raw-string args
            // (ARRAY_AGG, ANY_VALUE, ARG_MAX, JSON_ARRAYAGG, LAST_VALUE,
            // LISTAGG, NTILE) — when the raw text contains an ORDER BY
            // clause, propagate NULLS FIRST/LAST direction onto every
            // explicit ORDER BY item (postgres semantics: ASC = NULLS
            // LAST, DESC = NULLS FIRST). Python SQLGlot does this on
            // sqlite output.
            if is_postgres_family(source)
                && matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "ANY_VALUE"
                        | "ARG_MAX"
                        | "ARRAY_AGG"
                        | "JSON_ARRAYAGG"
                        | "LAST_VALUE"
                        | "LISTAGG"
                        | "NTILE"
                )
                && new_args.len() == 1
                && let Expr::StringLiteral(raw) = &new_args[0]
                && !raw.contains("NULLS FIRST")
                && !raw.contains("NULLS LAST")
                && raw.to_ascii_uppercase().contains(" ORDER BY ")
            {
                let rewritten = propagate_nulls_direction(raw);
                return Expr::Function {
                    name,
                    args: vec![Expr::StringLiteral(rewritten)],
                    distinct,
                    filter,
                    over,
                };
            }
            Expr::Function {
                name,
                args: new_args,
                distinct,
                filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                over: over.map(|spec| transform_window_spec(spec, source, target)),
            }
        }
        // Recurse into typed function child expressions; source-native
        // format strings are canonicalized by the parser and rendered by
        // the generator.
        Expr::TypedFunction { func, filter, over } => {
            let transformed_func = transform_typed_function(func, source, target);
            Expr::TypedFunction {
                func: transformed_func,
                filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                over: over.map(|spec| transform_window_spec(spec, source, target)),
            }
        }
        Expr::Like {
            expr,
            pattern,
            negated,
            escape,
        } => Expr::Like {
            expr: Box::new(transform_expr(*expr, source, target)),
            pattern: Box::new(transform_expr(*pattern, source, target)),
            negated,
            escape: escape.map(|e| Box::new(transform_expr(*e, source, target))),
        },
        // ILIKE → LOWER(expr) LIKE LOWER(pattern) for non-supporting dialects
        Expr::ILike {
            expr,
            pattern,
            negated,
            escape,
        } if !supports_ilike_builtin(target) => Expr::Like {
            expr: Box::new(Expr::TypedFunction {
                func: TypedFunction::Lower {
                    expr: Box::new(transform_expr(*expr, source, target)),
                },
                filter: None,
                over: None,
            }),
            pattern: Box::new(Expr::TypedFunction {
                func: TypedFunction::Lower {
                    expr: Box::new(transform_expr(*pattern, source, target)),
                },
                filter: None,
                over: None,
            }),
            negated,
            escape,
        },
        Expr::SimilarTo {
            expr,
            pattern,
            escape,
        } => Expr::SimilarTo {
            expr: Box::new(transform_expr(*expr, source, target)),
            pattern: Box::new(transform_expr(*pattern, source, target)),
            escape: escape.map(|e| Box::new(transform_expr(*e, source, target))),
        },
        // Map data types in CAST
        Expr::Cast { expr, data_type } => {
            let expr = transform_expr(*expr, source, target);
            Expr::Cast {
                expr: Box::new(expr),
                data_type,
            }
        }
        Expr::Extract { field, expr } => Expr::Extract {
            field,
            expr: Box::new(transform_expr(*expr, source, target)),
        },
        // Recurse into binary ops
        Expr::BinaryOp { left, op, right } => {
            let left = transform_expr(*left, source, target);
            let right = transform_expr(*right, source, target);
            Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }
        Expr::UnaryOp { op, expr } => Expr::UnaryOp {
            op,
            expr: Box::new(transform_expr(*expr, source, target)),
        },
        Expr::If {
            condition,
            true_val,
            false_val,
        } => Expr::If {
            condition: Box::new(transform_expr(*condition, source, target)),
            true_val: Box::new(transform_expr(*true_val, source, target)),
            false_val: false_val.map(|expr| Box::new(transform_expr(*expr, source, target))),
        },
        Expr::Coalesce {
            items,
            is_nvl,
            is_null,
            source_name,
        } => Expr::Coalesce {
            items: items
                .into_iter()
                .map(|item| transform_expr(item, source, target))
                .collect(),
            is_nvl,
            is_null,
            source_name,
        },
        Expr::Interval {
            value,
            unit,
            unit_text,
        } => transform_interval(*value, unit, unit_text, source, target),
        Expr::ArrayLiteral(items) => {
            let items: Vec<Expr> = items
                .into_iter()
                .map(|item| transform_expr(item, source, target))
                .collect();
            Expr::ArrayLiteral(items)
        }
        Expr::SqliteArrayLiteral(items) => {
            let items: Vec<Expr> = items
                .into_iter()
                .map(|item| transform_expr(item, source, target))
                .collect();
            Expr::SqliteArrayLiteral(items)
        }
        Expr::ArrayIndex { expr, index } => Expr::ArrayIndex {
            expr: Box::new(transform_expr(*expr, source, target)),
            index: Box::new(transform_expr(*index, source, target)),
        },
        Expr::PostgresArrayIndex { expr, index } => Expr::PostgresArrayIndex {
            expr: Box::new(transform_expr(*expr, source, target)),
            index: Box::new(transform_expr(*index, source, target)),
        },
        Expr::JsonAccess {
            expr,
            path,
            as_text,
        } => Expr::JsonAccess {
            expr: Box::new(transform_expr(*expr, source, target)),
            path: Box::new(transform_expr(*path, source, target)),
            as_text,
        },
        Expr::Alias { expr, name } => Expr::Alias {
            expr: Box::new(transform_expr(*expr, source, target)),
            name,
        },
        Expr::Nested(inner) => Expr::Nested(Box::new(transform_expr(*inner, source, target))),
        Expr::WithinGroup {
            expr,
            mut order_by,
            filter,
            over,
        } => {
            transform_order_by_items(&mut order_by, source, target);
            let inner = transform_expr(*expr, source, target);
            // SQLite GROUP_CONCAT (the lowered form of STRING_AGG) doesn't
            // support WITHIN GROUP; SQLGlot drops the clause for that
            // function only. Other functions (PERCENTILE_CONT, LISTAGG,
            // etc.) keep WITHIN GROUP.
            if matches!(target, Dialect::Sqlite)
                && let Expr::Function { name, .. } = &inner
                && (name.eq_ignore_ascii_case("GROUP_CONCAT")
                    || name.eq_ignore_ascii_case("STRING_AGG"))
            {
                return inner;
            }
            Expr::WithinGroup {
                expr: Box::new(inner),
                order_by,
                filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                over: over.map(|spec| transform_window_spec(spec, source, target)),
            }
        }
        // Transform quoting on column references
        Expr::Column {
            table,
            name,
            quote_style,
            table_quote_style,
        } => {
            let new_qs = if quote_style.is_quoted() {
                QuoteStyle::for_dialect(target)
            } else {
                QuoteStyle::None
            };
            let new_tqs = if table_quote_style.is_quoted() {
                QuoteStyle::for_dialect(target)
            } else {
                QuoteStyle::None
            };
            Expr::Column {
                table,
                name,
                quote_style: new_qs,
                table_quote_style: new_tqs,
            }
        }
        Expr::Exists {
            mut subquery,
            negated,
        } => {
            transform_statement(&mut subquery, source, target);
            Expr::Exists { subquery, negated }
        }
        Expr::Subquery(mut stmt) => {
            transform_statement(&mut stmt, source, target);
            Expr::Subquery(stmt)
        }
        Expr::InSubquery {
            expr,
            mut subquery,
            negated,
        } => {
            transform_statement(&mut subquery, source, target);
            Expr::InSubquery {
                expr: Box::new(transform_expr(*expr, source, target)),
                subquery,
                negated,
            }
        }
        // Everything else stays the same
        other => other,
    }
}

fn transform_expr_in_place(expr: &mut Expr, source: Dialect, target: Dialect) {
    let old = std::mem::replace(expr, Expr::Null);
    *expr = transform_expr(old, source, target);
}

fn transform_window_spec(mut spec: WindowSpec, source: Dialect, target: Dialect) -> WindowSpec {
    for expr in &mut spec.partition_by {
        transform_expr_in_place(expr, source, target);
    }
    transform_order_by_items(&mut spec.order_by, source, target);
    spec
}

fn transform_order_by_items(items: &mut [OrderByItem], source: Dialect, target: Dialect) {
    for item in items {
        transform_expr_in_place(&mut item.expr, source, target);
    }
}

/// Rewrite `SEMI JOIN` / `ANTI JOIN` clauses to `WHERE EXISTS (...)`
/// / `WHERE NOT EXISTS (...)` subqueries (Python SQLGlot's IR form).
/// Add NULLS FIRST/LAST direction to each ORDER BY item in a raw
/// aggregate-args string. Used for postgres source aggregates whose
/// args are parsed as raw text (ARRAY_AGG with ORDER BY, etc.).
fn propagate_nulls_direction(raw: &str) -> String {
    let upper = raw.to_ascii_uppercase();
    let Some(order_by_pos) = upper.find(" ORDER BY ") else {
        return raw.to_string();
    };
    let order_start = order_by_pos + " ORDER BY ".len();
    // The ORDER BY clause runs from order_start until the first LIMIT/
    // OFFSET keyword at the top level (depth 0).
    let bytes = raw.as_bytes();
    let upper_bytes = upper.as_bytes();
    let mut depth: i32 = 0;
    let mut end = raw.len();
    let mut i = order_start;
    while i < raw.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => {
                if depth == 0 {
                    end = i;
                    break;
                }
                depth -= 1;
            }
            _ if depth == 0
                && (upper_bytes[i..].starts_with(b" LIMIT ")
                    || upper_bytes[i..].starts_with(b" OFFSET ")) =>
            {
                end = i;
                break;
            }
            _ => {}
        }
        i += 1;
    }
    let order_clause = &raw[order_start..end];
    let rewritten = order_clause
        .split(',')
        .map(|item| {
            let trimmed = item.trim_end();
            let leading_spaces: String = item.chars().take_while(|c| c.is_whitespace()).collect();
            let core = trimmed.trim_start();
            // Detect parenthesized expr like `(a + b) DESC` — strip
            // any trailing direction keyword to decide.
            let upper_core = core.to_ascii_uppercase();
            let is_desc = upper_core.ends_with(" DESC");
            let already_has_nulls =
                upper_core.ends_with(" NULLS FIRST") || upper_core.ends_with(" NULLS LAST");
            if already_has_nulls || core.is_empty() {
                return format!("{leading_spaces}{core}");
            }
            let suffix = if is_desc {
                " NULLS FIRST"
            } else {
                " NULLS LAST"
            };
            format!("{leading_spaces}{core}{suffix}")
        })
        .collect::<Vec<_>>()
        .join(",");
    let mut out = String::with_capacity(raw.len() + 32);
    out.push_str(&raw[..order_start]);
    out.push_str(&rewritten);
    out.push_str(&raw[end..]);
    out
}

fn rewrite_semi_anti_joins(sel: &mut SelectStatement) {
    let mut new_joins = Vec::with_capacity(sel.joins.len());
    for join in std::mem::take(&mut sel.joins) {
        let negated = match join.join_type {
            JoinType::Semi => false,
            JoinType::Anti => true,
            _ => {
                new_joins.push(join);
                continue;
            }
        };
        // Build subquery: SELECT 1 FROM <table> [WHERE on]
        let subquery_select = SelectStatement {
            comments: vec![],
            ctes: vec![],
            distinct: false,
            distinct_on: vec![],
            top: None,
            columns: vec![SelectItem::Expr {
                expr: Expr::Number("1".to_string()),
                alias: None,
                alias_quote_style: QuoteStyle::None,
            }],
            from: Some(FromClause { source: join.table }),
            joins: vec![],
            where_clause: join.on,
            group_by: vec![],
            having: None,
            order_by: vec![],
            limit: None,
            offset: None,
            limit_by: vec![],
            fetch_first: None,
            qualify: None,
            window_definitions: vec![],
            lock: None,
        };
        let exists_expr = Expr::Exists {
            subquery: Box::new(Statement::Select(subquery_select)),
            negated,
        };
        sel.where_clause = Some(match sel.where_clause.take() {
            Some(existing) => Expr::BinaryOp {
                left: Box::new(existing),
                op: BinaryOperator::And,
                right: Box::new(exists_expr),
            },
            None => exists_expr,
        });
    }
    sel.joins = new_joins;
}

fn rewrite_postgres_distinct_on(
    sel: &SelectStatement,
    _source: Dialect,
    target: Dialect,
) -> Option<SelectStatement> {
    // SQLGlot lowers DISTINCT ON to a ROW_NUMBER() window for every source
    // dialect when the target can't represent it (the NULLS direction in the
    // window order is already carried by sel.order_by — present for postgres,
    // absent for mysql/sqlite).
    if !matches!(target, Dialect::Sqlite) || sel.distinct_on.is_empty() {
        return None;
    }

    let mut inner_columns = Vec::with_capacity(sel.columns.len() + 1);
    let mut outer_columns = Vec::with_capacity(sel.columns.len());
    let mut has_wildcard = false;

    for item in &sel.columns {
        match item {
            SelectItem::Wildcard => {
                has_wildcard = true;
                inner_columns.push(SelectItem::Wildcard);
            }
            SelectItem::QualifiedWildcard { table } => {
                has_wildcard = true;
                inner_columns.push(SelectItem::QualifiedWildcard {
                    table: table.clone(),
                });
            }
            SelectItem::Expr { expr, alias, .. } if has_wildcard => {
                inner_columns.push(SelectItem::Expr {
                    expr: expr.clone(),
                    alias: alias.clone(),
                    alias_quote_style: QuoteStyle::None,
                });
            }
            SelectItem::Expr { expr, alias, .. } => {
                let output_name = alias
                    .clone()
                    .or_else(|| column_name(expr))
                    .unwrap_or_else(|| generated_column_alias(inner_columns.len()));

                inner_columns.push(SelectItem::Expr {
                    expr: expr.clone(),
                    alias: Some(output_name.clone()),
                    alias_quote_style: QuoteStyle::None,
                });
                outer_columns.push(SelectItem::Expr {
                    expr: column_expr(&output_name),
                    alias: None,
                    alias_quote_style: QuoteStyle::None,
                });
            }
        }
    }

    if has_wildcard {
        outer_columns = vec![SelectItem::Wildcard];
    }

    let order_by = if sel.order_by.is_empty() {
        sel.distinct_on
            .iter()
            .cloned()
            .map(|expr| OrderByItem {
                expr,
                ascending: true,
                explicit_direction: false,
                nulls_first: None,
                implicit_nulls: false,
            })
            .collect()
    } else {
        sel.order_by.clone()
    };

    inner_columns.push(SelectItem::Expr {
        expr: Expr::TypedFunction {
            func: TypedFunction::RowNumber,
            filter: None,
            over: Some(WindowSpec {
                window_ref: None,
                partition_by: sel.distinct_on.clone(),
                order_by,
                frame: None,
            }),
        },
        alias: Some("_row_number".to_string()),
        alias_quote_style: QuoteStyle::None,
    });

    let mut inner = sel.clone();
    inner.distinct = false;
    inner.distinct_on.clear();
    inner.columns = inner_columns;
    inner.order_by.clear();

    Some(SelectStatement {
        comments: vec![],
        ctes: vec![],
        distinct: false,
        distinct_on: vec![],
        top: None,
        columns: outer_columns,
        from: Some(FromClause {
            source: TableSource::Subquery {
                query: Box::new(Statement::Select(inner)),
                alias: Some("_t".to_string()),
                alias_quote_style: QuoteStyle::None,
            },
        }),
        joins: vec![],
        where_clause: Some(Expr::BinaryOp {
            left: Box::new(column_expr("_row_number")),
            op: BinaryOperator::Eq,
            right: Box::new(Expr::Number("1".to_string())),
        }),
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None,
        offset: None,
        limit_by: vec![],
        fetch_first: None,
        qualify: None,
        window_definitions: vec![],
        lock: None,
    })
}

fn column_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Column { name, .. } => Some(name.clone()),
        _ => None,
    }
}

fn generated_column_alias(index: usize) -> String {
    if index == 0 {
        "_col".to_string()
    } else {
        format!("_col_{}", index + 1)
    }
}

fn column_expr(name: &str) -> Expr {
    Expr::Column {
        table: None,
        name: name.to_string(),
        quote_style: QuoteStyle::None,
        table_quote_style: QuoteStyle::None,
    }
}

fn transform_interval(
    value: Expr,
    unit: Option<DateTimeField>,
    unit_text: Option<String>,
    source: Dialect,
    target: Dialect,
) -> Expr {
    Expr::Interval {
        value: Box::new(transform_expr(value, source, target)),
        unit,
        unit_text,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Typed function transformation
// ═══════════════════════════════════════════════════════════════════════════

fn transform_typed_function(
    func: TypedFunction,
    source: Dialect,
    target: Dialect,
) -> TypedFunction {
    func.transform_children(&|e| transform_expr(e, source, target))
}

// ═══════════════════════════════════════════════════════════════════════════
// Function name mapping
// ═══════════════════════════════════════════════════════════════════════════

/// Map a canonical function name to the target spelling for the plugin path.
/// COALESCE-family spellings need AST flags/metadata and are handled by the
/// parser/generator, not this flat string helper.
pub(crate) fn map_function_name(name: &str, target: Dialect) -> String {
    rules::rename_function(target, &name.to_ascii_uppercase())
        .map(str::to_string)
        .unwrap_or_else(|| name.to_string())
}

// ═══════════════════════════════════════════════════════════════════════════
// Data-type mapping
// ═══════════════════════════════════════════════════════════════════════════

/// Map data types between dialects.
/// Source-independent Unknown-type-name normalizations applied for the
/// sqlite target (used both at top level and when recursing into the
/// element types of ARRAY/MAP/STRUCT/TUPLE). Returns the rewritten name.
pub(crate) fn map_data_type(dt: DataType, target: Dialect) -> DataType {
    if let DataType::Unknown(name) = &dt
        && let Some(mapped) = rules::map_type(target, &name.to_ascii_uppercase())
    {
        return DataType::Unknown(mapped.to_string());
    }
    if let DataType::Unknown(name) = &dt
        && matches!(target, Dialect::Sqlite)
    {
        let upper = name.to_ascii_uppercase();
        if let Some((base, rest)) = upper.split_once('(') {
            // MEDIUMINT(n) is kept by SQLGlot (not folded to INTEGER(n)).
            if matches!(base, "INT" | "INTEGER" | "BIGINT" | "SMALLINT" | "TINYINT") {
                return DataType::Unknown(format!("INTEGER({rest}"));
            }
            // VARCHAR(MAX) / CHAR(MAX) → TEXT(MAX)
            if matches!(base, "VARCHAR" | "CHAR") {
                return DataType::Unknown(format!("TEXT({rest}"));
            }
            // DOUBLE(p, s) → REAL(p, s)
            if base == "DOUBLE" {
                return DataType::Unknown(format!("REAL({rest}"));
            }
        }
        if let Some(inner) = strip_nullable_wrapper(name, &upper) {
            return map_data_type(DataType::Unknown(inner), target);
        }
    }
    match (dt, target) {
        (
            DataType::Collate {
                data_type,
                collation,
            },
            target,
        ) => DataType::Collate {
            data_type: Box::new(map_data_type(*data_type, target)),
            collation,
        },
        (DataType::Array(inner), target) if matches!(target, Dialect::Sqlite) => {
            DataType::Array(inner.map(|inner| Box::new(map_data_type(*inner, target))))
        }
        (DataType::Map { key, value }, target) if matches!(target, Dialect::Sqlite) => {
            DataType::Map {
                key: Box::new(map_data_type(*key, target)),
                value: Box::new(map_data_type(*value, target)),
            }
        }
        (DataType::Struct(fields), target) if matches!(target, Dialect::Sqlite) => {
            DataType::Struct(
                fields
                    .into_iter()
                    .map(|(name, dt)| (name, map_data_type(dt, target)))
                    .collect(),
            )
        }
        (DataType::Tuple(types), target) if matches!(target, Dialect::Sqlite) => DataType::Tuple(
            types
                .into_iter()
                .map(|dt| map_data_type(dt, target))
                .collect(),
        ),
        // ── SQLite type affinity ─────────────────────────────────────────
        (
            DataType::TinyInt | DataType::SmallInt | DataType::Int | DataType::BigInt,
            Dialect::Sqlite,
        ) => DataType::Unknown("INTEGER".to_string()),
        (DataType::Boolean, Dialect::Sqlite) => DataType::Unknown("INTEGER".to_string()),
        (DataType::Float | DataType::Double, Dialect::Sqlite) => DataType::Real,
        (
            DataType::Decimal { precision, scale } | DataType::Numeric { precision, scale },
            Dialect::Sqlite,
        ) => sqlite_type_with_params("REAL", precision, scale),
        (DataType::Varchar(len) | DataType::Char(len), Dialect::Sqlite) => match len {
            Some(n) => DataType::Unknown(format!("TEXT({n})")),
            None => DataType::Text,
        },
        (DataType::String, Dialect::Sqlite) => DataType::Text,
        (DataType::Binary(len) | DataType::Varbinary(len), Dialect::Sqlite) => match len {
            Some(n) => DataType::Unknown(format!("BLOB({n})")),
            None => DataType::Blob,
        },
        (DataType::Unknown(name), Dialect::Sqlite)
            if name.eq_ignore_ascii_case("INT UNSIGNED")
                || name.eq_ignore_ascii_case("INT SIGNED") =>
        {
            if name.to_ascii_uppercase().contains("UNSIGNED") {
                DataType::Unknown("UINT".to_string())
            } else {
                DataType::Unknown("INTEGER".to_string())
            }
        }
        (DataType::Unknown(name), Dialect::Sqlite)
            if name.eq_ignore_ascii_case("BIGINT UNSIGNED")
                || name.eq_ignore_ascii_case("BIGINT SIGNED") =>
        {
            if name.to_ascii_uppercase().contains("UNSIGNED") {
                DataType::Unknown("UBIGINT".to_string())
            } else {
                DataType::Unknown("BIGINT".to_string())
            }
        }
        (DataType::Unknown(name), Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("STRING FORMAT ") =>
        {
            DataType::Unknown(format!("TEXT{}", &name["STRING".len()..]))
        }
        (DataType::Unknown(name), Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("STRING(") =>
        {
            DataType::Unknown(format!("TEXT{}", &name["STRING".len()..]))
        }
        (DataType::Unknown(name), Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("FLOAT(") =>
        {
            DataType::Unknown(format!("REAL{}", &name["FLOAT".len()..]))
        }
        (DataType::Unknown(name), Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("JSON(") =>
        {
            DataType::Unknown(rewrite_raw_type_params_for_sqlite(&name))
        }
        (DataType::Unknown(name), Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("JSONB(") =>
        {
            DataType::Unknown(rewrite_raw_type_params_for_sqlite(&name))
        }
        (DataType::Unknown(name), Dialect::Sqlite)
            if matches!(
                name.to_ascii_uppercase().as_str(),
                "LONGVARCHAR" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT"
            ) =>
        {
            DataType::Text
        }
        (DataType::Unknown(name), Dialect::Sqlite)
            if matches!(
                name.to_ascii_uppercase().as_str(),
                "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB"
            ) =>
        {
            DataType::Blob
        }

        // ── TEXT / STRING ────────────────────────────────────────────────
        // TEXT → STRING for BigQuery, Hive, Spark, Databricks
        (DataType::Text, t) if matches!(t, Dialect::BigQuery) || is_hive_family(t) => {
            DataType::String
        }
        (DataType::Varchar(Some(n)) | DataType::Char(Some(n)), Dialect::BigQuery) => {
            DataType::Unknown(format!("STRING({n})"))
        }
        (DataType::Varchar(None) | DataType::Char(None), Dialect::BigQuery) => DataType::String,
        // STRING → TEXT for Postgres family, MySQL family, SQLite
        (DataType::String, t)
            if is_postgres_family(t) || is_mysql_family(t) || matches!(t, Dialect::Sqlite) =>
        {
            DataType::Text
        }
        (DataType::Varchar(_) | DataType::Char(_) | DataType::String, Dialect::DuckDb) => {
            DataType::Text
        }

        // ── INT → BIGINT (BigQuery) ─────────────────────────────────────
        (DataType::Int, Dialect::BigQuery) => DataType::BigInt,

        // ── FLOAT → DOUBLE (BigQuery) ───────────────────────────────────
        (DataType::Float | DataType::Real, Dialect::BigQuery) => DataType::Double,

        // ── DECIMAL → NUMERIC (BigQuery) ────────────────────────────────
        (DataType::Decimal { precision, scale }, Dialect::BigQuery) => {
            DataType::Numeric { precision, scale }
        }

        // ── BOOLEAN → BOOL (BigQuery generator spelling) ────────────────
        // The AST keeps the canonical Boolean variant; gen_data_type owns
        // the target spelling.

        // ── BYTEA ↔ BLOB ────────────────────────────────────────────────
        (DataType::Bytea | DataType::Blob, Dialect::BigQuery) => DataType::Bytes,
        (DataType::Bytea, t)
            if is_mysql_family(t)
                || matches!(t, Dialect::Sqlite | Dialect::Oracle)
                || is_hive_family(t) =>
        {
            DataType::Blob
        }
        (DataType::Blob, t) if is_postgres_family(t) => DataType::Bytea,

        // ── BOOLEAN → BOOL ──────────────────────────────────────────────
        (DataType::Boolean, Dialect::Mysql) => DataType::Boolean,

        // Everything else is unchanged
        (dt, _) => dt,
    }
}

fn strip_nullable_wrapper(name: &str, upper: &str) -> Option<String> {
    let prefix = "NULLABLE(";
    if !upper.starts_with(prefix) || !upper.ends_with(')') {
        return None;
    }
    // Use the original-case name (preserve inner type spelling).
    let inner_start = prefix.len();
    let inner_end = name.len() - 1;
    if inner_start >= inner_end {
        return None;
    }
    let inner = &name[inner_start..inner_end];
    let mut canonical = inner.trim().to_string();
    // Normalize common ClickHouse aliases that the postgres parser
    // leaves as-is (DateTime → DATETIME, etc.).
    let inner_upper = canonical.to_ascii_uppercase();
    if inner_upper == "DATETIME" {
        canonical = "DATETIME".to_string();
    }
    Some(canonical)
}

/// Postgres pseudo-types and range/multirange types that SQLGlot recognizes
/// as type keywords (and therefore uppercases) when the source is postgres.
pub(crate) fn is_postgres_pseudo_type(upper: &str) -> bool {
    matches!(
        upper,
        "CSTRING"
            | "OID"
            | "NAME"
            | "REGCLASS"
            | "REGCOLLATION"
            | "REGCONFIG"
            | "REGDICTIONARY"
            | "REGNAMESPACE"
            | "REGOPER"
            | "REGOPERATOR"
            | "REGPROC"
            | "REGPROCEDURE"
            | "REGROLE"
            | "REGTYPE"
            | "DATERANGE"
            | "DATEMULTIRANGE"
            | "TSRANGE"
            | "TSMULTIRANGE"
            | "TSTZRANGE"
            | "TSTZMULTIRANGE"
            | "NUMRANGE"
            | "NUMMULTIRANGE"
            | "INT4RANGE"
            | "INT4MULTIRANGE"
            | "INT8RANGE"
            | "INT8MULTIRANGE"
    )
}

fn sqlite_type_with_params(name: &str, precision: Option<u32>, scale: Option<u32>) -> DataType {
    match (precision, scale) {
        (Some(p), Some(s)) => DataType::Unknown(format!("{name}({p}, {s})")),
        (Some(p), None) => DataType::Unknown(format!("{name}({p})")),
        (None, _) => DataType::Unknown(name.to_string()),
    }
}

fn rewrite_raw_type_params_for_sqlite(name: &str) -> String {
    let mut output = String::with_capacity(name.len());
    let mut chars = name.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut end = start + ch.len_utf8();
            while let Some((idx, next)) = chars.peek().copied() {
                if next.is_ascii_alphanumeric() || next == '_' {
                    chars.next();
                    end = idx + next.len_utf8();
                } else {
                    break;
                }
            }
            let word = &name[start..end];
            if word.eq_ignore_ascii_case("STRING") {
                output.push_str("TEXT");
            } else {
                output.push_str(word);
            }
        } else {
            output.push(ch);
        }
    }
    output
}

// ═══════════════════════════════════════════════════════════════════════════
// LIMIT / TOP / FETCH FIRST transform
// ═══════════════════════════════════════════════════════════════════════════

/// Transform LIMIT / TOP / FETCH FIRST between dialects.
///
/// - T-SQL family:  `LIMIT n` → `TOP n` (OFFSET + FETCH handled separately)
/// - Oracle:        `LIMIT n` → `FETCH FIRST n ROWS ONLY`
/// - All others:    `TOP n` / `FETCH FIRST n` → `LIMIT n`
fn transform_limit(sel: &mut SelectStatement, target: Dialect) {
    if is_tsql_family(target) {
        // Move LIMIT → TOP for T-SQL (only when there's no OFFSET)
        if let Some(limit) = sel.limit.take() {
            if sel.offset.is_none() {
                sel.top = Some(Box::new(limit));
            } else {
                // T-SQL with OFFSET uses OFFSET n ROWS FETCH NEXT m ROWS ONLY
                sel.fetch_first = Some(limit);
            }
        }
        // Also move fetch_first → top when no offset
        if sel.offset.is_none()
            && let Some(fetch) = sel.fetch_first.take()
        {
            sel.top = Some(Box::new(fetch));
        }
    } else if matches!(target, Dialect::Oracle) {
        // Oracle prefers FETCH FIRST n ROWS ONLY (SQL:2008 syntax)
        if let Some(limit) = sel.limit.take() {
            sel.fetch_first = Some(limit);
        }
        if let Some(top) = sel.top.take() {
            sel.fetch_first = Some(*top);
        }
    } else {
        // All other dialects: normalize to LIMIT
        if let Some(top) = sel.top.take()
            && sel.limit.is_none()
        {
            sel.limit = Some(*top);
        }
        if let Some(fetch) = sel.fetch_first.take()
            && sel.limit.is_none()
        {
            sel.limit = Some(fetch);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Quoted-identifier transform
// ═══════════════════════════════════════════════════════════════════════════

/// Convert any quoted identifiers in expressions to the target dialect's
/// quoting convention.
fn transform_quotes(expr: Expr, target: Dialect) -> Expr {
    match expr {
        Expr::Column {
            table,
            name,
            quote_style,
            table_quote_style,
        } => {
            let new_qs = if quote_style.is_quoted() {
                QuoteStyle::for_dialect(target)
            } else {
                QuoteStyle::None
            };
            let new_tqs = if table_quote_style.is_quoted() {
                QuoteStyle::for_dialect(target)
            } else {
                QuoteStyle::None
            };
            Expr::Column {
                table,
                name,
                quote_style: new_qs,
                table_quote_style: new_tqs,
            }
        }
        // Recurse into sub-expressions
        Expr::BinaryOp { left, op, right } => Expr::BinaryOp {
            left: Box::new(transform_quotes(*left, target)),
            op,
            right: Box::new(transform_quotes(*right, target)),
        },
        Expr::UnaryOp { op, expr } => Expr::UnaryOp {
            op,
            expr: Box::new(transform_quotes(*expr, target)),
        },
        Expr::Function {
            name,
            args,
            distinct,
            filter,
            over,
        } => Expr::Function {
            name,
            args: args
                .into_iter()
                .map(|a| transform_quotes(a, target))
                .collect(),
            distinct,
            filter: filter.map(|f| Box::new(transform_quotes(*f, target))),
            over,
        },
        Expr::TypedFunction { func, filter, over } => Expr::TypedFunction {
            func: func.transform_children(&|e| transform_quotes(e, target)),
            filter: filter.map(|f| Box::new(transform_quotes(*f, target))),
            over,
        },
        Expr::Nested(inner) => Expr::Nested(Box::new(transform_quotes(*inner, target))),
        Expr::Alias { expr, name } => Expr::Alias {
            expr: Box::new(transform_quotes(*expr, target)),
            name,
        },
        other => other,
    }
}

/// Transform quoting for all identifier-bearing nodes inside a SELECT.
fn transform_quotes_in_select(sel: &mut SelectStatement, target: Dialect) {
    // Columns in the select list
    for item in &mut sel.columns {
        if let SelectItem::Expr { expr, .. } = item {
            *expr = transform_quotes(expr.clone(), target);
        }
    }
    // WHERE
    if let Some(wh) = &mut sel.where_clause {
        *wh = transform_quotes(wh.clone(), target);
    }
    // GROUP BY
    for gb in &mut sel.group_by {
        *gb = transform_quotes(gb.clone(), target);
    }
    // HAVING
    if let Some(having) = &mut sel.having {
        *having = transform_quotes(having.clone(), target);
    }
    // ORDER BY
    for ob in &mut sel.order_by {
        ob.expr = transform_quotes(ob.expr.clone(), target);
    }
    // Table refs (FROM, JOINs)
    if let Some(from) = &mut sel.from {
        transform_quotes_in_table_source(&mut from.source, target);
    }
    for join in &mut sel.joins {
        transform_quotes_in_table_source(&mut join.table, target);
        if let Some(on) = &mut join.on {
            *on = transform_quotes(on.clone(), target);
        }
    }
}

fn transform_exprs_in_table_source(ts: &mut TableSource, source: Dialect, target: Dialect) {
    match ts {
        TableSource::Table(_) => {}
        TableSource::Raw { source_dialect, .. } => {
            if source_dialect.is_none() {
                *source_dialect = Some(source);
            }
        }
        TableSource::Subquery { query, .. } => {
            transform_statement(query, source, target);
        }
        TableSource::TableFunction { args, .. } => {
            for arg in args {
                transform_expr_in_place(arg, source, target);
            }
        }
        TableSource::Values { rows, .. } => {
            for row in rows {
                for v in row {
                    transform_expr_in_place(v, source, target);
                }
            }
        }
        TableSource::Lateral { source: inner } => {
            transform_exprs_in_table_source(inner, source, target);
        }
        TableSource::Pivot { source: inner, .. } | TableSource::Unpivot { source: inner, .. } => {
            transform_exprs_in_table_source(inner, source, target);
        }
        TableSource::Unnest { expr, .. } => {
            transform_expr_in_place(expr, source, target);
        }
    }
}

fn transform_quotes_in_table_source(source: &mut TableSource, target: Dialect) {
    match source {
        TableSource::Table(tref) => {
            if tref.name_quote_style.is_quoted() {
                tref.name_quote_style = QuoteStyle::for_dialect(target);
            }
        }
        TableSource::Subquery { .. } => {}
        TableSource::TableFunction { .. } => {}
        TableSource::Raw { .. } => {}
        TableSource::Values { .. } => {}
        TableSource::Lateral { source } => transform_quotes_in_table_source(source, target),
        TableSource::Pivot { source, .. } | TableSource::Unpivot { source, .. } => {
            transform_quotes_in_table_source(source, target);
        }
        TableSource::Unnest { .. } => {}
    }
}
