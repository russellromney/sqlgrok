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
    if from == to {
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
            if let Some(having) = &mut sel.having {
                *having = transform_expr(having.clone(), source, target);
            }
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
                if let ConflictAction::DoUpdate(assignments) = &mut on_conflict.action {
                    for (_, val) in assignments {
                        *val = transform_expr(val.clone(), source, target);
                    }
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
        }
        // DDL: map data types in CREATE TABLE column definitions
        Statement::CreateTable(ct) => {
            if matches!(target, Dialect::Sqlite) {
                move_single_column_primary_key_to_column(ct);
            }
            for col in &mut ct.columns {
                if matches!(target, Dialect::Sqlite)
                    && is_mysql_family(source)
                    && (!matches!(col.data_type, DataType::Int)
                        || !col.primary_key
                        || col.auto_increment_before_primary_key)
                {
                    col.auto_increment = false;
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
                    _ => {}
                }
            }
        }
        Statement::CreateIndex(idx) => {
            for column in &mut idx.columns {
                column.expr = transform_expr(column.expr.clone(), source, target);
            }
            if let Some(predicate) = &mut idx.where_clause {
                *predicate = transform_expr(predicate.clone(), source, target);
            }
        }
        Statement::Raw(raw) => {
            if is_postgres_family(source)
                && matches!(target, Dialect::Sqlite)
                && let Some(sql) = normalize_postgres_create_type_enum(&raw.sql)
            {
                raw.sql = sql;
            }
        }
        _ => {}
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
                && (is_postgres_family(source) || is_mysql_family(source))
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
                && is_mysql_family(source)
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
                return Expr::If {
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
                && is_mysql_family(source)
                && name.eq_ignore_ascii_case("TIME_STR_TO_TIME")
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() == 1
            {
                return new_args[0].clone();
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
            if matches!(target, Dialect::Sqlite)
                && is_postgres_family(source)
                && matches!(
                    name.to_ascii_uppercase().as_str(),
                    "JSON_EXTRACT_PATH" | "JSON_EXTRACT_PATH_TEXT"
                )
                && !distinct
                && filter.is_none()
                && over.is_none()
                && new_args.len() >= 2
            {
                let as_text = name.eq_ignore_ascii_case("JSON_EXTRACT_PATH_TEXT");
                return Expr::JsonAccess {
                    expr: Box::new(new_args[0].clone()),
                    path: Box::new(postgres_json_extract_path_arg(&new_args[1..])),
                    as_text,
                };
            }

            let new_name = map_function_name_for_source(&name, source, target);
            Expr::Function {
                name: new_name,
                args: new_args,
                distinct,
                filter: filter.map(|f| Box::new(transform_expr(*f, source, target))),
                over,
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
            if matches!(target, Dialect::Sqlite) && matches!(data_type, DataType::Date) {
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
        Expr::Interval { value, unit } => transform_interval(*value, unit, source, target),
        Expr::ArrayLiteral(items) => {
            let items: Vec<Expr> = items
                .into_iter()
                .map(|item| transform_expr(item, source, target))
                .collect();
            if is_postgres_family(source) && matches!(target, Dialect::Sqlite) {
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
        Expr::JsonAccess {
            expr,
            path,
            as_text,
        } => Expr::JsonAccess {
            expr: Box::new(transform_expr(*expr, source, target)),
            path: Box::new(normalize_json_access_path(*path, target)),
            as_text,
        },
        Expr::Nested(inner) => Expr::Nested(Box::new(transform_expr(*inner, source, target))),
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
        };
    }

    Expr::Interval {
        value: Box::new(transformed_value),
        unit,
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
            if is_tsql_family(target) {
                name.to_string()
            } else if is_postgres_family(target)
                || matches!(target, Dialect::Mysql | Dialect::DuckDb | Dialect::Sqlite)
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
            if matches!(target, Dialect::Oracle | Dialect::Snowflake) {
                name.to_string()
            } else if is_mysql_family(target) || matches!(target, Dialect::Sqlite) {
                "IFNULL".to_string()
            } else if is_tsql_family(target) {
                "ISNULL".to_string()
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
        "STRPOS" if is_postgres_family(source) && matches!(target, Dialect::Sqlite) => {
            "INSTR".to_string()
        }
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

fn map_data_type_for_source(dt: DataType, source: Dialect, target: Dialect) -> DataType {
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
                "MEDIUMBLOB" | "LONGBLOB"
            ) =>
        {
            DataType::Blob
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
