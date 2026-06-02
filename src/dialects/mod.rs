use serde::{Deserialize, Serialize};

use crate::ast::*;

pub mod plugin;
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
fn is_mysql_family(d: Dialect) -> bool {
    matches!(
        d,
        Dialect::Mysql | Dialect::Doris | Dialect::SingleStore | Dialect::StarRocks
    )
}

/// Dialects in the Postgres family (support ILIKE, BYTEA, SUBSTRING).
fn is_postgres_family(d: Dialect) -> bool {
    matches!(
        d,
        Dialect::Postgres | Dialect::Redshift | Dialect::Materialize | Dialect::RisingWave
    )
}

/// Dialects in the Presto family (ANSI-like, VARCHAR oriented).
fn is_presto_family(d: Dialect) -> bool {
    matches!(d, Dialect::Presto | Dialect::Trino | Dialect::Athena)
}

/// Dialects in the Hive/Spark family (use STRING type, SUBSTR).
fn is_hive_family(d: Dialect) -> bool {
    matches!(d, Dialect::Hive | Dialect::Spark | Dialect::Databricks)
}

/// Dialects in the T-SQL family.
fn is_tsql_family(d: Dialect) -> bool {
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
    if from == to && !matches!(from, Dialect::Sqlite) {
        return statement.clone();
    }
    let mut stmt = statement.clone();
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
                    *expr = transform_expr(expr.clone(), source, target);
                }
            }
            if let Some(wh) = &mut sel.where_clause {
                *wh = transform_expr(wh.clone(), source, target);
            }
            for gb in &mut sel.group_by {
                *gb = transform_expr(gb.clone(), source, target);
            }
            for expr in &mut sel.distinct_on {
                *expr = transform_expr(expr.clone(), source, target);
            }
            transform_order_by_items(&mut sel.order_by, source, target);
            for expr in &mut sel.limit_by {
                *expr = transform_expr(expr.clone(), source, target);
            }
            if let Some(having) = &mut sel.having {
                *having = transform_expr(having.clone(), source, target);
            }
            if matches!(target, Dialect::Sqlite) {
                if let Some(from) = &mut sel.from {
                    strip_sqlite_unpivot_table_source(&mut from.source);
                }
                for join in &mut sel.joins {
                    strip_sqlite_unpivot_table_source(&mut join.table);
                }
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
            if is_postgres_family(source) && matches!(target, Dialect::Sqlite) {
                if let Some(from) = &mut sel.from {
                    normalize_postgres_sqlite_raw_table_source(&mut from.source);
                }
                for join in &mut sel.joins {
                    normalize_postgres_sqlite_raw_table_source(&mut join.table);
                }
            }
            // Rewrite `UNNEST(x) AS alias WITH OFFSET [AS pos]` into the
            // postgres-style `UNNEST(x) WITH ORDINALITY AS alias` for any
            // source dialect with sqlite target.
            if matches!(target, Dialect::Sqlite) {
                if let Some(from) = &mut sel.from {
                    normalize_sqlite_unnest_with_offset_in_table_source(&mut from.source);
                }
                for join in &mut sel.joins {
                    normalize_sqlite_unnest_with_offset_in_table_source(&mut join.table);
                }
            }
            // UNNEST([…]) inside a Raw table source needs source-specific
            // rewrites for sqlite output:
            //   - sqlite source: → UNNEST("…")  (quoted-id fallback)
            //   - postgres/mysql source: → UNNEST(ARRAY(…))
            if matches!(target, Dialect::Sqlite) {
                if let Some(from) = &mut sel.from {
                    rewrite_unnest_array_literal_in_table_source(&mut from.source, source);
                }
                for join in &mut sel.joins {
                    rewrite_unnest_array_literal_in_table_source(&mut join.table, source);
                }
            }
            // Replace backticks with double-quotes inside Raw table
            // sources (e.g. `UNNEST(\`a\`.\`b\`)`) for sqlite target.
            if matches!(target, Dialect::Sqlite) {
                if let Some(from) = &mut sel.from {
                    rewrite_backticks_in_table_source(&mut from.source);
                    uppercase_function_names_in_table_source(&mut from.source);
                    normalize_typed_literals_in_table_source(&mut from.source);
                }
                for join in &mut sel.joins {
                    rewrite_backticks_in_table_source(&mut join.table);
                    uppercase_function_names_in_table_source(&mut join.table);
                    normalize_typed_literals_in_table_source(&mut join.table);
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
                        *val = transform_expr(val.clone(), source, target);
                    }
                }
            }
            if let Some(on_conflict) = &mut ins.on_conflict {
                if is_postgres_family(source) && matches!(target, Dialect::Sqlite) {
                    on_conflict.compact_target = true;
                }
                if let Some(where_expr) = &mut on_conflict.target_where {
                    *where_expr = transform_expr(where_expr.clone(), source, target);
                }
                if let ConflictAction::DoUpdate(assignments) = &mut on_conflict.action {
                    for (_, val) in assignments {
                        *val = transform_expr(val.clone(), source, target);
                    }
                }
                if let Some(where_expr) = &mut on_conflict.action_where {
                    *where_expr = transform_expr(where_expr.clone(), source, target);
                }
            }
        }
        Statement::Update(upd) => {
            for (_, val) in &mut upd.assignments {
                *val = transform_expr(val.clone(), source, target);
            }
            if let Some(wh) = &mut upd.where_clause {
                *wh = transform_expr(wh.clone(), source, target);
            }
        }
        Statement::Expression(expr) => {
            *expr = transform_expr(expr.clone(), source, target);
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
                });
            }
        }
        // DDL: map data types in CREATE TABLE column definitions
        Statement::CreateTable(ct) => {
            if matches!(target, Dialect::Sqlite) {
                move_single_column_primary_key_to_column(ct);
            }
            for col in &mut ct.columns {
                if col.name_quote_style.is_quoted() {
                    col.name_quote_style = QuoteStyle::for_dialect(target);
                }
                if matches!(target, Dialect::Sqlite)
                    && is_mysql_family(source)
                    && (!matches!(col.data_type, DataType::Int) || !col.primary_key)
                {
                    col.auto_increment = false;
                }
                if matches!(target, Dialect::Sqlite)
                    && col.primary_key
                    && col.auto_increment
                    && (col.auto_increment_from_identity
                        || (is_postgres_family(source) && col.nullable != Some(false)))
                {
                    // GENERATED ... AS IDENTITY (postgres) and bare
                    // IDENTITY (tsql) are implicit NOT NULL; drop any
                    // redundant NOT NULL on the sqlite side. For plain
                    // auto_increment columns without IDENTITY, keep
                    // any explicit NOT NULL (mysql/sqlite sources).
                    col.nullable = None;
                }
                col.data_type = map_data_type_for_source(col.data_type.clone(), source, target);
                if let Some(default) = &mut col.default {
                    *default = transform_expr(default.clone(), source, target);
                }
            }
            // Transform constraints (CHECK expressions)
            for constraint in &mut ct.constraints {
                if let TableConstraint::Check { expr, .. } = constraint {
                    *expr = transform_expr(expr.clone(), source, target);
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
                        col.data_type =
                            map_data_type_for_source(col.data_type.clone(), source, target);
                        if let Some(default) = &mut col.default {
                            *default = transform_expr(default.clone(), source, target);
                        }
                    }
                    AlterTableAction::AlterColumnType { data_type, .. } => {
                        *data_type = map_data_type_for_source(data_type.clone(), source, target);
                    }
                    AlterTableAction::ChangeColumn { new_column, .. } => {
                        new_column.data_type = map_data_type_for_source(
                            new_column.data_type.clone(),
                            source,
                            target,
                        );
                        if let Some(default) = &mut new_column.default {
                            *default = transform_expr(default.clone(), source, target);
                        }
                    }
                    _ => {}
                }
            }
        }
        Statement::CreateIndex(idx) => {
            transform_order_by_items(&mut idx.columns, source, target);
            if let Some(predicate) = &mut idx.where_clause {
                *predicate = transform_expr(predicate.clone(), source, target);
            }
        }
        Statement::CreateSequence(seq) => {
            if let Some(as_type) = &mut seq.as_type {
                *as_type = map_data_type_for_source(as_type.clone(), source, target);
            }
        }
        Statement::Raw(raw) => {
            if is_postgres_family(source)
                && matches!(target, Dialect::Sqlite)
                && let Some(sql) = normalize_postgres_create_type_enum(&raw.sql)
            {
                raw.sql = sql;
            }
            if is_postgres_family(source) && matches!(target, Dialect::Sqlite) {
                raw.sql = normalize_postgres_recursive_cte_raw(&raw.sql);
                raw.sql = normalize_postgres_copy_raw(&raw.sql);
            }
            // Python SQLGlot drops SHOW statements that the mysql parser
            // recognizes (TABLES, DATABASES, INDEX, COLUMNS, CREATE *,
            // VARIABLES, etc.) but preserves unrecognized SHOW forms
            // (USERS, AGGREGATES, ...) as raw Command passthroughs.
            if is_mysql_family(source) && matches!(target, Dialect::Sqlite) {
                let trimmed = raw.sql.trim_start();
                if trimmed.get(..4).is_some_and(|p| p.eq_ignore_ascii_case("SHOW"))
                    && mysql_show_is_recognized(trimmed)
                {
                    raw.sql = String::new();
                }
            }
        }
        _ => {}
    }
}

/// Returns true if a SHOW statement is one of the forms the mysql
/// parser in Python SQLGlot recognizes (and therefore drops when
/// transpiling to sqlite). Unrecognized SHOWs fall back to Command and
/// are passed through verbatim.
fn mysql_show_is_recognized(trimmed: &str) -> bool {
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

fn normalize_postgres_create_type_enum(sql: &str) -> Option<String> {
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

fn normalize_postgres_recursive_cte_raw(sql: &str) -> String {
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

fn normalize_postgres_copy_raw(sql: &str) -> String {
    let trimmed = sql.trim_start();
    if trimmed
        .get(..6)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("COPY ("))
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

fn normalize_postgres_sqlite_raw_table_source(source: &mut TableSource) {
    match source {
        TableSource::Raw { sql, .. } => {
            *sql = strip_postgres_values_column_aliases(sql);
        }
        TableSource::Lateral { source } => normalize_postgres_sqlite_raw_table_source(source),
        TableSource::Pivot { source, .. } | TableSource::Unpivot { source, .. } => {
            normalize_postgres_sqlite_raw_table_source(source);
        }
        _ => {}
    }
}

fn normalize_sqlite_unnest_with_offset_in_table_source(source: &mut TableSource) {
    match source {
        TableSource::Raw { sql, .. } => {
            if let Some(rewritten) = rewrite_unnest_with_offset(sql) {
                *sql = rewritten;
            }
        }
        TableSource::Lateral { source } => {
            normalize_sqlite_unnest_with_offset_in_table_source(source);
        }
        TableSource::Pivot { source, .. } | TableSource::Unpivot { source, .. } => {
            normalize_sqlite_unnest_with_offset_in_table_source(source);
        }
        _ => {}
    }
}

fn rewrite_unnest_array_literal_in_table_source(source: &mut TableSource, src_dialect: Dialect) {
    match source {
        TableSource::Raw { sql, .. } => {
            if matches!(src_dialect, Dialect::Sqlite) {
                if let Some(r) = rewrite_unnest_array_literal_sqlite(sql) {
                    *sql = r;
                }
            } else if matches!(
                src_dialect,
                Dialect::Postgres | Dialect::Mysql | Dialect::SingleStore | Dialect::Doris
            ) {
                if let Some(r) = rewrite_unnest_array_literal_to_array_call(sql) {
                    *sql = r;
                }
            }
        }
        TableSource::Lateral { source } => {
            rewrite_unnest_array_literal_in_table_source(source, src_dialect);
        }
        TableSource::Pivot { source, .. } | TableSource::Unpivot { source, .. } => {
            rewrite_unnest_array_literal_in_table_source(source, src_dialect);
        }
        _ => {}
    }
}

fn rewrite_backticks_in_table_source(source: &mut TableSource) {
    match source {
        TableSource::Raw { sql, .. } => {
            if sql.contains('`') {
                *sql = sql.replace('`', "\"");
            }
        }
        TableSource::Lateral { source } => rewrite_backticks_in_table_source(source),
        TableSource::Pivot { source, .. } | TableSource::Unpivot { source, .. } => {
            rewrite_backticks_in_table_source(source);
        }
        _ => {}
    }
}

fn uppercase_function_names_in_raw_sql(sql: &str) -> String {
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
fn normalize_typed_literals_in_raw_sql(sql: &str) -> String {
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

fn normalize_typed_literals_in_table_source(source: &mut TableSource) {
    match source {
        TableSource::Raw { sql, .. } => {
            *sql = normalize_typed_literals_in_raw_sql(sql);
        }
        TableSource::Lateral { source } => {
            normalize_typed_literals_in_table_source(source);
        }
        TableSource::Pivot { source, .. } | TableSource::Unpivot { source, .. } => {
            normalize_typed_literals_in_table_source(source);
        }
        _ => {}
    }
}

fn uppercase_function_names_in_table_source(source: &mut TableSource) {
    match source {
        TableSource::Raw { sql, .. } => {
            *sql = uppercase_function_names_in_raw_sql(sql);
        }
        TableSource::Lateral { source } => {
            uppercase_function_names_in_table_source(source);
        }
        TableSource::Pivot { source, .. } | TableSource::Unpivot { source, .. } => {
            uppercase_function_names_in_table_source(source);
        }
        _ => {}
    }
}

/// For postgres/mysql source → sqlite target, `UNNEST([items])`
/// becomes `UNNEST(ARRAY(items))` — Python SQLGlot wraps the
/// bracketed list in an ARRAY function call. Handles nested brackets
/// inside the outer `[…]` by tracking depth.
fn rewrite_unnest_array_literal_to_array_call(sql: &str) -> Option<String> {
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
fn rewrite_unnest_array_literal_sqlite(sql: &str) -> Option<String> {
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
    let close_bracket = sql[body_start + 1..].find(']').map(|o| body_start + 1 + o)?;
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
fn rewrite_unnest_with_offset(sql: &str) -> Option<String> {
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
    for i in lparen_abs..sql.len() {
        match bytes[i] {
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

fn strip_postgres_values_column_aliases(sql: &str) -> String {
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

fn move_single_column_primary_key_to_column(ct: &mut CreateTableStatement) {
    let Some(primary_key_index) = ct.constraints.iter().position(|constraint| {
        matches!(
            constraint,
            TableConstraint::PrimaryKey { columns, .. } if columns.len() == 1
        )
    }) else {
        return;
    };

    let column_name = match &ct.constraints[primary_key_index] {
        TableConstraint::PrimaryKey { columns, .. } => columns[0].clone(),
        _ => return,
    };

    if let Some(column) = ct
        .columns
        .iter_mut()
        .find(|column| column.name.eq_ignore_ascii_case(&column_name))
    {
        column.primary_key = true;
        column.primary_key_from_table_constraint = true;
        ct.constraints.remove(primary_key_index);
    }
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
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("LIKE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && matches!(new_args.len(), 2 | 3)
            {
                return Expr::Like {
                    expr: Box::new(new_args[1].clone()),
                    pattern: Box::new(new_args[0].clone()),
                    negated: false,
                    escape: new_args.get(2).cloned().map(Box::new),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("GLOB")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() >= 2
            {
                return Expr::BinaryOp {
                    left: Box::new(new_args[1].clone()),
                    op: BinaryOperator::Glob,
                    right: Box::new(new_args[0].clone()),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("STRFTIME")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Function {
                    name,
                    args: vec![
                        new_args[0].clone(),
                        Expr::Column {
                            table: None,
                            name: "CURRENT_TIMESTAMP".to_string(),
                            quote_style: QuoteStyle::None,
                            table_quote_style: QuoteStyle::None,
                        },
                    ],
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("BTRIM")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && matches!(new_args.len(), 1 | 2)
            {
                return Expr::Function {
                    name: "TRIM".to_string(),
                    args: new_args,
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && !matches!(
                    source,
                    Dialect::Mysql | Dialect::Postgres | Dialect::Sqlite
                )
                && name.eq_ignore_ascii_case("STARTS_WITH")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Like {
                    expr: Box::new(new_args[0].clone()),
                    pattern: Box::new(Expr::BinaryOp {
                        left: Box::new(new_args[1].clone()),
                        op: BinaryOperator::Concat,
                        right: Box::new(Expr::StringLiteral("%".to_string())),
                    }),
                    negated: false,
                    escape: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "JSONB_BUILD_OBJECT" | "JSON_BUILD_OBJECT"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                return Expr::Function {
                    name: "JSON_OBJECT".to_string(),
                    args: new_args,
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "JSONB_BUILD_ARRAY" | "JSON_BUILD_ARRAY"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                return Expr::Function {
                    name: "JSON_ARRAY".to_string(),
                    args: new_args,
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "JSONB_ARRAY_LENGTH" | "JSON_ARRAY_LENGTH"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "JSON_ARRAY_LENGTH".to_string(),
                    args: new_args,
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "JSONB_TYPEOF" | "JSON_TYPEOF"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return sqlite_postgres_json_typeof(new_args[0].clone());
            }
            if matches!(target, Dialect::Sqlite)
                && !matches!(
                    source,
                    Dialect::Mysql | Dialect::Postgres | Dialect::Sqlite
                )
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "JSONB_EXTRACT" | "JSONB_EXTRACT_SCALAR"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "JSON_EXTRACT".to_string(),
                    args: vec![
                        new_args[0].clone(),
                        postgres_json_brace_path_to_sqlite(new_args[1].clone()),
                    ],
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "INT64" | "INTEGER" | "INT"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Cast {
                    expr: Box::new(new_args[0].clone()),
                    data_type: DataType::Unknown("INTEGER".to_string()),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("XOR")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::BinaryOp {
                    left: Box::new(new_args[0].clone()),
                    op: BinaryOperator::Xor,
                    right: Box::new(new_args[1].clone()),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("__SAFE_CAST_DATE_FORMAT")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "STR_TO_DATE".to_string(),
                    args: vec![
                        new_args[0].clone(),
                        transform_safe_cast_date_format(new_args[1].clone(), source, target),
                    ],
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("__SAFE_CAST_TIME_FORMAT")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "STR_TO_TIME".to_string(),
                    args: vec![
                        new_args[0].clone(),
                        transform_safe_cast_date_format(new_args[1].clone(), source, target),
                    ],
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("TRUNC")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() > 1
            {
                return Expr::Function {
                    name,
                    args: vec![new_args[0].clone()],
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("POSITION")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "INSTR".to_string(),
                    args: vec![new_args[1].clone(), new_args[0].clone()],
                    distinct,
                    filter,
                    over,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("CHARINDEX")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "INSTR".to_string(),
                    args: vec![new_args[1].clone(), new_args[0].clone()],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // CHARINDEX(needle, haystack, position) and
            // INSTR(haystack, needle, position[, occurrence]) lower to a
            // SUBSTRING-and-offset IIF expression for SQLite (which has no
            // 3-arg INSTR). Drop any occurrence arg per SQLGlot.
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("CHARINDEX")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 3
            {
                return sqlite_instr_with_position(
                    new_args[1].clone(),
                    new_args[0].clone(),
                    new_args[2].clone(),
                );
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("INSTR")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && (new_args.len() == 3 || new_args.len() == 4)
            {
                return sqlite_instr_with_position(
                    new_args[0].clone(),
                    new_args[1].clone(),
                    new_args[2].clone(),
                );
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(name.to_ascii_uppercase().as_str(), "MAX_BY" | "MIN_BY")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && matches!(new_args.len(), 2 | 3)
            {
                return Expr::Function {
                    name: if name.eq_ignore_ascii_case("MAX_BY") {
                        "ARG_MAX".to_string()
                    } else {
                        "ARG_MIN".to_string()
                    },
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("TO_NUMBER")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && (new_args.len() == 1 || new_args.len() == 2 || new_args.len() == 3)
            {
                return Expr::Cast {
                    expr: Box::new(new_args[0].clone()),
                    data_type: DataType::Unknown("REAL".to_string()),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("SAFE_DIVIDE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                let numerator = new_args[0].clone();
                let denominator = new_args[1].clone();
                let needs_paren = |e: &Expr| !matches!(e, Expr::Number(_) | Expr::Column { .. });
                let wrap_paren = |e: Expr| {
                    if needs_paren(&e) {
                        Expr::Nested(Box::new(e))
                    } else {
                        e
                    }
                };
                let denom_used = wrap_paren(denominator.clone());
                return Expr::If {
                    condition: Box::new(Expr::BinaryOp {
                        left: Box::new(denom_used.clone()),
                        op: BinaryOperator::Neq,
                        right: Box::new(Expr::Number("0".to_string())),
                    }),
                    true_val: Box::new(Expr::BinaryOp {
                        left: Box::new(Expr::Cast {
                            expr: Box::new(wrap_paren(numerator)),
                            data_type: DataType::Unknown("REAL".to_string()),
                        }),
                        op: BinaryOperator::Divide,
                        right: Box::new(denom_used),
                    }),
                    false_val: Some(Box::new(Expr::Null)),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "BOOLAND_AGG" | "BOOLOR_AGG"
                )
                && !distinct
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: if name.eq_ignore_ascii_case("BOOLAND_AGG") {
                        "MIN".to_string()
                    } else {
                        "MAX".to_string()
                    },
                    args: new_args,
                    distinct: false,
                    filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                    over: over.map(|spec| transform_window_spec(spec, source, target)),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(name.to_ascii_uppercase().as_str(), "BOOLAND" | "BOOLOR")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                let op = if name.eq_ignore_ascii_case("BOOLAND") {
                    BinaryOperator::And
                } else {
                    BinaryOperator::Or
                };
                return Expr::Nested(Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Nested(Box::new(new_args[0].clone()))),
                    op,
                    right: Box::new(Expr::Nested(Box::new(new_args[1].clone()))),
                }));
            }
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && matches!(name.to_ascii_uppercase().as_str(), "SHA256" | "SHA512")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                let bits = if name.eq_ignore_ascii_case("SHA256") {
                    "256"
                } else {
                    "512"
                };
                return Expr::TypedFunction {
                    func: TypedFunction::Sha2 {
                        expr: Box::new(new_args[0].clone()),
                        bit_length: Box::new(Expr::Number(bits.to_string())),
                    },
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("DATEFROMPARTS")
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                return Expr::Function {
                    name: "DATE_FROM_PARTS".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("LOCATE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && (new_args.len() == 2 || new_args.len() == 3)
            {
                if new_args.len() == 2 {
                    return Expr::Function {
                        name: "INSTR".to_string(),
                        args: vec![new_args[1].clone(), new_args[0].clone()],
                        distinct,
                        filter,
                        over,
                    };
                }
                let needle = new_args[0].clone();
                let haystack = new_args[1].clone();
                let position = new_args[2].clone();
                return sqlite_instr_with_position(haystack, needle, position);
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("STR_POSITION")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && (new_args.len() == 2 || new_args.len() == 3)
            {
                if new_args.len() == 2 {
                    return Expr::Function {
                        name: "INSTR".to_string(),
                        args: new_args,
                        distinct,
                        filter,
                        over,
                    };
                }
                return sqlite_instr_with_position(
                    new_args[0].clone(),
                    new_args[1].clone(),
                    new_args[2].clone(),
                );
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("NVL2")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && (new_args.len() == 2 || new_args.len() == 3)
            {
                return Expr::Case {
                    operand: None,
                    when_clauses: vec![(
                        Expr::UnaryOp {
                            op: UnaryOperator::Not,
                            expr: Box::new(Expr::IsNull {
                                expr: Box::new(new_args[0].clone()),
                                negated: false,
                            }),
                        },
                        new_args[1].clone(),
                    )],
                    else_clause: (new_args.len() == 3).then(|| Box::new(new_args[2].clone())),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("DECODE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() >= 3
            {
                let expr = new_args[0].clone();
                let comparisons = &new_args[1..];
                let has_default = comparisons.len() % 2 == 1;
                let comparison_count = if has_default {
                    comparisons.len() - 1
                } else {
                    comparisons.len()
                };
                let mut when_clauses = Vec::new();
                for pair in comparisons[..comparison_count].chunks(2) {
                    let condition = if matches!(pair[0], Expr::Null) {
                        Expr::IsNull {
                            expr: Box::new(expr.clone()),
                            negated: false,
                        }
                    } else if sqlite_decode_uses_plain_equality(&pair[0]) {
                        Expr::BinaryOp {
                            left: Box::new(expr.clone()),
                            op: BinaryOperator::Eq,
                            right: Box::new(pair[0].clone()),
                        }
                    } else {
                        let search = sqlite_decode_search_expr(pair[0].clone());
                        Expr::BinaryOp {
                            left: Box::new(Expr::BinaryOp {
                                left: Box::new(expr.clone()),
                                op: BinaryOperator::Eq,
                                right: Box::new(search.clone()),
                            }),
                            op: BinaryOperator::Or,
                            right: Box::new(Expr::Nested(Box::new(Expr::BinaryOp {
                                left: Box::new(Expr::IsNull {
                                    expr: Box::new(expr.clone()),
                                    negated: false,
                                }),
                                op: BinaryOperator::And,
                                right: Box::new(Expr::IsNull {
                                    expr: Box::new(search),
                                    negated: false,
                                }),
                            }))),
                        }
                    };
                    when_clauses.push((condition, pair[1].clone()));
                }
                return Expr::Case {
                    operand: None,
                    when_clauses,
                    else_clause: has_default
                        .then(|| Box::new(comparisons[comparisons.len() - 1].clone())),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("CONCAT")
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                return concat_expr(new_args);
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "SCHEMA" | "CURRENT_SCHEMA"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.is_empty()
            {
                return Expr::StringLiteral("main".to_string());
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(name.to_ascii_uppercase().as_str(), "LOG2" | "LOG10")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "LOG".to_string(),
                    args: vec![
                        Expr::Number(if name.eq_ignore_ascii_case("LOG2") {
                            "2".to_string()
                        } else {
                            "10".to_string()
                        }),
                        new_args[0].clone(),
                    ],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "JSON_AGG" | "JSON_OBJECT_AGG"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                return Expr::Function {
                    name: if name.eq_ignore_ascii_case("JSON_AGG") {
                        "JSON_GROUP_ARRAY".to_string()
                    } else {
                        "JSON_GROUP_OBJECT".to_string()
                    },
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_mysql_family(source)
                && name.eq_ignore_ascii_case("CURDATE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.is_empty()
            {
                return Expr::Column {
                    table: None,
                    name: "CURRENT_DATE".to_string(),
                    quote_style: QuoteStyle::None,
                    table_quote_style: QuoteStyle::None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_mysql_family(source)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "UTC_TIME" | "UTC_TIMESTAMP"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                return Expr::Column {
                    table: None,
                    name: if name.eq_ignore_ascii_case("UTC_TIME") {
                        "CURRENT_TIME".to_string()
                    } else {
                        "CURRENT_TIMESTAMP".to_string()
                    },
                    quote_style: QuoteStyle::None,
                    table_quote_style: QuoteStyle::None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && (is_mysql_family(source) || is_postgres_family(source))
                && matches!(name.to_ascii_uppercase().as_str(), "MAKETIME" | "MAKE_TIME")
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                return Expr::Function {
                    name: "TIME_FROM_PARTS".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("TIME_STR_TO_TIME")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && !new_args.is_empty()
            {
                return new_args[0].clone();
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("DATE_STR_TO_DATE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                // SQLGlot lowers DATE_STR_TO_DATE(x) to the bare expression
                // for SQLite (the value is already a date string).
                return new_args[0].clone();
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("DATE_TRUNC")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                // For postgres source, rename to TIMESTAMP_TRUNC and
                // swap the arg order (expr, UNIT) with the unit as a
                // bare uppercase keyword. For other sources, keep
                // DATE_TRUNC and uppercase the unit string.
                if is_postgres_family(source) {
                    let unit_upper = match &new_args[0] {
                        Expr::StringLiteral(s) => s.to_ascii_uppercase(),
                        Expr::Column { name: col, table: None, .. } => col.to_ascii_uppercase(),
                        _ => {
                            return Expr::Function {
                                name: "DATE_TRUNC".to_string(),
                                args: new_args,
                                distinct: false,
                                filter: None,
                                over: None,
                            };
                        }
                    };
                    return Expr::Function {
                        name: "TIMESTAMP_TRUNC".to_string(),
                        args: vec![
                            new_args[1].clone(),
                            Expr::Column {
                                table: None,
                                name: unit_upper,
                                quote_style: QuoteStyle::None,
                                table_quote_style: QuoteStyle::None,
                            },
                        ],
                        distinct: false,
                        filter: None,
                        over: None,
                    };
                }
                // SQLGlot uppercase-and-string-quotes the first arg of
                // DATE_TRUNC when it isn't already a string literal.
                let first = match &new_args[0] {
                    Expr::Column {
                        name: col, table, ..
                    } if table.is_none() => Expr::StringLiteral(col.to_ascii_uppercase()),
                    Expr::StringLiteral(s) => Expr::StringLiteral(s.to_ascii_uppercase()),
                    other => other.clone(),
                };
                return Expr::Function {
                    name: "DATE_TRUNC".to_string(),
                    args: vec![first, new_args[1].clone()],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("DATE_FROM_UNIX_DATE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                // SQLGlot lowers DATE_FROM_UNIX_DATE(n) to
                // DATE(DATE('1970-01-01'), '<n> DAY').
                let payload = match &new_args[0] {
                    Expr::Number(n) => format!("{n} DAY"),
                    other => format!("{other:?} DAY"),
                };
                return Expr::Function {
                    name: "DATE".to_string(),
                    args: vec![
                        Expr::Function {
                            name: "DATE".to_string(),
                            args: vec![Expr::StringLiteral("1970-01-01".to_string())],
                            distinct: false,
                            filter: None,
                            over: None,
                        },
                        Expr::StringLiteral(payload),
                    ],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("TS_OR_DS_TO_DATE_STR")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                // SQLGlot lowers TS_OR_DS_TO_DATE_STR(x) to
                // SUBSTRING(CAST(x AS TEXT), 1, 10) for SQLite.
                return Expr::TypedFunction {
                    func: TypedFunction::Substring {
                        expr: Box::new(Expr::Cast {
                            expr: Box::new(new_args[0].clone()),
                            data_type: DataType::Text,
                        }),
                        start: Box::new(Expr::Number("1".to_string())),
                        length: Some(Box::new(Expr::Number("10".to_string()))),
                    },
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && (name.eq_ignore_ascii_case("DATE_TO_DATE_STR")
                    || name.eq_ignore_ascii_case("TIME_TO_TIME_STR")
                    || name.eq_ignore_ascii_case("DATE_TO_TIME_STR"))
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Cast {
                    expr: Box::new(new_args[0].clone()),
                    data_type: DataType::Text,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("LEVENSHTEIN")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "EDITDIST3".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("MEDIAN")
                && !distinct
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "PERCENTILE_CONT".to_string(),
                    args: vec![new_args[0].clone(), Expr::Number("0.5".to_string())],
                    distinct: false,
                    filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                    over: over.map(|spec| transform_window_spec(spec, source, target)),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("COUNT_IF")
                && !distinct
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "SUM".to_string(),
                    args: vec![Expr::If {
                        condition: Box::new(new_args[0].clone()),
                        true_val: Box::new(Expr::Number("1".to_string())),
                        false_val: Some(Box::new(Expr::Number("0".to_string()))),
                    }],
                    distinct: false,
                    filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                    over: over.map(|spec| transform_window_spec(spec, source, target)),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && (name.eq_ignore_ascii_case("GENERATE_UUID")
                    || name.eq_ignore_ascii_case("UUID_STRING"))
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                return Expr::Function {
                    name: "UUID".to_string(),
                    args: vec![],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("ENDSWITH")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "ENDS_WITH".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // SQLite-targeted DATE_ADD(a, n, c) where the third arg is NOT a
            // recognized DateTimeField is lowered to DATE(a, '<n> <c>'). The
            // generic-Function path is the parser's escape hatch (see
            // try_typed_function) when the 3rd arg isn't a unit keyword.
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("DATE_ADD")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 3
            {
                let mut payload = render_for_date_add_payload(&new_args[1]);
                payload.push(' ');
                payload.push_str(&render_for_date_add_payload(&new_args[2]));
                return Expr::Function {
                    name: "DATE".to_string(),
                    args: vec![new_args[0].clone(), Expr::StringLiteral(payload)],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // CONVERT_TZ (mysql) rewrites for SQLite to match SQLGlot:
            //   CONVERT_TZ(x, from_tz, to_tz) →
            //       CAST(x AS TIMESTAMPNTZ) AT TIME ZONE from_tz AT TIME ZONE to_tz
            if is_mysql_family(source)
                && matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("CONVERT_TZ")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 3
            {
                let cast = Expr::Cast {
                    expr: Box::new(new_args[0].clone()),
                    data_type: DataType::Unknown("TIMESTAMPNTZ".to_string()),
                };
                return Expr::BinaryOp {
                    left: Box::new(Expr::BinaryOp {
                        left: Box::new(cast),
                        op: BinaryOperator::AtTimeZone,
                        right: Box::new(new_args[1].clone()),
                    }),
                    op: BinaryOperator::AtTimeZone,
                    right: Box::new(new_args[2].clone()),
                };
            }
            // CONVERT_TIMEZONE rewrites for SQLite to match SQLGlot:
            //   - 2-arg: CONVERT_TIMEZONE(zone, x) → x AT TIME ZONE zone
            //   - 3-arg: CONVERT_TIMEZONE(src, tgt, x) →
            //       CAST(x AS TIMESTAMPNTZ) AT TIME ZONE src AT TIME ZONE tgt
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("CONVERT_TIMEZONE")
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                if new_args.len() == 2 {
                    return Expr::BinaryOp {
                        left: Box::new(new_args[1].clone()),
                        op: BinaryOperator::AtTimeZone,
                        right: Box::new(new_args[0].clone()),
                    };
                }
                if new_args.len() == 3 {
                    let cast = match &new_args[2] {
                        Expr::StringLiteral(_) => Expr::Cast {
                            expr: Box::new(new_args[2].clone()),
                            data_type: DataType::Unknown("TIMESTAMPNTZ".to_string()),
                        },
                        other => other.clone(),
                    };
                    return Expr::BinaryOp {
                        left: Box::new(Expr::BinaryOp {
                            left: Box::new(cast),
                            op: BinaryOperator::AtTimeZone,
                            right: Box::new(new_args[0].clone()),
                        }),
                        op: BinaryOperator::AtTimeZone,
                        right: Box::new(new_args[1].clone()),
                    };
                }
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("VAR_POP")
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "VARIANCE_POP".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                    over: over.map(|spec| transform_window_spec(spec, source, target)),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("SPACE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "REPEAT".to_string(),
                    args: vec![Expr::StringLiteral(" ".to_string()), new_args[0].clone()],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // TIME_SLICE / TS_OR_DS_ADD take a string-literal unit ('DAY',
            // 'HOUR', etc.) that SQLGlot unquotes into a bare keyword
            // when targeting SQLite.
            if matches!(target, Dialect::Sqlite)
                && (name.eq_ignore_ascii_case("TIME_SLICE")
                    || name.eq_ignore_ascii_case("TS_OR_DS_ADD"))
                && !distinct
                && filter.is_none()
                && over.is_none()
                && (new_args.len() == 3 || new_args.len() == 4)
            {
                let mut rewritten = new_args.clone();
                if let Some(Expr::StringLiteral(unit)) = rewritten.get(2)
                    && is_recognized_interval_unit(unit)
                {
                    rewritten[2] = Expr::Column {
                        table: None,
                        name: unit.to_ascii_uppercase(),
                        quote_style: QuoteStyle::None,
                        table_quote_style: QuoteStyle::None,
                    };
                }
                return Expr::Function {
                    name,
                    args: rewritten,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("LAST_DAY_OF_MONTH")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "LAST_DAY".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if name.eq_ignore_ascii_case("TO_CHAR")
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                // Single-arg TO_CHAR(x) → CAST(x AS TEXT) for sqlite
                // target, all sources. Other targets keep TO_CHAR.
                if new_args.len() == 1 && matches!(target, Dialect::Sqlite) {
                    return Expr::Cast {
                        expr: Box::new(new_args[0].clone()),
                        data_type: DataType::Unknown("TEXT".to_string()),
                    };
                }
                // Two-arg TO_CHAR(x, fmt):
                //   - postgres source: convert to TimeToStr (generator
                //     picks the right name per target — STRFTIME for
                //     sqlite, DATE_FORMAT for mysql, TO_CHAR for pg, …).
                //   - mysql/sqlite source → sqlite target: Python drops
                //     the format and emits CAST(x AS TEXT).
                if new_args.len() == 2 {
                    if is_postgres_family(source) {
                        return Expr::TypedFunction {
                            func: TypedFunction::TimeToStr {
                                expr: Box::new(new_args[0].clone()),
                                format: Box::new(transform_format_expr(
                                    new_args[1].clone(),
                                    source,
                                    target,
                                )),
                            },
                            filter: None,
                            over: None,
                        };
                    }
                    if matches!(target, Dialect::Sqlite) {
                        return Expr::Cast {
                            expr: Box::new(new_args[0].clone()),
                            data_type: DataType::Unknown("TEXT".to_string()),
                        };
                    }
                }
            }
            // Mysql FORMAT(value, fmt[, locale]) is the NUMBER_TO_STR
            // function in Python SQLGlot's IR; rewrite the name for
            // sqlite (and other) targets.
            if is_mysql_family(source)
                && matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("FORMAT")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() >= 2
            {
                return Expr::Function {
                    name: "NUMBER_TO_STR".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
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
            // Postgres source UNNEST(c) in SELECT position → EXPLODE(c)
            // for sqlite target.
            if is_postgres_family(source)
                && matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("UNNEST")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "EXPLODE".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // COUNTIF(x) → SUM(IIF(x, 1, 0)) for sqlite target, any source.
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("COUNTIF")
                && !distinct
                && over.is_none()
                && new_args.len() == 1
            {
                let iif = Expr::Function {
                    name: "IIF".to_string(),
                    args: vec![
                        new_args[0].clone(),
                        Expr::Number("1".to_string()),
                        Expr::Number("0".to_string()),
                    ],
                    distinct: false,
                    filter: None,
                    over: None,
                };
                return Expr::TypedFunction {
                    func: TypedFunction::Sum {
                        expr: Box::new(iif),
                        distinct: false,
                    },
                    filter,
                    over: None,
                };
            }
            // TO_ARRAY(literal-array) → ARRAY(...) unwrap. For
            // mysql/postgres sources, Python unwraps array literals to
            // bare ARRAY(...). The non-literal form is left untouched
            // here since the IIF-wrapped fallback Python emits is
            // dialect-specific and rarely matched by sqlgrok's pipeline.
            if (is_mysql_family(source) || is_postgres_family(source))
                && matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("TO_ARRAY")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                if let Expr::Function { name: inner_name, args: inner_args, .. } = &new_args[0] {
                    if inner_name.eq_ignore_ascii_case("ARRAY") {
                        return Expr::Function {
                            name: "ARRAY".to_string(),
                            args: inner_args.clone(),
                            distinct: false,
                            filter: None,
                            over: None,
                        };
                    }
                }
            }
            // UNIX_SECONDS(x) → TIMESTAMPDIFF(x, CAST('1970-01-01 ...'
            //                                  AS TIMESTAMPTZ), SECONDS)
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("UNIX_SECONDS")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                let epoch = Expr::Cast {
                    expr: Box::new(Expr::StringLiteral(
                        "1970-01-01 00:00:00+00".to_string(),
                    )),
                    data_type: DataType::Unknown("TIMESTAMPTZ".to_string()),
                };
                let seconds = Expr::Column {
                    table: None,
                    name: "SECONDS".to_string(),
                    quote_style: QuoteStyle::None,
                    table_quote_style: QuoteStyle::None,
                };
                return Expr::Function {
                    name: "TIMESTAMPDIFF".to_string(),
                    args: vec![new_args[0].clone(), epoch, seconds],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // TIMESTAMP_DIFF(a, b, unit) → TIMESTAMPDIFF(a, b, UNIT)
            // and TIMESTAMP_SUB / TIMESTAMP_ADD keep their names but
            // uppercase the trailing unit arg. Python normalizes the
            // underscore-name form to the compact one.
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("TIMESTAMP_DIFF")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 3
            {
                let mut args = new_args.clone();
                if let Expr::Column { table: None, name: ref n, .. } = args[2] {
                    let upper = n.to_ascii_uppercase();
                    args[2] = Expr::Column {
                        table: None,
                        name: upper,
                        quote_style: QuoteStyle::None,
                        table_quote_style: QuoteStyle::None,
                    };
                }
                return Expr::Function {
                    name: "TIMESTAMPDIFF".to_string(),
                    args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // IS_ASCII(x) → (NOT x GLOB CAST(x'2a5b5e012d7f5d2a' AS TEXT))
            // The hex blob is the GLOB pattern `*[^\x01-\x7f]*` (any
            // non-ASCII char anywhere). Python SQLGlot uses this form
            // for sqlite output.
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("IS_ASCII")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                let glob_pattern = Expr::Cast {
                    expr: Box::new(Expr::HexString(
                        "2a5b5e012d7f5d2a".to_string(),
                    )),
                    data_type: DataType::Unknown("TEXT".to_string()),
                };
                let glob_op = Expr::BinaryOp {
                    left: Box::new(new_args[0].clone()),
                    op: BinaryOperator::Glob,
                    right: Box::new(glob_pattern),
                };
                let not_glob = Expr::UnaryOp {
                    op: UnaryOperator::Not,
                    expr: Box::new(glob_op),
                };
                return Expr::Tuple(vec![not_glob]);
            }
            // STRING(x) → CAST(x AS TEXT) for sqlite target, any source.
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("STRING")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Cast {
                    expr: Box::new(new_args[0].clone()),
                    data_type: DataType::Unknown("TEXT".to_string()),
                };
            }
            // ISNAN(x)/ISINF(x) → IS_NAN(x)/IS_INF(x) for sqlite target.
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "ISNAN" | "ISINF"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                let new_name = if name.eq_ignore_ascii_case("ISNAN") {
                    "IS_NAN"
                } else {
                    "IS_INF"
                };
                return Expr::Function {
                    name: new_name.to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // Mysql / Postgres VERSION() → SQLITE_VERSION() for sqlite.
            if (is_mysql_family(source) || is_postgres_family(source))
                && matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("VERSION")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.is_empty()
            {
                return Expr::Function {
                    name: "SQLITE_VERSION".to_string(),
                    args: vec![],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // INSERT(s, pos, len, repl) → STUFF(...) for sqlite target
            // (Python SQLGlot's IR name for the 4-arg INSERT function).
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("INSERT")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 4
            {
                return Expr::Function {
                    name: "STUFF".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("TRUNCATE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "TRUNC".to_string(),
                    args: vec![new_args[0].clone()],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("CURRENT_TIMESTAMP")
                && !distinct
                && filter.is_none()
                && over.is_none()
            {
                // SQLGlot lowers CURRENT_TIMESTAMP() / CURRENT_TIMESTAMP(n)
                // to the bare CURRENT_TIMESTAMP column form for SQLite.
                return Expr::Column {
                    table: None,
                    name: "CURRENT_TIMESTAMP".to_string(),
                    quote_style: QuoteStyle::None,
                    table_quote_style: QuoteStyle::None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("CURRENT_VERSION")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.is_empty()
            {
                return Expr::Function {
                    name: "SQLITE_VERSION".to_string(),
                    args: vec![],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "DAYOFMONTH" | "DAYOFYEAR" | "DAYOFWEEK" | "WEEKOFYEAR"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                let new_name = match name.to_ascii_uppercase().as_str() {
                    "DAYOFMONTH" => "DAY_OF_MONTH",
                    "DAYOFYEAR" => "DAY_OF_YEAR",
                    "DAYOFWEEK" => "DAY_OF_WEEK",
                    "WEEKOFYEAR" => "WEEK_OF_YEAR",
                    _ => unreachable!(),
                };
                let arg = if is_mysql_family(source) {
                    let inner = new_args[0].clone();
                    // Avoid wrapping in DATE() if the arg is already a
                    // DATE() call (e.g., from CAST(x AS DATE) lowering).
                    let already_date = matches!(
                        &inner,
                        Expr::Function { name, .. } if name.eq_ignore_ascii_case("DATE")
                    );
                    if already_date {
                        inner
                    } else {
                        Expr::Function {
                            name: "DATE".to_string(),
                            args: vec![inner],
                            distinct: false,
                            filter: None,
                            over: None,
                        }
                    }
                } else {
                    new_args[0].clone()
                };
                return Expr::Function {
                    name: new_name.to_string(),
                    args: vec![arg],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("WEEK")
                && is_mysql_family(source)
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "WEEK".to_string(),
                    args: vec![Expr::Function {
                        name: "DATE".to_string(),
                        args: vec![new_args[0].clone()],
                        distinct: false,
                        filter: None,
                        over: None,
                    }],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // mysql STR_TO_DATE(x, fmt) needs format conversion and (when
            // the format contains time markers) renaming to STR_TO_TIME for
            // SQLite output. Other source dialects preserve STR_TO_DATE.
            if matches!(target, Dialect::Sqlite)
                && is_mysql_family(source)
                && name.eq_ignore_ascii_case("STR_TO_DATE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                let transformed_format = transform_format_expr(new_args[1].clone(), source, target);
                return Expr::Function {
                    name: mysql_sqlite_str_to_time_name(&transformed_format).to_string(),
                    args: vec![new_args[0].clone(), transformed_format],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            // DATE_FORMAT in MySQL/Hive/Spark/Databricks is the native format
            // function and needs dialect-specific lowering (STRFTIME for
            // SQLite, TO_CHAR for Postgres, FORMAT_TIMESTAMP for BigQuery,
            // etc.). For Postgres/SQLite sources, DATE_FORMAT stays as a
            // plain function call so identity round-trips match SQLGlot
            // (those sources don't have DATE_FORMAT as a native function).
            if (is_mysql_family(source) || is_hive_family(source))
                && name.eq_ignore_ascii_case("DATE_FORMAT")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::TypedFunction {
                    func: TypedFunction::TimeToStr {
                        expr: Box::new(new_args[0].clone()),
                        format: Box::new(transform_format_expr(
                            new_args[1].clone(),
                            source,
                            target,
                        )),
                    },
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && ((is_mysql_family(source) && name.eq_ignore_ascii_case("FROM_UNIXTIME"))
                    || (is_postgres_family(source) && name.eq_ignore_ascii_case("TO_TIMESTAMP")))
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return Expr::Function {
                    name: "UNIX_TO_TIME".to_string(),
                    args: new_args,
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_mysql_family(source)
                && name.eq_ignore_ascii_case("FROM_UNIXTIME")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "UNIX_TO_TIME".to_string(),
                    args: vec![
                        new_args[0].clone(),
                        transform_format_expr(new_args[1].clone(), source, target),
                    ],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && name.eq_ignore_ascii_case("TO_TIMESTAMP")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::TypedFunction {
                    func: TypedFunction::StrToTime {
                        expr: Box::new(new_args[0].clone()),
                        format: Box::new(transform_format_expr(
                            new_args[1].clone(),
                            source,
                            target,
                        )),
                    },
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_mysql_family(source)
                && name.eq_ignore_ascii_case("TIMESTAMPDIFF")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 3
            {
                return Expr::Function {
                    name: "TIMESTAMPDIFF".to_string(),
                    args: vec![
                        new_args[2].clone(),
                        new_args[1].clone(),
                        timestampdiff_unit_arg(&new_args[0]),
                    ],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && name.eq_ignore_ascii_case("TO_DATE")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return Expr::Function {
                    name: "STR_TO_DATE".to_string(),
                    args: vec![
                        new_args[0].clone(),
                        transform_format_expr(new_args[1].clone(), source, target),
                    ],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && name.eq_ignore_ascii_case("DIV")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 2
            {
                return sqlite_real_cast(Expr::Cast {
                    expr: Box::new(Expr::BinaryOp {
                        left: Box::new(sqlite_real_cast(new_args[0].clone())),
                        op: BinaryOperator::Divide,
                        right: Box::new(new_args[1].clone()),
                    }),
                    data_type: DataType::Unknown("INTEGER".to_string()),
                });
            }
            // JSON_EXTRACT_PATH_TEXT is normalized to `x ->> $.path` for any
            // source dialect targeting SQLite (SQLGlot does this universally).
            // JSON_EXTRACT_PATH only collapses to the arrow form when the
            // source is Postgres-family; other sources keep the function call.
            // SQLGlot only joins ALL path args when the source is Postgres-
            // family; other sources use only the first path arg (the
            // intermediate keys are dropped because non-Postgres parsers
            // can't make sense of multi-arg JSON_EXTRACT_PATH_TEXT).
            if matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("JSON_EXTRACT_PATH_TEXT")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() >= 2
            {
                let path = if is_postgres_family(source) {
                    postgres_json_extract_path_arg(&new_args[1..])
                } else {
                    sqlite_json_path_for_first_arg(&new_args[1])
                };
                return Expr::JsonAccess {
                    expr: Box::new(new_args[0].clone()),
                    path: Box::new(path),
                    as_text: true,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && name.eq_ignore_ascii_case("JSON_EXTRACT_PATH")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() >= 2
            {
                return Expr::JsonAccess {
                    expr: Box::new(new_args[0].clone()),
                    path: Box::new(postgres_json_extract_path_arg(&new_args[1..])),
                    as_text: false,
                };
            }

            let new_name = map_function_name_for_source(&name, source, target);
            Expr::Function {
                name: new_name,
                args: new_args,
                distinct,
                filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                over: over.map(|spec| transform_window_spec(spec, source, target)),
            }
        }
        // Recurse into typed function child expressions, with special handling
        // for date/time formatting functions that need format string conversion
        Expr::TypedFunction { func, filter, over } => {
            if matches!(func, TypedFunction::CurrentTimestamp)
                && is_postgres_family(source)
                && matches!(target, Dialect::Sqlite)
                && filter.is_none()
                && over.is_none()
            {
                return Expr::Column {
                    table: None,
                    name: "CURRENT_TIMESTAMP".to_string(),
                    quote_style: QuoteStyle::None,
                    table_quote_style: QuoteStyle::None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_mysql_family(source)
                && let TypedFunction::StrToTime { expr, format } = func
            {
                let transformed_format = transform_format_expr(*format, source, target);
                return Expr::Function {
                    name: mysql_sqlite_str_to_time_name(&transformed_format).to_string(),
                    args: vec![transform_expr(*expr, source, target), transformed_format],
                    distinct: false,
                    filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                    over: over.map(|spec| transform_window_spec(spec, source, target)),
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && filter.is_none()
                && over.is_none()
                && let TypedFunction::GenerateSeries { start, stop, step } = func
            {
                return Expr::Function {
                    name: "UNNEST".to_string(),
                    args: vec![Expr::TypedFunction {
                        func: TypedFunction::GenerateSeries {
                            start: Box::new(transform_expr(*start, source, target)),
                            stop: Box::new(transform_expr(*stop, source, target)),
                            step: step.map(|step| {
                                Box::new(transform_generate_series_step(*step, source, target))
                            }),
                        },
                        filter: None,
                        over: None,
                    }],
                    distinct: false,
                    filter: None,
                    over: None,
                };
            }
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && let TypedFunction::StrToTime { expr, format } = func
            {
                return Expr::Function {
                    name: "STR_TO_TIME".to_string(),
                    args: vec![
                        transform_expr(*expr, source, target),
                        transform_format_expr(*format, source, target),
                    ],
                    distinct: false,
                    filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                    over: over.map(|spec| transform_window_spec(spec, source, target)),
                };
            }
            if matches!(target, Dialect::Sqlite) {
                match func {
                    TypedFunction::Greatest { exprs } => {
                        if exprs.len() == 1 {
                            let mut exprs = exprs;
                            return transform_expr(exprs.remove(0), source, target);
                        }
                        return Expr::Function {
                            name: "MAX".to_string(),
                            args: exprs
                                .into_iter()
                                .map(|e| transform_expr(e, source, target))
                                .collect(),
                            distinct: false,
                            filter,
                            over,
                        };
                    }
                    TypedFunction::Least { exprs } => {
                        if exprs.len() == 1 {
                            let mut exprs = exprs;
                            return transform_expr(exprs.remove(0), source, target);
                        }
                        return Expr::Function {
                            name: "MIN".to_string(),
                            args: exprs
                                .into_iter()
                                .map(|e| transform_expr(e, source, target))
                                .collect(),
                            distinct: false,
                            filter,
                            over,
                        };
                    }
                    TypedFunction::Log { expr, base: None } if is_mysql_family(source) => {
                        return Expr::TypedFunction {
                            func: TypedFunction::Ln {
                                expr: Box::new(transform_expr(*expr, source, target)),
                            },
                            filter,
                            over,
                        };
                    }
                    TypedFunction::ParseJSON { expr } => {
                        return transform_expr(*expr, source, target);
                    }
                    TypedFunction::Substring {
                        expr,
                        start,
                        length,
                    } if is_postgres_family(source) => {
                        let mut args = vec![
                            transform_expr(*expr, source, target),
                            transform_expr(*start, source, target),
                        ];
                        if let Some(length) = length {
                            args.push(transform_expr(*length, source, target));
                        }
                        return Expr::Function {
                            name: "SUBSTRING".to_string(),
                            args,
                            distinct: false,
                            filter,
                            over,
                        };
                    }
                    TypedFunction::Left { expr, n }
                        if !matches!(target, Dialect::Sqlite) =>
                    {
                        return Expr::Function {
                            name: "SUBSTR".to_string(),
                            args: vec![
                                transform_expr(*expr, source, target),
                                Expr::Number("1".to_string()),
                                transform_expr(*n, source, target),
                            ],
                            distinct: false,
                            filter,
                            over,
                        };
                    }
                    TypedFunction::Right { expr, n }
                        if !matches!(target, Dialect::Sqlite) =>
                    {
                        return Expr::Function {
                            name: "SUBSTR".to_string(),
                            args: vec![
                                transform_expr(*expr, source, target),
                                Expr::UnaryOp {
                                    op: UnaryOperator::Minus,
                                    expr: Box::new(transform_expr(*n, source, target)),
                                },
                            ],
                            distinct: false,
                            filter,
                            over,
                        };
                    }
                    // UPPER(HEX(x)) → HEX(x) for SQLite — HEX output is
                    // already uppercase, so SQLGlot drops the outer UPPER.
                    TypedFunction::Upper { expr }
                        if matches!(target, Dialect::Sqlite)
                            && filter.is_none()
                            && over.is_none()
                            && matches!(
                                expr.as_ref(),
                                Expr::TypedFunction {
                                    func: TypedFunction::Hex { .. },
                                    ..
                                }
                            ) =>
                    {
                        return transform_expr(*expr, source, target);
                    }
                    TypedFunction::Mod { left, right } if filter.is_none() && over.is_none() => {
                        let left = transform_expr(*left, source, target);
                        let right = transform_expr(*right, source, target);
                        let needs_paren = |e: &Expr| matches!(e, Expr::BinaryOp { .. });
                        let wrap = |e: Expr| {
                            if needs_paren(&e) {
                                Expr::Nested(Box::new(e))
                            } else {
                                e
                            }
                        };
                        return Expr::BinaryOp {
                            left: Box::new(wrap(left)),
                            op: BinaryOperator::Modulo,
                            right: Box::new(wrap(right)),
                        };
                    }
                    TypedFunction::Year { expr } if is_mysql_family(source) => {
                        return Expr::TypedFunction {
                            func: TypedFunction::Year {
                                expr: Box::new(Expr::Function {
                                    name: "DATE".to_string(),
                                    args: vec![transform_expr(*expr, source, target)],
                                    distinct: false,
                                    filter: None,
                                    over: None,
                                }),
                            },
                            filter,
                            over,
                        };
                    }
                    TypedFunction::Month { expr } if is_mysql_family(source) => {
                        return Expr::TypedFunction {
                            func: TypedFunction::Month {
                                expr: Box::new(Expr::Function {
                                    name: "DATE".to_string(),
                                    args: vec![transform_expr(*expr, source, target)],
                                    distinct: false,
                                    filter: None,
                                    over: None,
                                }),
                            },
                            filter,
                            over,
                        };
                    }
                    TypedFunction::Day { expr } if is_mysql_family(source) => {
                        return Expr::TypedFunction {
                            func: TypedFunction::Day {
                                expr: Box::new(Expr::Function {
                                    name: "DATE".to_string(),
                                    args: vec![transform_expr(*expr, source, target)],
                                    distinct: false,
                                    filter: None,
                                    over: None,
                                }),
                            },
                            filter,
                            over,
                        };
                    }
                    _ => {}
                }
            }

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
            let data_type = map_data_type_for_source(data_type, source, target);
            let is_date = matches!(&data_type, DataType::Date)
                || matches!(
                    &data_type,
                    DataType::Unknown(s) if s.eq_ignore_ascii_case("DATE")
                );
            if matches!(target, Dialect::Sqlite) && is_date {
                Expr::Function {
                    name: "DATE".to_string(),
                    args: vec![expr],
                    distinct: false,
                    filter: None,
                    over: None,
                }
            } else {
                Expr::Cast {
                    expr: Box::new(expr),
                    data_type,
                }
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
            if matches!(target, Dialect::Sqlite)
                && is_mysql_family(source)
                && op == BinaryOperator::Divide
            {
                Expr::BinaryOp {
                    left: Box::new(sqlite_real_cast(left)),
                    op,
                    right: Box::new(right),
                }
            } else if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && op == BinaryOperator::Power
            {
                Expr::Function {
                    name: "POWER".to_string(),
                    args: vec![left, right],
                    distinct: false,
                    filter: None,
                    over: None,
                }
            } else if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && op == BinaryOperator::ArrayContainedBy
            {
                Expr::BinaryOp {
                    left: Box::new(right),
                    op: BinaryOperator::ArrayContains,
                    right: Box::new(left),
                }
            } else {
                Expr::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }
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
        Expr::Interval { value, unit, unit_text } => {
            transform_interval(*value, unit, unit_text, source, target)
        }
        Expr::ArrayLiteral(items) => {
            let items: Vec<Expr> = items
                .into_iter()
                .map(|item| transform_expr(item, source, target))
                .collect();
            if matches!(target, Dialect::Sqlite) && matches!(source, Dialect::Sqlite) {
                // Python SQLGlot represents [1, 2, 3] for sqlite as a
                // double-quoted identifier with the rendered body.
                let body = items
                    .iter()
                    .map(crate::generator::Generator::expr_to_sql)
                    .collect::<Vec<_>>()
                    .join(", ");
                Expr::Column {
                    table: None,
                    name: body,
                    quote_style: crate::ast::QuoteStyle::DoubleQuote,
                    table_quote_style: crate::ast::QuoteStyle::None,
                }
            } else if (is_postgres_family(source) || is_mysql_family(source))
                && matches!(target, Dialect::Sqlite)
            {
                Expr::Function {
                    name: "ARRAY".to_string(),
                    args: items,
                    distinct: false,
                    filter: None,
                    over: None,
                }
            } else {
                Expr::ArrayLiteral(items)
            }
        }
        // Postgres source `col[N]` → sqlite `col[N-1]` (postgres uses
        // 1-based array indexing; Python normalizes to 0-based for
        // sqlite output).
        Expr::ArrayIndex { expr, index }
            if is_postgres_family(source) && matches!(target, Dialect::Sqlite) =>
        {
            let new_index = match *index {
                Expr::Number(n) => {
                    if let Ok(parsed) = n.parse::<i64>() {
                        Expr::Number((parsed - 1).to_string())
                    } else {
                        Expr::Number(n)
                    }
                }
                other => Expr::BinaryOp {
                    left: Box::new(transform_expr(other, source, target)),
                    op: BinaryOperator::Minus,
                    right: Box::new(Expr::Number("1".to_string())),
                },
            };
            Expr::ArrayIndex {
                expr: Box::new(transform_expr(*expr, source, target)),
                index: Box::new(new_index),
            }
        }
        Expr::JsonAccess {
            expr,
            path,
            as_text,
        } => Expr::JsonAccess {
            expr: Box::new(transform_expr(*expr, source, target)),
            path: Box::new(normalize_json_access_path(*path, target)),
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
        Expr::Parameter(param)
            if is_postgres_family(source)
                && matches!(target, Dialect::Sqlite)
                && param.starts_with('$') =>
        {
            Expr::Parameter(format!("@{}", &param[1..]))
        }
        // Transform quoting on column references
        Expr::Column {
            table,
            name,
            quote_style,
            table_quote_style,
        } => {
            if table.is_none()
                && is_postgres_family(source)
                && matches!(target, Dialect::Sqlite)
                && name.eq_ignore_ascii_case("current_date")
            {
                return Expr::Column {
                    table: None,
                    name: "CURRENT_DATE".to_string(),
                    quote_style: QuoteStyle::None,
                    table_quote_style: QuoteStyle::None,
                };
            }
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
        Expr::Exists { mut subquery, negated } => {
            transform_statement(&mut subquery, source, target);
            Expr::Exists { subquery, negated }
        }
        Expr::Subquery(mut stmt) => {
            transform_statement(&mut stmt, source, target);
            Expr::Subquery(stmt)
        }
        Expr::InSubquery { expr, mut subquery, negated } => {
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

fn concat_expr(args: Vec<Expr>) -> Expr {
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        return Expr::StringLiteral(String::new());
    };
    args.fold(first, |left, right| Expr::BinaryOp {
        left: Box::new(left),
        op: BinaryOperator::Concat,
        right: Box::new(right),
    })
}

fn sqlite_instr_with_position(haystack: Expr, needle: Expr, position: Expr) -> Expr {
    let substring = Expr::Function {
        name: "SUBSTRING".to_string(),
        args: vec![haystack, position.clone()],
        distinct: false,
        filter: None,
        over: None,
    };
    let instr = Expr::Function {
        name: "INSTR".to_string(),
        args: vec![substring, needle],
        distinct: false,
        filter: None,
        over: None,
    };
    Expr::If {
        condition: Box::new(Expr::BinaryOp {
            left: Box::new(instr.clone()),
            op: BinaryOperator::Eq,
            right: Box::new(Expr::Number("0".to_string())),
        }),
        true_val: Box::new(Expr::Number("0".to_string())),
        false_val: Some(Box::new(Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(instr),
                op: BinaryOperator::Plus,
                right: Box::new(position),
            }),
            op: BinaryOperator::Minus,
            right: Box::new(Expr::Number("1".to_string())),
        })),
    }
}

fn sqlite_decode_uses_plain_equality(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Number(_)
            | Expr::StringLiteral(_)
            | Expr::EscapedStringLiteral(_)
            | Expr::HexString(_)
    )
}

fn sqlite_decode_search_expr(expr: Expr) -> Expr {
    match expr {
        Expr::BinaryOp { .. } => Expr::Nested(Box::new(expr)),
        other => other,
    }
}

fn sqlite_real_cast(expr: Expr) -> Expr {
    Expr::Cast {
        expr: Box::new(expr),
        data_type: DataType::Real,
    }
}

fn transform_window_spec(mut spec: WindowSpec, source: Dialect, target: Dialect) -> WindowSpec {
    for expr in &mut spec.partition_by {
        *expr = transform_expr(expr.clone(), source, target);
    }
    transform_order_by_items(&mut spec.order_by, source, target);
    spec
}

fn transform_order_by_items(items: &mut [OrderByItem], source: Dialect, target: Dialect) {
    for item in items {
        item.expr = transform_expr(item.expr.clone(), source, target);
        if is_postgres_family(source)
            && matches!(target, Dialect::Sqlite)
            && item.nulls_first.is_none()
        {
            item.nulls_first = Some(!item.ascending);
        }
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
            _ if depth == 0 => {
                if upper_bytes[i..].starts_with(b" LIMIT ")
                    || upper_bytes[i..].starts_with(b" OFFSET ")
                {
                    end = i;
                    break;
                }
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
            let leading_spaces: String = item.chars()
                .take_while(|c| c.is_whitespace())
                .collect();
            let core = trimmed.trim_start();
            // Detect parenthesized expr like `(a + b) DESC` — strip
            // any trailing direction keyword to decide.
            let upper_core = core.to_ascii_uppercase();
            let is_desc = upper_core.ends_with(" DESC");
            let already_has_nulls = upper_core.ends_with(" NULLS FIRST")
                || upper_core.ends_with(" NULLS LAST");
            if already_has_nulls || core.is_empty() {
                return format!("{leading_spaces}{core}");
            }
            let suffix = if is_desc { " NULLS FIRST" } else { " NULLS LAST" };
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
    source: Dialect,
    target: Dialect,
) -> Option<SelectStatement> {
    if !is_postgres_family(source)
        || !matches!(target, Dialect::Sqlite)
        || sel.distinct_on.is_empty()
    {
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
    let transformed_value = transform_expr(value, source, target);
    if is_postgres_family(source)
        && matches!(target, Dialect::Sqlite)
        && unit.is_none()
        && let Expr::StringLiteral(literal) = &transformed_value
        && let Some((amount, parsed_unit)) = split_postgres_interval_literal(literal)
    {
        return Expr::Interval {
            value: Box::new(Expr::StringLiteral(amount.to_string())),
            unit: Some(parsed_unit),
            unit_text: None,
        };
    }

    Expr::Interval {
        value: Box::new(transformed_value),
        unit,
        unit_text,
    }
}

fn split_postgres_interval_literal(literal: &str) -> Option<(&str, DateTimeField)> {
    let mut parts = literal.split_whitespace();
    let amount = parts.next()?;
    let unit = parts.next()?;
    if parts.next().is_some() {
        return None;
    }

    parse_interval_unit(unit).map(|field| (amount, field))
}

fn parse_interval_unit(unit: &str) -> Option<DateTimeField> {
    match unit.trim_end_matches('s').to_ascii_uppercase().as_str() {
        "YEAR" => Some(DateTimeField::Year),
        "QUARTER" => Some(DateTimeField::Quarter),
        "MONTH" => Some(DateTimeField::Month),
        "WEEK" => Some(DateTimeField::Week),
        "DAY" => Some(DateTimeField::Day),
        "HOUR" => Some(DateTimeField::Hour),
        "MINUTE" => Some(DateTimeField::Minute),
        "SECOND" => Some(DateTimeField::Second),
        "MILLISECOND" => Some(DateTimeField::Millisecond),
        "MICROSECOND" => Some(DateTimeField::Microsecond),
        _ => None,
    }
}

fn normalize_json_access_path(path: Expr, target: Dialect) -> Expr {
    if !matches!(target, Dialect::Sqlite) {
        return path;
    }

    match path {
        Expr::StringLiteral(key) => Expr::StringLiteral(sqlite_json_key_path(&key)),
        Expr::Number(index) => Expr::StringLiteral(format!("$[{index}]")),
        other => other,
    }
}

fn sqlite_json_key_path(key: &str) -> String {
    if key.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
        && key
            .chars()
            .next()
            .is_some_and(|c| c == '_' || c.is_ascii_alphabetic())
    {
        format!("$.{key}")
    } else {
        format!("$.\"{}\"", key.replace('"', "\\\""))
    }
}

/// For non-Postgres source JSON_EXTRACT_PATH_TEXT(x, ...), use only the
/// first path arg and prefix with `$.` if not already prefixed.
/// SQLGlot's non-Postgres parsers can't natively make sense of multi-arg
/// JSON_EXTRACT_PATH_TEXT and only consume the first segment.
fn sqlite_json_path_for_first_arg(arg: &Expr) -> Expr {
    match arg {
        Expr::StringLiteral(s) => {
            if s.starts_with("$") {
                Expr::StringLiteral(s.clone())
            } else {
                Expr::StringLiteral(format!("$.{s}"))
            }
        }
        other => other.clone(),
    }
}

fn postgres_json_extract_path_arg(path_args: &[Expr]) -> Expr {
    if path_args
        .iter()
        .all(|arg| matches!(arg, Expr::StringLiteral(_)))
    {
        let mut path = "$".to_string();
        for arg in path_args {
            let Expr::StringLiteral(segment) = arg else {
                unreachable!("all path args are string literals");
            };
            path.push_str(&sqlite_json_path_segment(segment));
        }
        Expr::StringLiteral(path)
    } else {
        path_args[0].clone()
    }
}

fn postgres_json_brace_path_to_sqlite(path: Expr) -> Expr {
    match path {
        Expr::StringLiteral(path) if path.starts_with('{') && path.ends_with('}') => {
            let mut sqlite_path = "$".to_string();
            let inner = &path[1..path.len() - 1];
            if inner.is_empty() {
                return Expr::StringLiteral(sqlite_path);
            }
            for segment in inner.split(',') {
                sqlite_path.push_str(&sqlite_json_path_segment(segment));
            }
            Expr::StringLiteral(sqlite_path)
        }
        Expr::StringLiteral(key) => Expr::StringLiteral(sqlite_json_key_path(&key)),
        Expr::Number(index) => Expr::StringLiteral(format!("$[{index}]")),
        other => other,
    }
}

fn sqlite_json_path_segment(segment: &str) -> String {
    if segment.chars().all(|c| c.is_ascii_digit()) {
        format!("[{segment}]")
    } else if segment
        .chars()
        .all(|c| c == '_' || c.is_ascii_alphanumeric())
        && segment
            .chars()
            .next()
            .is_some_and(|c| c == '_' || c.is_ascii_alphabetic())
    {
        format!(".{segment}")
    } else {
        format!(".\"{}\"", segment.replace('"', "\\\""))
    }
}

fn sqlite_postgres_json_typeof(expr: Expr) -> Expr {
    fn json_type(expr: Expr) -> Expr {
        Expr::Function {
            name: "JSON_TYPE".to_string(),
            args: vec![expr],
            distinct: false,
            filter: None,
            over: None,
        }
    }

    Expr::Case {
        operand: Some(Box::new(json_type(expr.clone()))),
        when_clauses: vec![
            (
                Expr::StringLiteral("integer".to_string()),
                Expr::StringLiteral("number".to_string()),
            ),
            (
                Expr::StringLiteral("real".to_string()),
                Expr::StringLiteral("number".to_string()),
            ),
            (
                Expr::StringLiteral("text".to_string()),
                Expr::StringLiteral("string".to_string()),
            ),
            (
                Expr::StringLiteral("true".to_string()),
                Expr::StringLiteral("boolean".to_string()),
            ),
            (
                Expr::StringLiteral("false".to_string()),
                Expr::StringLiteral("boolean".to_string()),
            ),
        ],
        else_clause: Some(Box::new(json_type(expr))),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Typed function transformation with format string conversion
// ═══════════════════════════════════════════════════════════════════════════

/// Transform a TypedFunction, including date/time format string conversion.
///
/// For TimeToStr and StrToTime functions, this converts the format string
/// from the source dialect's convention to the target dialect's convention.
fn transform_typed_function(
    func: TypedFunction,
    source: Dialect,
    target: Dialect,
) -> TypedFunction {
    match func {
        TypedFunction::TimeToStr { expr, format } => {
            let transformed_expr = Box::new(transform_expr(*expr, source, target));
            let transformed_format = transform_format_expr(*format, source, target);
            TypedFunction::TimeToStr {
                expr: transformed_expr,
                format: Box::new(transformed_format),
            }
        }
        TypedFunction::StrToTime { expr, format } => {
            let transformed_expr = Box::new(transform_expr(*expr, source, target));
            let transformed_format = transform_format_expr(*format, source, target);
            TypedFunction::StrToTime {
                expr: transformed_expr,
                format: Box::new(transformed_format),
            }
        }
        TypedFunction::DatePart { part, expr }
            if matches!(source, Dialect::Postgres) && matches!(target, Dialect::Sqlite) =>
        {
            TypedFunction::ExtractPart {
                part: Box::new(transform_expr(*part, source, target)),
                expr: Box::new(transform_expr(*expr, source, target)),
            }
        }
        TypedFunction::DateTrunc { unit, expr }
            if matches!(source, Dialect::Postgres) && matches!(target, Dialect::Sqlite) =>
        {
            TypedFunction::TimestampTrunc {
                unit,
                expr: Box::new(transform_expr(*expr, source, target)),
            }
        }
        // Mysql DATE_SUB / DATE_ADD with an INTERVAL second arg unboxes
        // to the 3-arg form (value, value_str, unit) for sqlite output,
        // matching Python SQLGlot.
        TypedFunction::DateSub {
            expr,
            interval,
            unit,
        } if is_mysql_family(source)
            && matches!(target, Dialect::Sqlite)
            && matches!(interval.as_ref(), Expr::Interval { .. }) =>
        {
            let (value, ivl_unit) = match *interval {
                Expr::Interval { value, unit: u, .. } => (*value, u),
                _ => unreachable!(),
            };
            let unit = unit.or(ivl_unit);
            // Python SQLGlot renders the numeric value as a string literal
            // in the 3-arg DATE_SUB form ("DATE_SUB(x, '1', WEEK)").
            let interval_arg = match value {
                Expr::Number(n) => Expr::StringLiteral(n),
                other => transform_expr(other, source, target),
            };
            TypedFunction::DateSub {
                expr: Box::new(transform_expr(*expr, source, target)),
                interval: Box::new(interval_arg),
                unit,
            }
        }
        TypedFunction::DateAdd {
            expr,
            interval,
            unit,
        } if is_mysql_family(source)
            && matches!(target, Dialect::Sqlite)
            && matches!(interval.as_ref(), Expr::Interval { .. }) =>
        {
            let (value, ivl_unit) = match *interval {
                Expr::Interval { value, unit: u, .. } => (*value, u),
                _ => unreachable!(),
            };
            let unit = unit.or(ivl_unit);
            // Strip string-literal quotes off the interval value so the
            // generator's DATE_ADD→DATE(x, 'N UNIT') payload doesn't end
            // up with nested quotes like 'INTERVAL '3' DAY' → ''3' DAY'.
            let value = match value {
                Expr::StringLiteral(s) => Expr::Number(s),
                other => other,
            };
            TypedFunction::DateAdd {
                expr: Box::new(transform_expr(*expr, source, target)),
                interval: Box::new(transform_expr(value, source, target)),
                unit,
            }
        }
        // For all other typed functions, just transform child expressions
        other => other.transform_children(&|e| transform_expr(e, source, target)),
    }
}

/// Transform a format string expression for the target dialect.
///
/// If the expression is a string literal, convert the format specifiers.
/// Otherwise, just recursively transform child expressions.
fn transform_format_expr(expr: Expr, source: Dialect, target: Dialect) -> Expr {
    match &expr {
        Expr::StringLiteral(s) => {
            let detected_source = source_time_format_style(s, source);
            let target_style = time::TimeFormatStyle::for_dialect(target);

            // Only convert if styles differ
            if detected_source != target_style {
                let converted = time::format_time(s, detected_source, target_style);
                Expr::StringLiteral(converted)
            } else {
                expr
            }
        }
        _ => transform_expr(expr, source, target),
    }
}

fn transform_safe_cast_date_format(expr: Expr, source: Dialect, target: Dialect) -> Expr {
    match (expr, source, target) {
        (Expr::StringLiteral(format), Dialect::Postgres, Dialect::Sqlite) => {
            Expr::StringLiteral(format_postgres_safe_cast_date_format(&format))
        }
        (expr, _, _) => transform_expr(expr, source, target),
    }
}

fn format_postgres_safe_cast_date_format(format: &str) -> String {
    format
        .replace("YYYY", "%Y")
        .replace("YY", "%y")
        .replace("MM", "%m")
        .replace("DD", "%d")
}

fn source_time_format_style(format_str: &str, source: Dialect) -> time::TimeFormatStyle {
    match source {
        Dialect::Ansi | Dialect::Prql => detect_format_style(format_str),
        _ => time::TimeFormatStyle::for_dialect(source),
    }
}

fn mysql_sqlite_str_to_time_name(format: &Expr) -> &'static str {
    match format {
        Expr::StringLiteral(format) if mysql_format_contains_time(format) => "STR_TO_TIME",
        _ => "STR_TO_DATE",
    }
}

/// Render an Expr to a string suitable for embedding inside SQLite's
/// DATE() payload (`DATE(x, '<n> <unit>')`). Strings are stripped of
/// quotes, columns are uppercased, numbers passed through.
fn render_for_date_add_payload(expr: &Expr) -> String {
    match expr {
        Expr::StringLiteral(s) => s.clone(),
        Expr::Number(n) => n.clone(),
        Expr::Column { name, .. } => name.to_ascii_uppercase(),
        other => format!("{other:?}"),
    }
}

fn is_recognized_interval_unit(unit: &str) -> bool {
    matches!(
        unit.to_ascii_uppercase().as_str(),
        "YEAR"
            | "YEARS"
            | "QUARTER"
            | "QUARTERS"
            | "MONTH"
            | "MONTHS"
            | "WEEK"
            | "WEEKS"
            | "DAY"
            | "DAYS"
            | "HOUR"
            | "HOURS"
            | "MINUTE"
            | "MINUTES"
            | "SECOND"
            | "SECONDS"
            | "MILLISECOND"
            | "MILLISECONDS"
            | "MICROSECOND"
            | "MICROSECONDS"
            | "NANOSECOND"
            | "NANOSECONDS"
    )
}

fn mysql_format_contains_time(format: &str) -> bool {
    [
        "%f", "%H", "%h", "%I", "%i", "%k", "%l", "%p", "%r", "%S", "%s", "%T",
    ]
    .iter()
    .any(|needle| format.contains(needle))
}

fn timestampdiff_unit_arg(expr: &Expr) -> Expr {
    match expr {
        Expr::Column { name, .. } => Expr::Column {
            table: None,
            name: name.to_ascii_uppercase(),
            quote_style: QuoteStyle::None,
            table_quote_style: QuoteStyle::None,
        },
        Expr::StringLiteral(unit) => Expr::Column {
            table: None,
            name: unit.to_ascii_uppercase(),
            quote_style: QuoteStyle::None,
            table_quote_style: QuoteStyle::None,
        },
        other => other.clone(),
    }
}

fn transform_generate_series_step(step: Expr, source: Dialect, target: Dialect) -> Expr {
    match step {
        Expr::StringLiteral(literal)
            if matches!(source, Dialect::Postgres) && matches!(target, Dialect::Sqlite) =>
        {
            if let Some((amount, unit)) = split_compact_interval_literal(&literal).or_else(|| {
                split_postgres_interval_literal(&literal).map(|(a, u)| (a.to_string(), u))
            }) {
                return Expr::Interval {
                    value: Box::new(Expr::StringLiteral(amount)),
                    unit: Some(unit),
                    unit_text: None,
                };
            }
            Expr::StringLiteral(literal)
        }
        other => transform_expr(other, source, target),
    }
}

fn split_compact_interval_literal(literal: &str) -> Option<(String, DateTimeField)> {
    let split_at = literal
        .char_indices()
        .find_map(|(index, ch)| ch.is_ascii_alphabetic().then_some(index))?;
    let (amount, unit) = literal.split_at(split_at);
    if amount.is_empty() || unit.is_empty() {
        return None;
    }
    parse_interval_unit(unit).map(|field| (amount.to_string(), field))
}

/// Detect the format style from a format string based on its content.
fn detect_format_style(format_str: &str) -> time::TimeFormatStyle {
    // Check for style-specific patterns
    if format_str.contains('%') {
        // strftime-style format
        if format_str.contains("%i") {
            // MySQL uses %i for minutes
            time::TimeFormatStyle::Mysql
        } else {
            // Generic strftime (SQLite, BigQuery, etc.)
            time::TimeFormatStyle::Strftime
        }
    } else if format_str.contains("YYYY") || format_str.contains("yyyy") {
        // Check for Java vs Postgres/Snowflake
        if format_str.contains("HH24") || format_str.contains("MI") || format_str.contains("SS") {
            // Postgres/Oracle style
            time::TimeFormatStyle::Postgres
        } else if format_str.contains("mm") && format_str.contains("ss") {
            // Java style (lowercase seconds and minutes)
            time::TimeFormatStyle::Java
        } else if format_str.contains("FF") {
            // Snowflake fractional seconds
            time::TimeFormatStyle::Snowflake
        } else if format_str.contains("MM") && format_str.contains("DD") {
            // Could be Postgres or Snowflake - default to Postgres
            time::TimeFormatStyle::Postgres
        } else {
            // Default to Java for ambiguous cases with lowercase patterns
            time::TimeFormatStyle::Java
        }
    } else {
        // Unknown format - default to strftime
        time::TimeFormatStyle::Strftime
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Function name mapping
// ═══════════════════════════════════════════════════════════════════════════

/// Map function names between dialects.
pub(crate) fn map_function_name(name: &str, target: Dialect) -> String {
    map_function_name_for_source(name, Dialect::Ansi, target)
}

fn map_function_name_for_source(name: &str, source: Dialect, target: Dialect) -> String {
    let upper = name.to_uppercase();
    match upper.as_str() {
        // ── NOW / CURRENT_TIMESTAMP / GETDATE ────────────────────────────
        "NOW" => {
            if is_tsql_family(target) {
                "GETDATE".to_string()
            } else if matches!(
                target,
                Dialect::Ansi
                    | Dialect::BigQuery
                    | Dialect::Snowflake
                    | Dialect::Oracle
                    | Dialect::ClickHouse
                    | Dialect::Exasol
                    | Dialect::Teradata
                    | Dialect::Druid
                    | Dialect::Dremio
                    | Dialect::Tableau
            ) || is_presto_family(target)
                || is_hive_family(target)
            {
                "CURRENT_TIMESTAMP".to_string()
            } else {
                // Postgres, MySQL, SQLite, DuckDB, Redshift, etc. – keep NOW
                name.to_string()
            }
        }
        "GETDATE" => {
            if matches!(target, Dialect::Sqlite) {
                if is_tsql_family(source) {
                    // SQLGlot converts T-SQL GETDATE() to CURRENT_TIMESTAMP
                    // when targeting SQLite, but preserves it for every
                    // other source dialect.
                    "CURRENT_TIMESTAMP".to_string()
                } else {
                    name.to_string()
                }
            } else if is_tsql_family(target) {
                name.to_string()
            } else if is_postgres_family(target)
                || matches!(target, Dialect::Mysql | Dialect::DuckDb)
            {
                "NOW".to_string()
            } else {
                "CURRENT_TIMESTAMP".to_string()
            }
        }

        // ── LEN / LENGTH ─────────────────────────────────────────────────
        "LEN" => {
            if is_tsql_family(target) || matches!(target, Dialect::BigQuery | Dialect::Snowflake) {
                name.to_string()
            } else {
                "LENGTH".to_string()
            }
        }
        "LENGTH" if is_tsql_family(target) => "LEN".to_string(),
        "ANY_VALUE" if matches!(target, Dialect::Sqlite) => "MAX".to_string(),

        // ── SUBSTR / SUBSTRING ───────────────────────────────────────────
        "SUBSTR" => {
            if is_mysql_family(target)
                || matches!(target, Dialect::Sqlite | Dialect::Oracle)
                || is_hive_family(target)
            {
                "SUBSTR".to_string()
            } else {
                "SUBSTRING".to_string()
            }
        }
        "SUBSTRING" => {
            if is_mysql_family(target)
                || matches!(target, Dialect::Sqlite | Dialect::Oracle)
                || is_hive_family(target)
            {
                "SUBSTR".to_string()
            } else {
                name.to_string()
            }
        }

        // ── IFNULL / COALESCE / ISNULL ───────────────────────────────────
        "IFNULL" => {
            if is_tsql_family(target) {
                "ISNULL".to_string()
            } else if is_mysql_family(target) {
                name.to_string()
            } else {
                "COALESCE".to_string()
            }
        }
        "ISNULL" => {
            if is_tsql_family(target) {
                name.to_string()
            } else if is_mysql_family(target) || matches!(target, Dialect::Sqlite) {
                "IFNULL".to_string()
            } else {
                "COALESCE".to_string()
            }
        }

        // ── NVL → COALESCE (Oracle to others) ───────────────────────────
        "NVL" => {
            if matches!(target, Dialect::Oracle) {
                name.to_string()
            } else {
                "COALESCE".to_string()
            }
        }

        // ── RANDOM / RAND ────────────────────────────────────────────────
        "RANDOM" => {
            if matches!(
                target,
                Dialect::Postgres | Dialect::Sqlite | Dialect::DuckDb
            ) {
                name.to_string()
            } else {
                "RAND".to_string()
            }
        }
        "RAND" => {
            if matches!(
                target,
                Dialect::Postgres | Dialect::Sqlite | Dialect::DuckDb
            ) {
                "RANDOM".to_string()
            } else {
                name.to_string()
            }
        }

        // ── STRING_AGG / GROUP_CONCAT ───────────────────────────────────
        "STRING_AGG" if matches!(target, Dialect::Sqlite) => "GROUP_CONCAT".to_string(),
        "STRPOS" if matches!(target, Dialect::Sqlite) => "INSTR".to_string(),
        "CHR" if is_postgres_family(source) && matches!(target, Dialect::Sqlite) => {
            "CHAR".to_string()
        }
        "ASCII" if is_postgres_family(source) && matches!(target, Dialect::Sqlite) => {
            "ASCII".to_string()
        }
        "SPLIT_PART" if is_postgres_family(source) && matches!(target, Dialect::Sqlite) => {
            "SPLIT_PART".to_string()
        }
        "BOOL_AND" if is_postgres_family(source) && matches!(target, Dialect::Sqlite) => {
            "MIN".to_string()
        }
        "BOOL_OR" if is_postgres_family(source) && matches!(target, Dialect::Sqlite) => {
            "MAX".to_string()
        }

        // ── BIT aggregate functions ─────────────────────────────────────
        "BIT_AND"
            if matches!(target, Dialect::Sqlite)
                && (is_mysql_family(source) || is_postgres_family(source)) =>
        {
            "BITWISE_AND_AGG".to_string()
        }
        "BIT_OR"
            if matches!(target, Dialect::Sqlite)
                && (is_mysql_family(source) || is_postgres_family(source)) =>
        {
            "BITWISE_OR_AGG".to_string()
        }
        "BIT_XOR"
            if matches!(target, Dialect::Sqlite)
                && (is_mysql_family(source) || is_postgres_family(source)) =>
        {
            "BITWISE_XOR_AGG".to_string()
        }
        "BIT_COUNT" if matches!(target, Dialect::Sqlite) && is_mysql_family(source) => {
            "BITWISE_COUNT".to_string()
        }

        // ── UUID functions ──────────────────────────────────────────────
        "GEN_RANDOM_UUID" if matches!(target, Dialect::Sqlite) => "UUID".to_string(),

        // Everything else – preserve original name
        _ => name.to_string(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Data-type mapping
// ═══════════════════════════════════════════════════════════════════════════

/// Map data types between dialects.
pub(crate) fn map_data_type(dt: DataType, target: Dialect) -> DataType {
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

        // ── TEXT / STRING ────────────────────────────────────────────────
        // TEXT → STRING for BigQuery, Hive, Spark, Databricks
        (DataType::Text, t) if matches!(t, Dialect::BigQuery) || is_hive_family(t) => {
            DataType::String
        }
        // STRING → TEXT for Postgres family, MySQL family, SQLite
        (DataType::String, t)
            if is_postgres_family(t) || is_mysql_family(t) || matches!(t, Dialect::Sqlite) =>
        {
            DataType::Text
        }

        // ── INT → BIGINT (BigQuery) ─────────────────────────────────────
        (DataType::Int, Dialect::BigQuery) => DataType::BigInt,

        // ── FLOAT → DOUBLE (BigQuery) ───────────────────────────────────
        (DataType::Float, Dialect::BigQuery) => DataType::Double,

        // ── BYTEA ↔ BLOB ────────────────────────────────────────────────
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
fn is_postgres_pseudo_type(upper: &str) -> bool {
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

fn map_data_type_for_source(dt: DataType, source: Dialect, target: Dialect) -> DataType {
    // Common type aliases that always map to the canonical sqlite-
    // target name regardless of source.
    if let DataType::Unknown(name) = &dt {
        if matches!(target, Dialect::Sqlite) {
            let upper = name.to_ascii_uppercase();
            let mapped = match upper.as_str() {
                "NUMBER" | "FLOAT4" => Some("REAL".to_string()),
                "INT4" | "INT1" | "MEDIUMINT" => Some("INTEGER".to_string()),
                "HUGEINT" => Some("INT128".to_string()),
                "UHUGEINT" => Some("UINT128".to_string()),
                _ => None,
            };
            if let Some(m) = mapped {
                return DataType::Unknown(m);
            }
        }
        // Postgres pseudo-types and range types are recognized type keywords
        // in SQLGlot, so they're uppercased on output — but only when the
        // SOURCE is postgres. Other sources treat them as user-defined type
        // names and preserve the original case.
        if is_postgres_family(source) && is_postgres_pseudo_type(&name.to_ascii_uppercase()) {
            return DataType::Unknown(name.to_ascii_uppercase());
        }
    }
    // ClickHouse-style `Nullable(T)` wrappers: strip for sqlite output
    // and recurse on the inner type (matching Python SQLGlot, which
    // models nullability separately).
    if let DataType::Unknown(name) = &dt {
        if matches!(target, Dialect::Sqlite) {
            let upper = name.to_ascii_uppercase();
            if let Some(inner) = strip_nullable_wrapper(name, &upper) {
                let inner_dt = DataType::Unknown(inner);
                return map_data_type_for_source(inner_dt, source, target);
            }
        }
    }
    match (&dt, source, target) {
        (DataType::Unknown(name), s, Dialect::Sqlite)
            if is_mysql_family(s)
                && (name.eq_ignore_ascii_case("SIGNED")
                    || name.eq_ignore_ascii_case("SIGNED INTEGER")) =>
        {
            DataType::Unknown("INTEGER".to_string())
        }
        (DataType::Unknown(name), s, Dialect::Sqlite)
            if is_mysql_family(s)
                && (name.eq_ignore_ascii_case("UNSIGNED")
                    || name.eq_ignore_ascii_case("UNSIGNED INTEGER")) =>
        {
            DataType::Unknown("UBIGINT".to_string())
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if name.eq_ignore_ascii_case("INT UNSIGNED")
                || name.eq_ignore_ascii_case("INT SIGNED") =>
        {
            if name.to_ascii_uppercase().contains("UNSIGNED") {
                DataType::Unknown("UINT".to_string())
            } else {
                DataType::Unknown("INTEGER".to_string())
            }
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if name.eq_ignore_ascii_case("BIGINT UNSIGNED")
                || name.eq_ignore_ascii_case("BIGINT SIGNED") =>
        {
            if name.to_ascii_uppercase().contains("UNSIGNED") {
                DataType::Unknown("UBIGINT".to_string())
            } else {
                DataType::Unknown("BIGINT".to_string())
            }
        }
        (
            DataType::Timestamp {
                precision,
                with_tz: true,
            },
            _,
            Dialect::Sqlite,
        ) => DataType::Unknown(match precision {
            Some(p) => format!("TIMESTAMPTZ({p})"),
            None => "TIMESTAMPTZ".to_string(),
        }),
        (
            DataType::Timestamp {
                precision,
                with_tz: false,
            },
            s,
            Dialect::Sqlite,
        ) if is_mysql_family(s) => DataType::Unknown(match precision {
            Some(p) => format!("TIMESTAMPTZ({p})"),
            None => "TIMESTAMPTZ".to_string(),
        }),
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("STRING FORMAT ") =>
        {
            DataType::Unknown(format!("TEXT{}", &name["STRING".len()..]))
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("STRING(") =>
        {
            DataType::Unknown(format!("TEXT{}", &name["STRING".len()..]))
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("FLOAT(") =>
        {
            DataType::Unknown(format!("REAL{}", &name["FLOAT".len()..]))
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("JSON(") =>
        {
            DataType::Unknown(rewrite_raw_type_params_for_sqlite(name))
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if name.to_ascii_uppercase().starts_with("JSONB(") =>
        {
            DataType::Unknown(rewrite_raw_type_params_for_sqlite(name))
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if matches!(
                name.to_ascii_uppercase().as_str(),
                "LONGVARCHAR" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT"
            ) =>
        {
            DataType::Text
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if matches!(
                name.to_ascii_uppercase().as_str(),
                "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB"
            ) =>
        {
            DataType::Blob
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if matches!(
                name.to_ascii_uppercase().as_str(),
                "INT8" | "INT16" | "INT32" | "INT64" | "INT128" | "INT256"
            ) =>
        {
            DataType::Unknown("INTEGER".to_string())
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if name.eq_ignore_ascii_case("BIGNUMERIC") =>
        {
            DataType::Unknown("BIGDECIMAL".to_string())
        }
        (DataType::Unknown(name), _, Dialect::Sqlite)
            if matches!(
                name.to_ascii_uppercase().as_str(),
                "TIMESTAMP_NTZ" | "TIMESTAMP_LTZ" | "TIMESTAMP_TZ"
            ) =>
        {
            DataType::Unknown(name.replace('_', ""))
        }
        _ => map_data_type(dt, target),
    }
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
        TableSource::Table(_) | TableSource::Raw { .. } => {}
        TableSource::Subquery { query, .. } => {
            transform_statement(query, source, target);
        }
        TableSource::TableFunction { args, .. } => {
            for arg in args {
                *arg = transform_expr(arg.clone(), source, target);
            }
        }
        TableSource::Values { rows, .. } => {
            for row in rows {
                for v in row {
                    *v = transform_expr(v.clone(), source, target);
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
            *expr = Box::new(transform_expr(*expr.clone(), source, target));
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

fn strip_sqlite_unpivot_table_source(source: &mut TableSource) {
    match source {
        TableSource::Unpivot { source: inner, .. } => {
            let mut replacement = (**inner).clone();
            strip_sqlite_unpivot_table_source(&mut replacement);
            *source = replacement;
        }
        TableSource::Lateral { source } | TableSource::Pivot { source, .. } => {
            strip_sqlite_unpivot_table_source(source);
        }
        _ => {}
    }
}
