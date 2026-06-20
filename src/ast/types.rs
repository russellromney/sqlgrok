use serde::{Deserialize, Serialize};

use crate::dialects::Dialect;

// ═══════════════════════════════════════════════════════════════════════
// Comment types
// ═══════════════════════════════════════════════════════════════════════

/// The type of a SQL comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommentType {
    /// Single-line comment starting with `--`
    Line,
    /// Block comment delimited by `/* ... */`
    Block,
    /// MySQL-style hash comment starting with `#`
    Hash,
}

// ═══════════════════════════════════════════════════════════════════════
// Identifier quoting style
// ═══════════════════════════════════════════════════════════════════════

/// How an identifier (column, table, alias) was quoted in the source SQL.
///
/// Used to preserve and transform quoting across dialects (e.g. backtick
/// for MySQL/BigQuery → double-quote for PostgreSQL → bracket for T-SQL).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum QuoteStyle {
    /// Bare / unquoted identifier
    #[default]
    None,
    /// `"identifier"` — ANSI SQL, PostgreSQL, Oracle, Snowflake, etc.
    DoubleQuote,
    /// `` `identifier` `` — MySQL, BigQuery, Hive, Spark, etc.
    Backtick,
    /// `[identifier]` — T-SQL / SQL Server
    Bracket,
}

impl QuoteStyle {
    /// Returns the canonical quoting style for the given dialect.
    #[must_use]
    pub fn for_dialect(dialect: crate::dialects::Dialect) -> Self {
        use crate::dialects::Dialect;
        match dialect {
            Dialect::Tsql | Dialect::Fabric => QuoteStyle::Bracket,
            Dialect::Mysql
            | Dialect::BigQuery
            | Dialect::Hive
            | Dialect::Spark
            | Dialect::Databricks
            | Dialect::Doris
            | Dialect::SingleStore
            | Dialect::StarRocks => QuoteStyle::Backtick,
            // ANSI, Postgres, Oracle, Snowflake, Presto, Trino, etc.
            _ => QuoteStyle::DoubleQuote,
        }
    }

    /// Returns `true` when the identifier carries explicit quoting.
    #[must_use]
    pub fn is_quoted(self) -> bool {
        !matches!(self, QuoteStyle::None)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Top-level statement types
// ═══════════════════════════════════════════════════════════════════════

/// A fully parsed SQL statement.
///
/// Corresponds to the top-level node in sqlglot's expression tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    CreateTable(CreateTableStatement),
    DropTable(DropTableStatement),
    /// CREATE INDEX ...
    CreateIndex(CreateIndexStatement),
    /// DROP INDEX ...
    DropIndex(DropIndexStatement),
    /// UNION / INTERSECT / EXCEPT between queries
    SetOperation(SetOperationStatement),
    /// ALTER TABLE ...
    AlterTable(AlterTableStatement),
    /// CREATE VIEW ...
    CreateView(CreateViewStatement),
    /// CREATE SEQUENCE ... (most options dropped on output; an optional
    /// `AS <type>` is preserved and type-mapped).
    CreateSequence(CreateSequenceStatement),
    /// CREATE FUNCTION ... — RETURNS/LANGUAGE/modifier clauses dropped,
    /// parameter types mapped, body preserved (prefixed AS).
    CreateFunction(CreateFunctionStatement),
    /// DROP VIEW ...
    DropView(DropViewStatement),
    /// TRUNCATE TABLE ...
    Truncate(TruncateStatement),
    /// BEGIN / COMMIT / ROLLBACK
    Transaction(TransactionStatement),
    /// EXPLAIN <statement>
    Explain(ExplainStatement),
    /// USE database
    Use(UseStatement),
    /// MERGE INTO ... USING ... WHEN MATCHED / WHEN NOT MATCHED
    Merge(MergeStatement),
    /// Opaque command preserved textually until sqlgrok grows a typed AST for it.
    Command(CommandStatement),
    /// Opaque fallback SQL preserved textually. New target behavior should move
    /// to parser/generator-owned statement variants instead of this carrier.
    Raw(RawStatement),
    /// Raw / passthrough expression (for expressions that don't fit a specific statement type)
    Expression(Expr),
}

/// CREATE SEQUENCE statement. SQLGlot drops sequence options (START,
/// INCREMENT, CYCLE, CACHE, SHARING, ...) when the target can't represent
/// them, keeping only the name and an optional `AS <datatype>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSequenceStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    #[serde(default)]
    pub or_replace: bool,
    #[serde(default)]
    pub temporary: bool,
    #[serde(default)]
    pub if_not_exists: bool,
    pub name: TableRef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub as_type: Option<DataType>,
}

/// CREATE FUNCTION statement (sqlite-target lowering). SQLGlot drops the
/// RETURNS clause and modifier properties (LANGUAGE, IMMUTABLE, SET, ...),
/// maps parameter types, and renders the body prefixed with AS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateFunctionStatement {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    #[serde(default)]
    pub or_replace: bool,
    #[serde(default)]
    pub temporary: bool,
    /// Function name, captured verbatim from source (dotted/quoted preserved).
    pub name: String,
    /// Whether the source had a parenthesized parameter list. A function
    /// declared without `(...)` (e.g. `CREATE FUNCTION a AS b`) renders
    /// without parentheses.
    #[serde(default)]
    pub parenthesized: bool,
    pub params: Vec<FunctionParam>,
    /// Body rendered verbatim after `AS` (e.g. `RETURN x`, `'sql'`, `$$...$$`,
    /// `(expr)`, `SELECT ...`). None when the function has no body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

/// A single CREATE FUNCTION parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionParam {
    /// Parameter mode rendered after the type: "IN", "OUT", "IN OUT", "VARIADIC".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Parameter name (verbatim source: may be `@x`, `"x"`, dotted, etc.).
    /// None is unused; a bare unnamed type parameter carries its text here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The parameter type (mapped for the target). None for a bare param,
    /// in which case `name` carries the verbatim token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub sql: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalization: Option<RawStatementNormalization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_dialect: Option<Dialect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandKind {
    Copy,
    Show,
    Pivot,
    Unpivot,
    CreateTypeEnum,
    Generic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub sql: String,
    pub kind: CommandKind,
    #[serde(default)]
    pub drop_for_sqlite: bool,
}

// ═══════════════════════════════════════════════════════════════════════
// SELECT
// ═══════════════════════════════════════════════════════════════════════

/// A SELECT statement, including CTEs.
///
/// Aligned with sqlglot's `Select` expression which wraps `With`, `From`,
/// `Where`, `Group`, `Having`, `Order`, `Limit`, `Offset`, `Window`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    /// Common Table Expressions (WITH clause)
    pub ctes: Vec<Cte>,
    pub distinct: bool,
    /// PostgreSQL `DISTINCT ON (expr, ...)`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub distinct_on: Vec<Expr>,
    /// TOP N (TSQL-style)
    pub top: Option<Box<Expr>>,
    pub columns: Vec<SelectItem>,
    pub from: Option<FromClause>,
    pub joins: Vec<JoinClause>,
    pub where_clause: Option<Expr>,
    pub group_by: Vec<Expr>,
    pub having: Option<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub limit: Option<Expr>,
    #[serde(default)]
    pub limit_renders_as_tsql_top: bool,
    pub offset: Option<Expr>,
    /// ClickHouse-style LIMIT n BY expr, ...
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limit_by: Vec<Expr>,
    /// Oracle-style FETCH FIRST n ROWS ONLY
    pub fetch_first: Option<Expr>,
    /// QUALIFY clause (BigQuery, Snowflake)
    pub qualify: Option<Expr>,
    /// Named WINDOW definitions
    pub window_definitions: Vec<WindowDefinition>,
    /// Locking clause such as `FOR UPDATE`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
}

/// A Common Table Expression: `name [(col1, col2)] AS [NOT] MATERIALIZED (query)`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cte {
    pub name: String,
    #[serde(default)]
    pub name_quote_style: QuoteStyle,
    pub columns: Vec<String>,
    pub query: Box<Statement>,
    pub materialized: Option<bool>,
    pub recursive: bool,
}

/// Named WINDOW definition: `window_name AS (window_spec)`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowDefinition {
    pub name: String,
    pub spec: WindowSpec,
}

// ═══════════════════════════════════════════════════════════════════════
// Set operations (UNION, INTERSECT, EXCEPT)
// ═══════════════════════════════════════════════════════════════════════

/// UNION / INTERSECT / EXCEPT between two or more queries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetOperationStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub op: SetOperationType,
    pub all: bool,
    pub left: Box<Statement>,
    pub right: Box<Statement>,
    pub order_by: Vec<OrderByItem>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetOperationType {
    Union,
    Intersect,
    Except,
}

// ═══════════════════════════════════════════════════════════════════════
// SELECT items and FROM
// ═══════════════════════════════════════════════════════════════════════

/// An item in a SELECT list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum SelectItem {
    /// `*`
    Wildcard,
    /// `table.*`
    QualifiedWildcard { table: String },
    /// An expression with optional alias: `expr AS alias`
    Expr {
        expr: Expr,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
    },
}

/// A FROM clause, now supporting subqueries and multiple tables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FromClause {
    pub source: TableSource,
}

/// Column names attached to a table alias, e.g. `UNNEST(x) AS t(col)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AliasColumn {
    pub name: String,
    #[serde(default)]
    pub quote_style: QuoteStyle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RowsFromAliasColumn {
    pub name: String,
    #[serde(default)]
    pub quote_style: QuoteStyle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RowsFromItem {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<Expr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    pub alias_quote_style: QuoteStyle,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_columns: Vec<RowsFromAliasColumn>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RawTableFunctionKind {
    JsonTable,
    XmlTable,
}

/// A table source can be a table reference, subquery, or table function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableSource {
    Table(TableRef),
    Subquery {
        query: Box<Statement>,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
    },
    TableFunction {
        name: String,
        args: Vec<Expr>,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
    },
    RawTableFunction {
        kind: RawTableFunctionKind,
        name: String,
        body: String,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
    },
    TableWithTails {
        table: TableRef,
        tails: String,
    },
    Raw {
        sql: String,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        normalization: Option<RawTableSourceNormalization>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_dialect: Option<Dialect>,
    },
    Values {
        rows: Vec<Vec<Expr>>,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
    },
    /// LATERAL subquery or function
    Lateral {
        source: Box<TableSource>,
    },
    /// ROWS FROM (func(...), ...)
    RowsFrom {
        items: Vec<RowsFromItem>,
        #[serde(default)]
        with_ordinality: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        alias_columns: Vec<RowsFromAliasColumn>,
    },
    /// UNNEST(array_expr)
    Unnest {
        expr: Box<Expr>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        extra_exprs: Vec<Expr>,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        alias_columns: Vec<AliasColumn>,
        #[serde(default)]
        with_offset: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        offset_alias: Option<String>,
        #[serde(default)]
        offset_alias_quote_style: QuoteStyle,
        #[serde(default)]
        use_generated_offset_alias: bool,
        #[serde(default)]
        with_ordinality: bool,
    },
    /// PIVOT (aggregate FOR column IN (values))
    Pivot {
        source: Box<TableSource>,
        aggregate: Box<Expr>,
        for_column: String,
        in_values: Vec<PivotValue>,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
    },
    /// UNPIVOT (value_column FOR name_column IN (columns))
    Unpivot {
        source: Box<TableSource>,
        value_column: String,
        for_column: String,
        in_columns: Vec<PivotValue>,
        alias: Option<String>,
        #[serde(default)]
        alias_quote_style: QuoteStyle,
    },
}

/// A value/column in a PIVOT IN list, optionally aliased.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PivotValue {
    pub value: Expr,
    pub alias: Option<String>,
    #[serde(default)]
    pub alias_quote_style: QuoteStyle,
}

/// A reference to a table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableRef {
    pub catalog: Option<String>,
    pub schema: Option<String>,
    pub name: String,
    pub alias: Option<String>,
    /// How the table name was quoted in the source SQL.
    #[serde(default)]
    pub name_quote_style: QuoteStyle,
    /// How the alias was quoted in the source SQL.
    #[serde(default)]
    pub alias_quote_style: QuoteStyle,
}

/// A JOIN clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: TableSource,
    pub on: Option<Expr>,
    pub using: Vec<String>,
    /// SQLite parser metadata for identity rendering: comma joins become
    /// CROSS JOIN and criteria-less joins render `ON TRUE`.
    #[serde(default)]
    pub sqlite_identity_normalization: bool,
}

/// The type of JOIN.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinType {
    /// Bare `JOIN` syntax, semantically an inner join.
    Join,
    Inner,
    Left,
    LeftOuter,
    Right,
    RightOuter,
    Full,
    FullOuter,
    Cross,
    /// Comma join syntax (`FROM a, b`), semantically a cross join.
    Comma,
    /// NATURAL JOIN
    Natural,
    /// LATERAL JOIN
    Lateral,
    /// SQL Server-style CROSS APPLY, generated as an inner lateral join.
    CrossApply,
    /// SQL Server-style OUTER APPLY, generated as a left lateral join.
    OuterApply,
    /// MySQL STRAIGHT_JOIN
    Straight,
    /// SEMI JOIN — keep left rows matched in right.
    Semi,
    /// ANTI JOIN — keep left rows NOT matched in right.
    Anti,
}

/// An ORDER BY item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderByItem {
    pub expr: Expr,
    pub ascending: bool,
    #[serde(default)]
    pub explicit_direction: bool,
    /// NULLS FIRST / NULLS LAST
    pub nulls_first: Option<bool>,
    /// Whether NULLS FIRST / NULLS LAST was inferred from source dialect semantics.
    #[serde(default)]
    pub implicit_nulls: bool,
}

/// Null-ordering semantics attached to raw aggregate/function argument text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RawOrderNullsPolicy {
    /// PostgreSQL defaults: ascending/default sorts NULLS LAST, descending sorts
    /// NULLS FIRST. This is parser-owned source semantics consumed by targets
    /// that need explicit null ordering when rendering raw argument carriers.
    PostgresDefault,
}

/// Array-literal rendering policy for raw `UNNEST(...)` table sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RawUnnestArrayLiteralPolicy {
    /// `UNNEST([1, 2])` -> `UNNEST(ARRAY(1, 2))`.
    ArrayCall,
    /// `UNNEST([1, 2])` -> `UNNEST("1, 2")`.
    SqliteQuotedString,
}

/// Parser-owned normalization flags for raw table-source text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawTableSourceNormalization {
    #[serde(default)]
    pub strip_postgres_values_column_aliases: bool,
    #[serde(default)]
    pub rewrite_unnest_with_offset: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unnest_array_literal: Option<RawUnnestArrayLiteralPolicy>,
    #[serde(default)]
    pub quote_backticks: bool,
    #[serde(default)]
    pub uppercase_function_names: bool,
    #[serde(default)]
    pub normalize_typed_literals: bool,
}

/// Parser-owned normalization flags for raw statement text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawStatementNormalization {
    #[serde(default)]
    pub normalize_postgres_recursive_cte: bool,
    #[serde(default)]
    pub normalize_insert_into_function: bool,
}

// ═══════════════════════════════════════════════════════════════════════
// Expressions (the core of the AST)
// ═══════════════════════════════════════════════════════════════════════

/// An expression in SQL.
///
/// This enum is aligned with sqlglot's Expression class hierarchy.
/// Key additions over the basic implementation:
/// - Subquery, Exists, Cast, Extract, Window functions
/// - TypedString, Interval, Array/Struct constructors
/// - Postgres-style casting (`::`)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// A column reference, possibly qualified: `[catalog.][schema.]table.column`
    Column {
        table: Option<String>,
        name: String,
        /// How the column name was quoted in the source SQL.
        #[serde(default)]
        quote_style: QuoteStyle,
        /// How the table qualifier was quoted, if present.
        #[serde(default)]
        table_quote_style: QuoteStyle,
    },
    /// A numeric literal.
    Number(String),
    /// A hexadecimal literal, generated as `x'...'`.
    HexString(String),
    /// A string literal.
    StringLiteral(String),
    /// A JSON object-key literal used by dialects whose JSON access
    /// operator treats a string operand as a key rather than a JSONPath.
    JsonKey(String),
    /// A Postgres escape string literal, `E'...'`.
    EscapedStringLiteral(String),
    /// A boolean literal.
    Boolean(bool),
    /// NULL literal.
    Null,
    /// A binary operation: `left op right`
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    /// A unary operation: `op expr`
    UnaryOp { op: UnaryOperator, expr: Box<Expr> },
    /// A function call: `name(args...)` with optional DISTINCT, ORDER BY, etc.
    Function {
        name: String,
        args: Vec<Expr>,
        distinct: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        raw_order_nulls: Option<RawOrderNullsPolicy>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        arg_order_by: Vec<OrderByItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        arg_limit: Option<Box<Expr>>,
        /// FILTER (WHERE expr) clause on aggregate
        filter: Option<Box<Expr>>,
        /// OVER window specification for window functions
        over: Option<WindowSpec>,
    },
    /// `func(...) WITHIN GROUP (ORDER BY ...)` with optional FILTER / OVER.
    WithinGroup {
        expr: Box<Expr>,
        order_by: Vec<OrderByItem>,
        filter: Option<Box<Expr>>,
        over: Option<WindowSpec>,
    },
    /// `expr BETWEEN low AND high`
    Between {
        expr: Box<Expr>,
        low: Box<Expr>,
        high: Box<Expr>,
        negated: bool,
    },
    /// `expr IN (list...)` or `expr IN (subquery)`
    InList {
        expr: Box<Expr>,
        list: Vec<Expr>,
        negated: bool,
    },
    /// `expr IN (SELECT ...)`
    InSubquery {
        expr: Box<Expr>,
        subquery: Box<Statement>,
        negated: bool,
    },
    /// `expr op ANY(subexpr)` — PostgreSQL array/subquery comparison
    AnyOp {
        expr: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    /// `expr op ALL(subexpr)` — PostgreSQL array/subquery comparison
    AllOp {
        expr: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    /// `expr IS [NOT] NULL`
    IsNull { expr: Box<Expr>, negated: bool },
    /// `expr IS [NOT] TRUE` / `expr IS [NOT] FALSE`
    IsBool {
        expr: Box<Expr>,
        value: bool,
        negated: bool,
    },
    /// `expr [NOT] LIKE pattern [ESCAPE escape_char]`
    Like {
        expr: Box<Expr>,
        pattern: Box<Expr>,
        negated: bool,
        escape: Option<Box<Expr>>,
    },
    /// `expr [NOT] ILIKE pattern [ESCAPE escape_char]` (case-insensitive LIKE)
    ILike {
        expr: Box<Expr>,
        pattern: Box<Expr>,
        negated: bool,
        #[serde(default)]
        lower_on_ansi: bool,
        escape: Option<Box<Expr>>,
    },
    /// `expr SIMILAR TO pattern [ESCAPE escape_char]` (PostgreSQL)
    SimilarTo {
        expr: Box<Expr>,
        pattern: Box<Expr>,
        escape: Option<Box<Expr>>,
    },
    /// `CASE [operand] WHEN ... THEN ... ELSE ... END`
    Case {
        operand: Option<Box<Expr>>,
        when_clauses: Vec<(Expr, Expr)>,
        else_clause: Option<Box<Expr>>,
    },
    /// A parenthesized sub-expression.
    Nested(Box<Expr>),
    /// A wildcard `*` used in contexts like `COUNT(*)`.
    Wildcard,
    /// A scalar subquery: `(SELECT ...)`
    Subquery(Box<Statement>),
    /// `EXISTS (SELECT ...)`
    Exists {
        subquery: Box<Statement>,
        negated: bool,
    },
    /// `CAST(expr AS type)` or `expr::type` (PostgreSQL)
    Cast {
        expr: Box<Expr>,
        data_type: DataType,
    },
    /// `TRY_CAST(expr AS type)`
    TryCast {
        expr: Box<Expr>,
        data_type: DataType,
    },
    /// `EXTRACT(field FROM expr)`
    Extract {
        field: DateTimeField,
        expr: Box<Expr>,
    },
    /// `INTERVAL 'value' unit`
    Interval {
        value: Box<Expr>,
        unit: Option<DateTimeField>,
        /// Original spelling of the unit token (e.g. \"SECONDS\",
        /// \"minute\"). Used to preserve case and plural form on
        /// round-trip. When None, the generator falls back to the
        /// canonical DateTimeField keyword.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit_text: Option<String>,
    },
    /// Array literal: `ARRAY[1, 2, 3]` or `[1, 2, 3]`
    ArrayLiteral(Vec<Expr>),
    /// SQLite-source array literal fallback. SQLGlot renders these as a
    /// quoted comma-list for SQLite identity, but normal array syntax for
    /// other targets.
    SqliteArrayLiteral(Vec<Expr>),
    /// Struct literal / row constructor: `(1, 'a', true)`
    Tuple(Vec<Expr>),
    /// `COALESCE(a, b, c)` / `NVL(a, b)` / T-SQL `ISNULL(a, b)`.
    ///
    /// SQLGlot carries `is_nvl` and `is_null` flags so home generators can
    /// re-render `NVL` (oracle/bigquery/clickhouse) or `ISNULL` (tsql/fabric)
    /// while other targets render plain `COALESCE`.
    Coalesce {
        items: Vec<Expr>,
        #[serde(default)]
        is_nvl: bool,
        #[serde(default)]
        is_null: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_name: Option<String>,
    },
    /// `IF(condition, true_val, false_val)` (MySQL, BigQuery)
    If {
        condition: Box<Expr>,
        true_val: Box<Expr>,
        false_val: Option<Box<Expr>>,
    },
    /// `NULLIF(a, b)`
    NullIf { expr: Box<Expr>, r#else: Box<Expr> },
    /// `expr COLLATE collation`
    Collate { expr: Box<Expr>, collation: String },
    /// Parameter / placeholder: `$1`, `?`, `:name`
    Parameter(String),
    /// PostgreSQL positional parameter (`$1`) for target-specific rendering.
    PostgresParameter(String),
    /// A type expression used in DDL contexts or CAST
    TypeExpr(DataType),
    /// `table.*` in expression context
    QualifiedWildcard { table: String },
    /// Star expression `*`
    Star,
    /// Alias expression: `expr AS name`
    Alias { expr: Box<Expr>, name: String },
    /// Array access: `expr[index]`
    ArrayIndex { expr: Box<Expr>, index: Box<Expr> },
    /// PostgreSQL array access, whose indexes are one-based. Generators that
    /// need zero-based indexes can lower the index without source branching.
    PostgresArrayIndex { expr: Box<Expr>, index: Box<Expr> },
    /// JSON access: `expr->key` or `expr->>key`
    JsonAccess {
        expr: Box<Expr>,
        path: Box<Expr>,
        /// false = ->, true = ->>
        as_text: bool,
    },
    /// Lambda expression: `x -> x + 1`
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    /// `DEFAULT` keyword in INSERT/UPDATE contexts
    Default,
    /// `CUBE(a, b, ...)` in GROUP BY clause
    Cube { exprs: Vec<Expr> },
    /// `ROLLUP(a, b, ...)` in GROUP BY clause
    Rollup { exprs: Vec<Expr> },
    /// `GROUPING SETS ((a, b), (c), ...)` in GROUP BY clause
    GroupingSets { sets: Vec<Expr> },
    /// A typed function expression with semantic awareness.
    /// Enables per-function, per-dialect code generation and transpilation.
    TypedFunction {
        func: TypedFunction,
        /// FILTER (WHERE expr) clause on aggregate
        filter: Option<Box<Expr>>,
        /// OVER window specification for window functions
        over: Option<WindowSpec>,
    },
    /// An expression with attached SQL comments.
    /// Wraps an inner expression so that comments survive transformations.
    Commented {
        expr: Box<Expr>,
        comments: Vec<String>,
    },
}

// ═══════════════════════════════════════════════════════════════════════
// Window specification
// ═══════════════════════════════════════════════════════════════════════

/// Window specification for window functions: OVER (PARTITION BY ... ORDER BY ... frame)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowSpec {
    /// Reference to a named window
    pub window_ref: Option<String>,
    pub partition_by: Vec<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub frame: Option<WindowFrame>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowFrame {
    pub kind: WindowFrameKind,
    pub start: WindowFrameBound,
    pub end: Option<WindowFrameBound>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<WindowFrameExclude>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowFrameKind {
    Rows,
    Range,
    Groups,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WindowFrameBound {
    CurrentRow,
    Preceding(Option<Box<Expr>>), // None = UNBOUNDED PRECEDING
    Following(Option<Box<Expr>>), // None = UNBOUNDED FOLLOWING
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowFrameExclude {
    NoOthers,
    CurrentRow,
    Group,
    Ties,
}

// ═══════════════════════════════════════════════════════════════════════
// Date/time fields (for EXTRACT, INTERVAL)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateTimeField {
    Year,
    Quarter,
    Month,
    Week,
    Day,
    DayOfWeek,
    DayOfYear,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Epoch,
    Timezone,
    TimezoneHour,
    TimezoneMinute,
}

// ═══════════════════════════════════════════════════════════════════════
// Trim type
// ═══════════════════════════════════════════════════════════════════════

/// The type of TRIM operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrimType {
    Leading,
    Trailing,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatePartFunction {
    DayOfMonth,
    DayOfYear,
    DayOfWeek,
    WeekOfYear,
    Week,
}

// ═══════════════════════════════════════════════════════════════════════
// Typed function expressions
// ═══════════════════════════════════════════════════════════════════════

/// Typed function variants enabling per-function transpilation rules,
/// function signature validation, and dialect-specific code generation.
///
/// Each variant carries semantically typed arguments rather than a generic
/// `Vec<Expr>`, allowing the generator to emit dialect-specific SQL.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypedFunction {
    // ── Date/Time ──────────────────────────────────────────────────────
    /// `DATE_ADD(expr, interval)` — add an interval to a date/timestamp
    DateAdd {
        expr: Box<Expr>,
        interval: Box<Expr>,
        unit: Option<DateTimeField>,
        #[serde(default)]
        mysql_interval: bool,
    },
    /// `DATE_DIFF(start, end)` — difference between two dates
    DateDiff {
        start: Box<Expr>,
        end: Box<Expr>,
        unit: Option<DateTimeField>,
    },
    /// `TIMESTAMPDIFF(unit, start, end)` / `TIMESTAMP_DIFF(end, start, unit)`
    TimestampDiff {
        start: Box<Expr>,
        end: Box<Expr>,
        unit: Option<DateTimeField>,
    },
    /// `DATE_PART(part, expr)` — extract a date/time field expression
    DatePart { part: Box<Expr>, expr: Box<Expr> },
    /// `EXTRACT(part FROM expr)` where `part` is itself an expression
    ExtractPart { part: Box<Expr>, expr: Box<Expr> },
    /// `DATE_TRUNC(unit, expr)` — truncate to the given precision
    DateTrunc {
        unit: DateTimeField,
        expr: Box<Expr>,
    },
    /// `DATE_TRUNC(unit_expr, expr)` with a non-standard or expression-valued unit.
    DateTruncExpr {
        unit: Box<Expr>,
        expr: Box<Expr>,
        #[serde(default)]
        timestamp: bool,
    },
    /// `TIMESTAMP_TRUNC(expr, unit)`
    TimestampTrunc {
        unit: DateTimeField,
        expr: Box<Expr>,
    },
    /// `DATE_SUB(expr, interval)` — subtract an interval from a date
    DateSub {
        expr: Box<Expr>,
        interval: Box<Expr>,
        unit: Option<DateTimeField>,
        #[serde(default)]
        mysql_interval: bool,
    },
    /// `CURRENT_DATE`
    CurrentDate,
    /// `CURRENT_TIMESTAMP` / `NOW()` / `GETDATE()`
    CurrentTimestamp,
    /// `UTC_TIME`
    UtcTime { precision: Option<Box<Expr>> },
    /// `UTC_TIMESTAMP`
    UtcTimestamp { precision: Option<Box<Expr>> },
    /// `VERSION()` / `CURRENT_VERSION()`
    Version,
    /// `CONVERT_TIMEZONE([source_tz,] target_tz, timestamp)` / `CONVERT_TZ`
    ConvertTimezone {
        source_tz: Option<Box<Expr>>,
        target_tz: Box<Expr>,
        timestamp: Box<Expr>,
    },
    /// `MAKE_TIME` / `MAKETIME` / `TIME_FROM_PARTS`
    TimeFromParts { parts: Vec<Expr> },
    /// `TO_TIMESTAMP(epoch[, format])` / `FROM_UNIXTIME(epoch[, format])` / `UNIX_TO_TIME(epoch[, format])`
    UnixToTime {
        expr: Box<Expr>,
        format: Option<Box<Expr>>,
    },
    /// `STR_TO_DATE(expr, format)` / `TO_DATE(expr, format)` / `PARSE_DATE`
    StrToDate { expr: Box<Expr>, format: Box<Expr> },
    /// `STR_TO_TIME(expr, format)` / `TO_TIMESTAMP` / `PARSE_DATETIME`
    StrToTime { expr: Box<Expr>, format: Box<Expr> },
    /// `TIME_TO_STR(expr, format)` / `DATE_FORMAT` / `FORMAT_DATETIME`
    TimeToStr { expr: Box<Expr>, format: Box<Expr> },
    /// `TS_OR_DS_TO_DATE(expr)` — convert timestamp or date-string to date
    TsOrDsToDate { expr: Box<Expr> },
    /// `DAYOFMONTH(expr)` / `DAY_OF_MONTH(expr)` and related date-part aliases
    DatePartAlias {
        name: DatePartFunction,
        expr: Box<Expr>,
    },
    /// `YEAR(expr)` — extract year from a date/timestamp
    Year { expr: Box<Expr> },
    /// `MONTH(expr)` — extract month from a date/timestamp
    Month { expr: Box<Expr> },
    /// `DAY(expr)` — extract day from a date/timestamp
    Day { expr: Box<Expr> },

    // ── String ─────────────────────────────────────────────────────────
    /// `TRIM([LEADING|TRAILING|BOTH] [chars FROM] expr)`
    Trim {
        expr: Box<Expr>,
        trim_type: TrimType,
        trim_chars: Option<Box<Expr>>,
    },
    /// `SUBSTRING(expr, start [, length])` / `SUBSTR`
    Substring {
        expr: Box<Expr>,
        start: Box<Expr>,
        length: Option<Box<Expr>>,
    },
    /// `UPPER(expr)` / `UCASE`
    Upper { expr: Box<Expr> },
    /// `LOWER(expr)` / `LCASE`
    Lower { expr: Box<Expr> },
    /// `REGEXP_LIKE(expr, pattern [, flags])` / `~` (Postgres)
    RegexpLike {
        expr: Box<Expr>,
        pattern: Box<Expr>,
        flags: Option<Box<Expr>>,
    },
    /// `REGEXP_EXTRACT(expr, pattern [, group_index])`
    RegexpExtract {
        expr: Box<Expr>,
        pattern: Box<Expr>,
        group_index: Option<Box<Expr>>,
    },
    /// `REGEXP_REPLACE(expr, pattern, replacement [, flags])`
    RegexpReplace {
        expr: Box<Expr>,
        pattern: Box<Expr>,
        replacement: Box<Expr>,
        flags: Option<Box<Expr>>,
    },
    /// `CONCAT_WS(separator, expr, ...)`
    ConcatWs {
        separator: Box<Expr>,
        exprs: Vec<Expr>,
    },
    /// `SPLIT(expr, delimiter)` / `STRING_SPLIT`
    Split {
        expr: Box<Expr>,
        delimiter: Box<Expr>,
    },
    /// `INITCAP(expr)` — capitalize first letter of each word
    Initcap { expr: Box<Expr> },
    /// `LENGTH(expr)` / `LEN` / `CHAR_LENGTH` / `CHARACTER_LENGTH`.
    /// `binary` mirrors SQLGlot's `Length(binary=...)`: true when the source
    /// dialect's LENGTH counts bytes (mysql family, clickhouse, snowflake,
    /// bigquery), which renders LENGTH instead of CHAR_LENGTH for targets
    /// that distinguish the two.
    Length {
        expr: Box<Expr>,
        #[serde(default)]
        binary: bool,
    },
    /// `REPLACE(expr, from, to)`
    Replace {
        expr: Box<Expr>,
        from: Box<Expr>,
        to: Box<Expr>,
    },
    /// `STARTS_WITH(expr, prefix)`
    StartsWith { expr: Box<Expr>, prefix: Box<Expr> },
    /// `REVERSE(expr)`
    Reverse { expr: Box<Expr> },
    /// `LEFT(expr, n)`
    Left { expr: Box<Expr>, n: Box<Expr> },
    /// `RIGHT(expr, n)`
    Right { expr: Box<Expr>, n: Box<Expr> },
    /// `LPAD(expr, length [, pad])`
    Lpad {
        expr: Box<Expr>,
        length: Box<Expr>,
        pad: Option<Box<Expr>>,
    },
    /// `RPAD(expr, length [, pad])`
    Rpad {
        expr: Box<Expr>,
        length: Box<Expr>,
        pad: Option<Box<Expr>>,
    },

    // ── Aggregate ──────────────────────────────────────────────────────
    /// `COUNT(expr)` or `COUNT(DISTINCT expr)` or `COUNT(*)`
    Count { expr: Box<Expr>, distinct: bool },
    /// `SUM([DISTINCT] expr)`
    Sum { expr: Box<Expr>, distinct: bool },
    /// `AVG([DISTINCT] expr)`
    Avg { expr: Box<Expr>, distinct: bool },
    /// `MIN(expr)`
    Min { expr: Box<Expr> },
    /// `MAX(expr)`
    Max { expr: Box<Expr> },
    /// `ARRAY_AGG([DISTINCT] expr)` / `LIST` / `COLLECT_LIST`
    ArrayAgg { expr: Box<Expr>, distinct: bool },
    /// `JSON_AGG([DISTINCT] expr)` / `JSON_ARRAYAGG(expr)` / `JSON_GROUP_ARRAY(expr)`
    JSONAgg { expr: Box<Expr>, distinct: bool },
    /// `JSON_OBJECT_AGG(key, value)` / `JSON_GROUP_OBJECT(key, value)`
    JSONObjectAgg { exprs: Vec<Expr> },
    /// `APPROX_DISTINCT(expr)` / `APPROX_COUNT_DISTINCT`
    ApproxDistinct { expr: Box<Expr> },
    /// `VARIANCE(expr)` / `VAR_SAMP`
    Variance { expr: Box<Expr> },
    /// `STDDEV(expr)` / `STDDEV_SAMP`
    Stddev { expr: Box<Expr> },

    // ── Array ──────────────────────────────────────────────────────────
    /// `ARRAY_CONCAT(arr1, arr2)` / `ARRAY_CAT`
    ArrayConcat { arrays: Vec<Expr> },
    /// `ARRAY_CONTAINS(array, element)` / `ARRAY_POSITION`
    ArrayContains {
        array: Box<Expr>,
        element: Box<Expr>,
    },
    /// `ARRAY_SIZE(expr)` / `ARRAY_LENGTH` / `CARDINALITY`
    ArraySize { expr: Box<Expr> },
    /// `EXPLODE(expr)` — Hive/Spark array expansion
    Explode { expr: Box<Expr> },
    /// `TO_ARRAY(expr)`
    ToArray { expr: Box<Expr> },
    /// `GENERATE_SERIES(start, stop [, step])`
    GenerateSeries {
        start: Box<Expr>,
        stop: Box<Expr>,
        step: Option<Box<Expr>>,
    },
    /// Postgres projection-position `GENERATE_SERIES`, which explodes on targets that support it.
    ExplodingGenerateSeries {
        start: Box<Expr>,
        stop: Box<Expr>,
        step: Option<Box<Expr>>,
    },
    /// `FLATTEN(expr)` — flatten nested arrays
    Flatten { expr: Box<Expr> },

    // ── JSON ───────────────────────────────────────────────────────────
    /// `JSON_EXTRACT(expr, path)` / `JSON_VALUE` / `->` (Postgres)
    JSONExtract { expr: Box<Expr>, path: Box<Expr> },
    /// `JSON_EXTRACT_SCALAR(expr, path)` / `->>`
    JSONExtractScalar { expr: Box<Expr>, path: Box<Expr> },
    /// Postgres `JSON_EXTRACT_PATH[_TEXT](expr, key [, ...])`
    JSONExtractPath {
        expr: Box<Expr>,
        path: Vec<Expr>,
        as_text: bool,
    },
    /// `PARSE_JSON(expr)` / `JSON_PARSE`
    ParseJSON { expr: Box<Expr> },
    /// `JSON_FORMAT(expr)` / `TO_JSON`
    JSONFormat { expr: Box<Expr> },

    // ── Window ─────────────────────────────────────────────────────────
    /// `ROW_NUMBER()`
    RowNumber,
    /// `RANK()`
    Rank,
    /// `DENSE_RANK()`
    DenseRank,
    /// `NTILE(n)`
    NTile { n: Box<Expr> },
    /// `LEAD(expr [, offset [, default]])`
    Lead {
        expr: Box<Expr>,
        offset: Option<Box<Expr>>,
        default: Option<Box<Expr>>,
    },
    /// `LAG(expr [, offset [, default]])`
    Lag {
        expr: Box<Expr>,
        offset: Option<Box<Expr>>,
        default: Option<Box<Expr>>,
    },
    /// `FIRST_VALUE(expr)`
    FirstValue { expr: Box<Expr> },
    /// `LAST_VALUE(expr)`
    LastValue { expr: Box<Expr> },

    // ── Math ───────────────────────────────────────────────────────────
    /// `ABS(expr)`
    Abs { expr: Box<Expr> },
    /// `CEIL(expr)` / `CEILING`
    Ceil { expr: Box<Expr> },
    /// `FLOOR(expr)`
    Floor { expr: Box<Expr> },
    /// `ROUND(expr [, decimals])`
    Round {
        expr: Box<Expr>,
        decimals: Option<Box<Expr>>,
    },
    /// `LOG(expr [, base])` — semantics vary by dialect
    Log {
        expr: Box<Expr>,
        base: Option<Box<Expr>>,
    },
    /// `LN(expr)` — natural logarithm
    Ln { expr: Box<Expr> },
    /// `POW(base, exponent)` / `POWER`
    Pow {
        base: Box<Expr>,
        exponent: Box<Expr>,
    },
    /// `SQRT(expr)`
    Sqrt { expr: Box<Expr> },
    /// `GREATEST(expr, ...)`
    Greatest { exprs: Vec<Expr> },
    /// `LEAST(expr, ...)`
    Least { exprs: Vec<Expr> },
    /// `MOD(a, b)` — modulo function
    Mod { left: Box<Expr>, right: Box<Expr> },
    /// `NUMBER_TO_STR(expr, decimals [, locale])`
    NumberToStr { exprs: Vec<Expr> },

    // ── Conversion ─────────────────────────────────────────────────────
    /// `HEX(expr)` / `TO_HEX`
    Hex { expr: Box<Expr> },
    /// `UNHEX(expr)` / `FROM_HEX`
    Unhex { expr: Box<Expr> },
    /// `MD5(expr)`
    Md5 { expr: Box<Expr> },
    /// `SHA(expr)` / `SHA1`
    Sha { expr: Box<Expr> },
    /// `SHA256(expr)`
    Sha256 { expr: Box<Expr> },
    /// `SHA512(expr)`
    Sha512 { expr: Box<Expr> },
    /// `SHA2(expr, bit_length)` — SHA-256/SHA-512
    Sha2 {
        expr: Box<Expr>,
        bit_length: Box<Expr>,
    },
}

impl TypedFunction {
    /// Walk child expressions, calling `visitor` on each.
    pub fn walk_children<F>(&self, visitor: &mut F)
    where
        F: FnMut(&Expr) -> bool,
    {
        match self {
            // Date/Time
            TypedFunction::DateAdd { expr, interval, .. }
            | TypedFunction::DateSub { expr, interval, .. } => {
                expr.walk(visitor);
                interval.walk(visitor);
            }
            TypedFunction::DateDiff { start, end, .. }
            | TypedFunction::TimestampDiff { start, end, .. } => {
                start.walk(visitor);
                end.walk(visitor);
            }
            TypedFunction::DatePart { part, expr } | TypedFunction::ExtractPart { part, expr } => {
                part.walk(visitor);
                expr.walk(visitor);
            }
            TypedFunction::DateTrunc { expr, .. } | TypedFunction::TimestampTrunc { expr, .. } => {
                expr.walk(visitor);
            }
            TypedFunction::DateTruncExpr { unit, expr, .. } => {
                unit.walk(visitor);
                expr.walk(visitor);
            }
            TypedFunction::ConvertTimezone {
                source_tz,
                target_tz,
                timestamp,
            } => {
                if let Some(source_tz) = source_tz {
                    source_tz.walk(visitor);
                }
                target_tz.walk(visitor);
                timestamp.walk(visitor);
            }
            TypedFunction::CurrentDate
            | TypedFunction::CurrentTimestamp
            | TypedFunction::Version => {}
            TypedFunction::UtcTime { precision } | TypedFunction::UtcTimestamp { precision } => {
                if let Some(precision) = precision {
                    precision.walk(visitor);
                }
            }
            TypedFunction::TimeFromParts { parts } => {
                for part in parts {
                    part.walk(visitor);
                }
            }
            TypedFunction::UnixToTime { expr, format } => {
                expr.walk(visitor);
                if let Some(format) = format {
                    format.walk(visitor);
                }
            }
            TypedFunction::StrToDate { expr, format }
            | TypedFunction::StrToTime { expr, format }
            | TypedFunction::TimeToStr { expr, format } => {
                expr.walk(visitor);
                format.walk(visitor);
            }
            TypedFunction::TsOrDsToDate { expr }
            | TypedFunction::DatePartAlias { expr, .. }
            | TypedFunction::Year { expr }
            | TypedFunction::Month { expr }
            | TypedFunction::Day { expr } => expr.walk(visitor),

            // String
            TypedFunction::Trim {
                expr, trim_chars, ..
            } => {
                expr.walk(visitor);
                if let Some(c) = trim_chars {
                    c.walk(visitor);
                }
            }
            TypedFunction::Substring {
                expr,
                start,
                length,
            } => {
                expr.walk(visitor);
                start.walk(visitor);
                if let Some(l) = length {
                    l.walk(visitor);
                }
            }
            TypedFunction::Upper { expr }
            | TypedFunction::Lower { expr }
            | TypedFunction::Initcap { expr }
            | TypedFunction::Length { expr, .. }
            | TypedFunction::Reverse { expr } => expr.walk(visitor),
            TypedFunction::RegexpLike {
                expr,
                pattern,
                flags,
            } => {
                expr.walk(visitor);
                pattern.walk(visitor);
                if let Some(f) = flags {
                    f.walk(visitor);
                }
            }
            TypedFunction::RegexpExtract {
                expr,
                pattern,
                group_index,
            } => {
                expr.walk(visitor);
                pattern.walk(visitor);
                if let Some(g) = group_index {
                    g.walk(visitor);
                }
            }
            TypedFunction::RegexpReplace {
                expr,
                pattern,
                replacement,
                flags,
            } => {
                expr.walk(visitor);
                pattern.walk(visitor);
                replacement.walk(visitor);
                if let Some(f) = flags {
                    f.walk(visitor);
                }
            }
            TypedFunction::ConcatWs { separator, exprs } => {
                separator.walk(visitor);
                for e in exprs {
                    e.walk(visitor);
                }
            }
            TypedFunction::Split { expr, delimiter } => {
                expr.walk(visitor);
                delimiter.walk(visitor);
            }
            TypedFunction::Replace { expr, from, to } => {
                expr.walk(visitor);
                from.walk(visitor);
                to.walk(visitor);
            }
            TypedFunction::StartsWith { expr, prefix } => {
                expr.walk(visitor);
                prefix.walk(visitor);
            }
            TypedFunction::Left { expr, n } | TypedFunction::Right { expr, n } => {
                expr.walk(visitor);
                n.walk(visitor);
            }
            TypedFunction::Lpad { expr, length, pad }
            | TypedFunction::Rpad { expr, length, pad } => {
                expr.walk(visitor);
                length.walk(visitor);
                if let Some(p) = pad {
                    p.walk(visitor);
                }
            }

            // Aggregate
            TypedFunction::Count { expr, .. }
            | TypedFunction::Sum { expr, .. }
            | TypedFunction::Avg { expr, .. }
            | TypedFunction::Min { expr }
            | TypedFunction::Max { expr }
            | TypedFunction::ArrayAgg { expr, .. }
            | TypedFunction::JSONAgg { expr, .. }
            | TypedFunction::ApproxDistinct { expr }
            | TypedFunction::Variance { expr }
            | TypedFunction::Stddev { expr } => expr.walk(visitor),
            TypedFunction::JSONObjectAgg { exprs } => {
                for e in exprs {
                    e.walk(visitor);
                }
            }

            // Array
            TypedFunction::ArrayConcat { arrays } => {
                for a in arrays {
                    a.walk(visitor);
                }
            }
            TypedFunction::ArrayContains { array, element } => {
                array.walk(visitor);
                element.walk(visitor);
            }
            TypedFunction::ArraySize { expr }
            | TypedFunction::Explode { expr }
            | TypedFunction::ToArray { expr }
            | TypedFunction::Flatten { expr } => expr.walk(visitor),
            TypedFunction::GenerateSeries { start, stop, step }
            | TypedFunction::ExplodingGenerateSeries { start, stop, step } => {
                start.walk(visitor);
                stop.walk(visitor);
                if let Some(s) = step {
                    s.walk(visitor);
                }
            }

            // JSON
            TypedFunction::JSONExtract { expr, path }
            | TypedFunction::JSONExtractScalar { expr, path } => {
                expr.walk(visitor);
                path.walk(visitor);
            }
            TypedFunction::JSONExtractPath { expr, path, .. } => {
                expr.walk(visitor);
                for part in path {
                    part.walk(visitor);
                }
            }
            TypedFunction::ParseJSON { expr } | TypedFunction::JSONFormat { expr } => {
                expr.walk(visitor)
            }

            // Window
            TypedFunction::RowNumber | TypedFunction::Rank | TypedFunction::DenseRank => {}
            TypedFunction::NTile { n } => n.walk(visitor),
            TypedFunction::Lead {
                expr,
                offset,
                default,
            }
            | TypedFunction::Lag {
                expr,
                offset,
                default,
            } => {
                expr.walk(visitor);
                if let Some(o) = offset {
                    o.walk(visitor);
                }
                if let Some(d) = default {
                    d.walk(visitor);
                }
            }
            TypedFunction::FirstValue { expr } | TypedFunction::LastValue { expr } => {
                expr.walk(visitor)
            }

            // Math
            TypedFunction::Abs { expr }
            | TypedFunction::Ceil { expr }
            | TypedFunction::Floor { expr }
            | TypedFunction::Ln { expr }
            | TypedFunction::Sqrt { expr } => expr.walk(visitor),
            TypedFunction::Round { expr, decimals } => {
                expr.walk(visitor);
                if let Some(d) = decimals {
                    d.walk(visitor);
                }
            }
            TypedFunction::Log { expr, base } => {
                expr.walk(visitor);
                if let Some(b) = base {
                    b.walk(visitor);
                }
            }
            TypedFunction::Pow { base, exponent } => {
                base.walk(visitor);
                exponent.walk(visitor);
            }
            TypedFunction::Greatest { exprs } | TypedFunction::Least { exprs } => {
                for e in exprs {
                    e.walk(visitor);
                }
            }
            TypedFunction::Mod { left, right } => {
                left.walk(visitor);
                right.walk(visitor);
            }
            TypedFunction::NumberToStr { exprs } => {
                for e in exprs {
                    e.walk(visitor);
                }
            }

            // Conversion
            TypedFunction::Hex { expr }
            | TypedFunction::Unhex { expr }
            | TypedFunction::Md5 { expr }
            | TypedFunction::Sha { expr }
            | TypedFunction::Sha256 { expr }
            | TypedFunction::Sha512 { expr } => expr.walk(visitor),
            TypedFunction::Sha2 { expr, bit_length } => {
                expr.walk(visitor);
                bit_length.walk(visitor);
            }
        }
    }

    /// Transform child expressions, returning a new `TypedFunction`.
    #[must_use]
    pub fn transform_children<F>(self, func: &F) -> TypedFunction
    where
        F: Fn(Expr) -> Expr,
    {
        match self {
            // Date/Time
            TypedFunction::DateAdd {
                expr,
                interval,
                unit,
                mysql_interval,
            } => TypedFunction::DateAdd {
                expr: Box::new(expr.transform(func)),
                interval: Box::new(interval.transform(func)),
                unit,
                mysql_interval,
            },
            TypedFunction::DateDiff { start, end, unit } => TypedFunction::DateDiff {
                start: Box::new(start.transform(func)),
                end: Box::new(end.transform(func)),
                unit,
            },
            TypedFunction::TimestampDiff { start, end, unit } => TypedFunction::TimestampDiff {
                start: Box::new(start.transform(func)),
                end: Box::new(end.transform(func)),
                unit,
            },
            TypedFunction::DatePart { part, expr } => TypedFunction::DatePart {
                part: Box::new(part.transform(func)),
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::ExtractPart { part, expr } => TypedFunction::ExtractPart {
                part: Box::new(part.transform(func)),
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::DateTrunc { unit, expr } => TypedFunction::DateTrunc {
                unit,
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::DateTruncExpr {
                unit,
                expr,
                timestamp,
            } => TypedFunction::DateTruncExpr {
                unit: Box::new(unit.transform(func)),
                expr: Box::new(expr.transform(func)),
                timestamp,
            },
            TypedFunction::TimestampTrunc { unit, expr } => TypedFunction::TimestampTrunc {
                unit,
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::DateSub {
                expr,
                interval,
                unit,
                mysql_interval,
            } => TypedFunction::DateSub {
                expr: Box::new(expr.transform(func)),
                interval: Box::new(interval.transform(func)),
                unit,
                mysql_interval,
            },
            TypedFunction::CurrentDate => TypedFunction::CurrentDate,
            TypedFunction::CurrentTimestamp => TypedFunction::CurrentTimestamp,
            TypedFunction::UtcTime { precision } => TypedFunction::UtcTime {
                precision: precision.map(|p| Box::new(p.transform(func))),
            },
            TypedFunction::UtcTimestamp { precision } => TypedFunction::UtcTimestamp {
                precision: precision.map(|p| Box::new(p.transform(func))),
            },
            TypedFunction::Version => TypedFunction::Version,
            TypedFunction::ConvertTimezone {
                source_tz,
                target_tz,
                timestamp,
            } => TypedFunction::ConvertTimezone {
                source_tz: source_tz.map(|expr| Box::new(expr.transform(func))),
                target_tz: Box::new(target_tz.transform(func)),
                timestamp: Box::new(timestamp.transform(func)),
            },
            TypedFunction::TimeFromParts { parts } => TypedFunction::TimeFromParts {
                parts: parts.into_iter().map(|part| part.transform(func)).collect(),
            },
            TypedFunction::UnixToTime { expr, format } => TypedFunction::UnixToTime {
                expr: Box::new(expr.transform(func)),
                format: format.map(|format| Box::new(format.transform(func))),
            },
            TypedFunction::StrToDate { expr, format } => TypedFunction::StrToDate {
                expr: Box::new(expr.transform(func)),
                format: Box::new(format.transform(func)),
            },
            TypedFunction::StrToTime { expr, format } => TypedFunction::StrToTime {
                expr: Box::new(expr.transform(func)),
                format: Box::new(format.transform(func)),
            },
            TypedFunction::TimeToStr { expr, format } => TypedFunction::TimeToStr {
                expr: Box::new(expr.transform(func)),
                format: Box::new(format.transform(func)),
            },
            TypedFunction::TsOrDsToDate { expr } => TypedFunction::TsOrDsToDate {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::DatePartAlias { name, expr } => TypedFunction::DatePartAlias {
                name,
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Year { expr } => TypedFunction::Year {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Month { expr } => TypedFunction::Month {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Day { expr } => TypedFunction::Day {
                expr: Box::new(expr.transform(func)),
            },

            // String
            TypedFunction::Trim {
                expr,
                trim_type,
                trim_chars,
            } => TypedFunction::Trim {
                expr: Box::new(expr.transform(func)),
                trim_type,
                trim_chars: trim_chars.map(|c| Box::new(c.transform(func))),
            },
            TypedFunction::Substring {
                expr,
                start,
                length,
            } => TypedFunction::Substring {
                expr: Box::new(expr.transform(func)),
                start: Box::new(start.transform(func)),
                length: length.map(|l| Box::new(l.transform(func))),
            },
            TypedFunction::Upper { expr } => TypedFunction::Upper {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Lower { expr } => TypedFunction::Lower {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::RegexpLike {
                expr,
                pattern,
                flags,
            } => TypedFunction::RegexpLike {
                expr: Box::new(expr.transform(func)),
                pattern: Box::new(pattern.transform(func)),
                flags: flags.map(|f| Box::new(f.transform(func))),
            },
            TypedFunction::RegexpExtract {
                expr,
                pattern,
                group_index,
            } => TypedFunction::RegexpExtract {
                expr: Box::new(expr.transform(func)),
                pattern: Box::new(pattern.transform(func)),
                group_index: group_index.map(|g| Box::new(g.transform(func))),
            },
            TypedFunction::RegexpReplace {
                expr,
                pattern,
                replacement,
                flags,
            } => TypedFunction::RegexpReplace {
                expr: Box::new(expr.transform(func)),
                pattern: Box::new(pattern.transform(func)),
                replacement: Box::new(replacement.transform(func)),
                flags: flags.map(|f| Box::new(f.transform(func))),
            },
            TypedFunction::ConcatWs { separator, exprs } => TypedFunction::ConcatWs {
                separator: Box::new(separator.transform(func)),
                exprs: exprs.into_iter().map(|e| e.transform(func)).collect(),
            },
            TypedFunction::Split { expr, delimiter } => TypedFunction::Split {
                expr: Box::new(expr.transform(func)),
                delimiter: Box::new(delimiter.transform(func)),
            },
            TypedFunction::Initcap { expr } => TypedFunction::Initcap {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Length { expr, binary } => TypedFunction::Length {
                expr: Box::new(expr.transform(func)),
                binary,
            },
            TypedFunction::Replace { expr, from, to } => TypedFunction::Replace {
                expr: Box::new(expr.transform(func)),
                from: Box::new(from.transform(func)),
                to: Box::new(to.transform(func)),
            },
            TypedFunction::StartsWith { expr, prefix } => TypedFunction::StartsWith {
                expr: Box::new(expr.transform(func)),
                prefix: Box::new(prefix.transform(func)),
            },
            TypedFunction::Reverse { expr } => TypedFunction::Reverse {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Left { expr, n } => TypedFunction::Left {
                expr: Box::new(expr.transform(func)),
                n: Box::new(n.transform(func)),
            },
            TypedFunction::Right { expr, n } => TypedFunction::Right {
                expr: Box::new(expr.transform(func)),
                n: Box::new(n.transform(func)),
            },
            TypedFunction::Lpad { expr, length, pad } => TypedFunction::Lpad {
                expr: Box::new(expr.transform(func)),
                length: Box::new(length.transform(func)),
                pad: pad.map(|p| Box::new(p.transform(func))),
            },
            TypedFunction::Rpad { expr, length, pad } => TypedFunction::Rpad {
                expr: Box::new(expr.transform(func)),
                length: Box::new(length.transform(func)),
                pad: pad.map(|p| Box::new(p.transform(func))),
            },

            // Aggregate
            TypedFunction::Count { expr, distinct } => TypedFunction::Count {
                expr: Box::new(expr.transform(func)),
                distinct,
            },
            TypedFunction::Sum { expr, distinct } => TypedFunction::Sum {
                expr: Box::new(expr.transform(func)),
                distinct,
            },
            TypedFunction::Avg { expr, distinct } => TypedFunction::Avg {
                expr: Box::new(expr.transform(func)),
                distinct,
            },
            TypedFunction::Min { expr } => TypedFunction::Min {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Max { expr } => TypedFunction::Max {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::ArrayAgg { expr, distinct } => TypedFunction::ArrayAgg {
                expr: Box::new(expr.transform(func)),
                distinct,
            },
            TypedFunction::JSONAgg { expr, distinct } => TypedFunction::JSONAgg {
                expr: Box::new(expr.transform(func)),
                distinct,
            },
            TypedFunction::JSONObjectAgg { exprs } => TypedFunction::JSONObjectAgg {
                exprs: exprs.into_iter().map(|e| e.transform(func)).collect(),
            },
            TypedFunction::ApproxDistinct { expr } => TypedFunction::ApproxDistinct {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Variance { expr } => TypedFunction::Variance {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Stddev { expr } => TypedFunction::Stddev {
                expr: Box::new(expr.transform(func)),
            },

            // Array
            TypedFunction::ArrayConcat { arrays } => TypedFunction::ArrayConcat {
                arrays: arrays.into_iter().map(|a| a.transform(func)).collect(),
            },
            TypedFunction::ArrayContains { array, element } => TypedFunction::ArrayContains {
                array: Box::new(array.transform(func)),
                element: Box::new(element.transform(func)),
            },
            TypedFunction::ArraySize { expr } => TypedFunction::ArraySize {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Explode { expr } => TypedFunction::Explode {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::ToArray { expr } => TypedFunction::ToArray {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::GenerateSeries { start, stop, step } => TypedFunction::GenerateSeries {
                start: Box::new(start.transform(func)),
                stop: Box::new(stop.transform(func)),
                step: step.map(|s| Box::new(s.transform(func))),
            },
            TypedFunction::ExplodingGenerateSeries { start, stop, step } => {
                TypedFunction::ExplodingGenerateSeries {
                    start: Box::new(start.transform(func)),
                    stop: Box::new(stop.transform(func)),
                    step: step.map(|s| Box::new(s.transform(func))),
                }
            }
            TypedFunction::Flatten { expr } => TypedFunction::Flatten {
                expr: Box::new(expr.transform(func)),
            },

            // JSON
            TypedFunction::JSONExtract { expr, path } => TypedFunction::JSONExtract {
                expr: Box::new(expr.transform(func)),
                path: Box::new(path.transform(func)),
            },
            TypedFunction::JSONExtractScalar { expr, path } => TypedFunction::JSONExtractScalar {
                expr: Box::new(expr.transform(func)),
                path: Box::new(path.transform(func)),
            },
            TypedFunction::JSONExtractPath {
                expr,
                path,
                as_text,
            } => TypedFunction::JSONExtractPath {
                expr: Box::new(expr.transform(func)),
                path: path.into_iter().map(|part| part.transform(func)).collect(),
                as_text,
            },
            TypedFunction::ParseJSON { expr } => TypedFunction::ParseJSON {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::JSONFormat { expr } => TypedFunction::JSONFormat {
                expr: Box::new(expr.transform(func)),
            },

            // Window
            TypedFunction::RowNumber => TypedFunction::RowNumber,
            TypedFunction::Rank => TypedFunction::Rank,
            TypedFunction::DenseRank => TypedFunction::DenseRank,
            TypedFunction::NTile { n } => TypedFunction::NTile {
                n: Box::new(n.transform(func)),
            },
            TypedFunction::Lead {
                expr,
                offset,
                default,
            } => TypedFunction::Lead {
                expr: Box::new(expr.transform(func)),
                offset: offset.map(|o| Box::new(o.transform(func))),
                default: default.map(|d| Box::new(d.transform(func))),
            },
            TypedFunction::Lag {
                expr,
                offset,
                default,
            } => TypedFunction::Lag {
                expr: Box::new(expr.transform(func)),
                offset: offset.map(|o| Box::new(o.transform(func))),
                default: default.map(|d| Box::new(d.transform(func))),
            },
            TypedFunction::FirstValue { expr } => TypedFunction::FirstValue {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::LastValue { expr } => TypedFunction::LastValue {
                expr: Box::new(expr.transform(func)),
            },

            // Math
            TypedFunction::Abs { expr } => TypedFunction::Abs {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Ceil { expr } => TypedFunction::Ceil {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Floor { expr } => TypedFunction::Floor {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Round { expr, decimals } => TypedFunction::Round {
                expr: Box::new(expr.transform(func)),
                decimals: decimals.map(|d| Box::new(d.transform(func))),
            },
            TypedFunction::Log { expr, base } => TypedFunction::Log {
                expr: Box::new(expr.transform(func)),
                base: base.map(|b| Box::new(b.transform(func))),
            },
            TypedFunction::Ln { expr } => TypedFunction::Ln {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Pow { base, exponent } => TypedFunction::Pow {
                base: Box::new(base.transform(func)),
                exponent: Box::new(exponent.transform(func)),
            },
            TypedFunction::Sqrt { expr } => TypedFunction::Sqrt {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Greatest { exprs } => TypedFunction::Greatest {
                exprs: exprs.into_iter().map(|e| e.transform(func)).collect(),
            },
            TypedFunction::Least { exprs } => TypedFunction::Least {
                exprs: exprs.into_iter().map(|e| e.transform(func)).collect(),
            },
            TypedFunction::Mod { left, right } => TypedFunction::Mod {
                left: Box::new(left.transform(func)),
                right: Box::new(right.transform(func)),
            },
            TypedFunction::NumberToStr { exprs } => TypedFunction::NumberToStr {
                exprs: exprs.into_iter().map(|e| e.transform(func)).collect(),
            },

            // Conversion
            TypedFunction::Hex { expr } => TypedFunction::Hex {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Unhex { expr } => TypedFunction::Unhex {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Md5 { expr } => TypedFunction::Md5 {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Sha { expr } => TypedFunction::Sha {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Sha256 { expr } => TypedFunction::Sha256 {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Sha512 { expr } => TypedFunction::Sha512 {
                expr: Box::new(expr.transform(func)),
            },
            TypedFunction::Sha2 { expr, bit_length } => TypedFunction::Sha2 {
                expr: Box::new(expr.transform(func)),
                bit_length: Box::new(bit_length.transform(func)),
            },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Operators
// ═══════════════════════════════════════════════════════════════════════

/// Binary operators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    MysqlDivide,
    Power,
    PostgresPower,
    IntDiv,
    Modulo,
    Eq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    NullSafeEq,
    FatArrow,
    Assign,
    And,
    Or,
    Xor,
    Concat,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
    Is,
    Match,
    ArrayContains,
    ArrayContainedBy,
    RangeAdjacent,
    AtTimeZone,
    Distance,
    Distance3D,
    Glob,
    /// `->` JSON access operator
    Arrow,
    /// `->>` JSON text access
    DoubleArrow,
}

/// Unary operators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Minus,
    Plus,
    BitwiseNot,
    Variadic,
}

// ═══════════════════════════════════════════════════════════════════════
// DML statements
// ═══════════════════════════════════════════════════════════════════════

/// An INSERT statement, now supporting INSERT ... SELECT and ON CONFLICT.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    /// Whether this statement was written as `REPLACE INTO`.
    #[serde(default)]
    pub replace: bool,
    /// Whether this statement was written as MySQL `INSERT IGNORE`.
    #[serde(default)]
    pub ignore: bool,
    /// Raw source SQL for statements Python SQLGlot preserves as unsupported commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_sql: Option<String>,
    pub table: TableRef,
    pub columns: Vec<String>,
    pub source: InsertSource,
    /// Optional source alias after VALUES/query source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_alias: Option<String>,
    /// ON CONFLICT / ON DUPLICATE KEY
    pub on_conflict: Option<OnConflict>,
    /// RETURNING clause
    pub returning: Vec<SelectItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InsertSource {
    Values(Vec<Vec<Expr>>),
    Query(Box<Statement>),
    Default,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnConflict {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_where: Option<Expr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_where: Option<Expr>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub duplicate_key: bool,
    pub action: ConflictAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictAction {
    DoNothing,
    DoUpdate(Vec<(String, Expr)>),
}

/// An UPDATE statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub table: TableRef,
    pub assignments: Vec<(String, Expr)>,
    pub from: Option<FromClause>,
    pub where_clause: Option<Expr>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_by: Vec<OrderByItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<Expr>,
    pub returning: Vec<SelectItem>,
}

/// A DELETE statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub table: TableRef,
    pub using: Option<FromClause>,
    pub where_clause: Option<Expr>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_by: Vec<OrderByItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<Expr>,
    pub returning: Vec<SelectItem>,
}

// ═══════════════════════════════════════════════════════════════════════
// MERGE statement
// ═══════════════════════════════════════════════════════════════════════

/// A MERGE (UPSERT) statement.
///
/// MERGE INTO target USING source ON condition
///   WHEN MATCHED THEN UPDATE SET ...
///   WHEN NOT MATCHED THEN INSERT ...
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergeStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub target: TableRef,
    pub source: TableSource,
    pub on: Expr,
    pub clauses: Vec<MergeClause>,
    /// OUTPUT clause (T-SQL extension)
    pub output: Vec<SelectItem>,
}

/// A single WHEN MATCHED / WHEN NOT MATCHED clause in a MERGE statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergeClause {
    pub kind: MergeClauseKind,
    /// Optional additional condition: WHEN MATCHED AND <condition>
    pub condition: Option<Expr>,
    pub action: MergeAction,
}

/// The kind of WHEN clause in a MERGE statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeClauseKind {
    /// WHEN MATCHED
    Matched,
    /// WHEN NOT MATCHED (BY TARGET) — standard SQL and most dialects
    NotMatched,
    /// WHEN NOT MATCHED BY SOURCE — T-SQL extension
    NotMatchedBySource,
}

/// The action to take in a MERGE WHEN clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MergeAction {
    /// UPDATE SET col = val, ...
    Update(Vec<(String, Expr)>),
    /// INSERT (columns) VALUES (values)
    Insert {
        columns: Vec<String>,
        values: Vec<Expr>,
    },
    /// INSERT ROW (BigQuery)
    InsertRow,
    /// DELETE
    Delete,
    /// DO NOTHING
    DoNothing,
}

// ═══════════════════════════════════════════════════════════════════════
// DDL statements
// ═══════════════════════════════════════════════════════════════════════

/// A CREATE TABLE statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTableStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub if_not_exists: bool,
    pub temporary: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub or_replace: bool,
    pub table: TableRef,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CreateTableOption>,
    /// CREATE TABLE ... AS SELECT ...
    pub as_select: Option<Box<Statement>>,
    /// CREATE TABLE … (LIKE other_table) — captured when LIKE appears
    /// inside the column-list parens. Python SQLGlot rewrites this to
    /// an `AS SELECT * FROM <ref> LIMIT 0` clause rendered next to any
    /// preceding columns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_in_columns: Option<TableRef>,
}

/// Table-level options following a CREATE TABLE schema, mostly used by
/// MySQL-family dialects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateTableOption {
    Engine(String),
    AutoIncrement(String),
    CharacterSet { default: bool, value: String },
    Collate { default: bool, value: String },
    Comment(String),
    RowFormat(String),
    Unknown { name: String, value: Option<String> },
}

/// Table-level constraints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableConstraint {
    PrimaryKey {
        name: Option<String>,
        columns: Vec<String>,
    },
    Unique {
        name: Option<String>,
        columns: Vec<String>,
    },
    ForeignKey {
        name: Option<String>,
        columns: Vec<String>,
        ref_table: TableRef,
        ref_columns: Vec<String>,
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
    },
    Check {
        name: Option<String>,
        expr: Expr,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferentialAction {
    Cascade,
    Restrict,
    NoAction,
    SetNull,
    SetDefault,
}

/// CREATE INDEX statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateIndexStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub name: Option<String>,
    pub table: TableRef,
    pub columns: Vec<OrderByItem>,
    pub unique: bool,
    pub if_not_exists: bool,
    pub concurrently: bool,
    pub using: Option<String>,
    /// Partial-index predicate (`WHERE ...`). Supported by SQLite and Postgres.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub where_clause: Option<Expr>,
}

/// DROP INDEX statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropIndexStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub name: String,
    pub table: Option<TableRef>,
    pub if_exists: bool,
    pub concurrently: bool,
}

/// A column definition in CREATE TABLE.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    #[serde(default)]
    pub name_quote_style: QuoteStyle,
    pub data_type: DataType,
    pub nullable: Option<bool>,
    pub default: Option<Expr>,
    pub primary_key: bool,
    pub unique: bool,
    /// True when UNIQUE appeared before NOT NULL in the source column
    /// definition. SQLGlot preserves source constraint order, so we carry
    /// this to render `UNIQUE NOT NULL` rather than the default
    /// `NOT NULL UNIQUE`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unique_before_not_null: bool,
    pub auto_increment: bool,
    #[serde(default)]
    pub auto_increment_before_primary_key: bool,
    #[serde(default)]
    pub primary_key_from_table_constraint: bool,
    /// Whether the column's auto_increment came from a postgres-style
    /// `GENERATED ... AS IDENTITY` clause (which implies NOT NULL, so
    /// sqlite output drops an explicit NOT NULL).
    #[serde(default)]
    pub auto_increment_from_identity: bool,
    pub collation: Option<String>,
    pub comment: Option<String>,
    /// Inline foreign-key reference spec, captured verbatim from source
    /// (`[CONSTRAINT name] [FOREIGN KEY] REFERENCES ...`). Preserved as-is
    /// since SQLGlot round-trips it unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_spec: Option<String>,
    /// Computed/generated column expression: `AS (<expr>)`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_as: Option<Expr>,
    /// Whether the generated column is STORED (rendered as PERSISTED).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub generated_stored: bool,
}

/// ALTER TABLE statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlterTableStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub table: TableRef,
    pub actions: Vec<AlterTableAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlterTableAction {
    AddColumn(ColumnDef),
    DropColumn {
        name: String,
        if_exists: bool,
    },
    /// `SET ...` table-property forms. SQLGlot drops unrepresentable
    /// property keywords (LOGGED, TABLESPACE, COMMENT=, ...) leaving a bare
    /// `SET`, and unwraps parenthesized assignment lists (`SET (a = 1)` →
    /// `SET a = 1`).
    SetProperties {
        assignments: Vec<Expr>,
    },
    RenameColumn {
        old_name: String,
        new_name: String,
    },
    AlterColumnType {
        name: String,
        data_type: DataType,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        tail: String,
    },
    AlterColumnRaw {
        name: String,
        tail: String,
    },
    /// MySQL CHANGE [COLUMN] old_name new_name <full column definition>
    ChangeColumn {
        old_name: String,
        new_column: ColumnDef,
        /// Anything trailing the column definition (e.g. `FIRST`,
        /// `AFTER col`) that we don't fully model — preserved raw.
        #[serde(default, skip_serializing_if = "String::is_empty")]
        tail: String,
        /// True when the source kw was MODIFY (single-name) rather than CHANGE (old + new).
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_modify: bool,
    },
    AddConstraint(TableConstraint),
    DropConstraint {
        name: String,
    },
    RenameTable {
        new_name: String,
    },
}

/// CREATE VIEW statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateViewStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub name: TableRef,
    pub columns: Vec<String>,
    pub query: Box<Statement>,
    pub or_replace: bool,
    pub materialized: bool,
    pub if_not_exists: bool,
}

/// DROP VIEW statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropViewStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub name: TableRef,
    pub if_exists: bool,
    pub materialized: bool,
}

/// TRUNCATE TABLE statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TruncateStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub targets: Vec<TruncateTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TruncateTarget {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub only: bool,
    pub table: TableRef,
}

/// Transaction control statements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatement {
    Begin,
    Commit,
    CommitAndChain { chain: bool },
    Rollback,
    Savepoint(String),
    ReleaseSavepoint(String),
    RollbackTo(String),
}

/// EXPLAIN statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub analyze: bool,
    pub statement: Box<Statement>,
}

/// USE database statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub name: String,
}

/// A DROP TABLE statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropTableStatement {
    /// Comments attached to this statement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,
    pub if_exists: bool,
    pub table: TableRef,
    pub cascade: bool,
}

// ═══════════════════════════════════════════════════════════════════════
// Data types
// ═══════════════════════════════════════════════════════════════════════

/// SQL data types. Significantly expanded to match sqlglot's DataType.Type enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    // Numeric
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    Float,
    Double,
    Decimal {
        precision: Option<u32>,
        scale: Option<u32>,
    },
    Numeric {
        precision: Option<u32>,
        scale: Option<u32>,
    },
    Real,

    // String
    Varchar(Option<u32>),
    Char(Option<u32>),
    Text,
    String,
    Binary(Option<u32>),
    Varbinary(Option<u32>),

    // Boolean
    Boolean,

    // Date/Time
    Date,
    Time {
        precision: Option<u32>,
    },
    Timestamp {
        precision: Option<u32>,
        with_tz: bool,
    },
    Interval,
    DateTime,

    // Binary
    Blob,
    Bytea,
    Bytes,

    // JSON
    Json,
    Jsonb,

    // UUID
    Uuid,

    // Complex types
    Array(Option<Box<DataType>>),
    Map {
        key: Box<DataType>,
        value: Box<DataType>,
    },
    Struct(Vec<(String, DataType)>),
    Tuple(Vec<DataType>),

    // Special
    Null,
    Collate {
        data_type: Box<DataType>,
        collation: String,
    },
    Unknown(String),
    Variant,
    Object,
    Xml,
    Inet,
    Cidr,
    Macaddr,
    Bit(Option<u32>),
    Money,
    Serial,
    BigSerial,
    SmallSerial,
    Regclass,
    Regtype,
    Hstore,
    Geography,
    Geometry,
    Super,
}

// ═══════════════════════════════════════════════════════════════════════
// Expression tree traversal helpers
// ═══════════════════════════════════════════════════════════════════════

impl Expr {
    /// Recursively walk this expression tree, calling `visitor` on each node.
    /// If `visitor` returns `false`, children of that node are not visited.
    pub fn walk<F>(&self, visitor: &mut F)
    where
        F: FnMut(&Expr) -> bool,
    {
        if !visitor(self) {
            return;
        }
        match self {
            Expr::BinaryOp { left, right, .. } => {
                left.walk(visitor);
                right.walk(visitor);
            }
            Expr::UnaryOp { expr, .. } => expr.walk(visitor),
            Expr::Function {
                args,
                arg_order_by,
                arg_limit,
                filter,
                ..
            } => {
                for arg in args {
                    arg.walk(visitor);
                }
                for item in arg_order_by {
                    item.expr.walk(visitor);
                }
                if let Some(limit) = arg_limit {
                    limit.walk(visitor);
                }
                if let Some(f) = filter {
                    f.walk(visitor);
                }
            }
            Expr::WithinGroup {
                expr,
                order_by,
                filter,
                ..
            } => {
                expr.walk(visitor);
                for item in order_by {
                    item.expr.walk(visitor);
                }
                if let Some(f) = filter {
                    f.walk(visitor);
                }
            }
            Expr::Between {
                expr, low, high, ..
            } => {
                expr.walk(visitor);
                low.walk(visitor);
                high.walk(visitor);
            }
            Expr::InList { expr, list, .. } => {
                expr.walk(visitor);
                for item in list {
                    item.walk(visitor);
                }
            }
            Expr::InSubquery { expr, .. } => {
                expr.walk(visitor);
            }
            Expr::IsNull { expr, .. } => expr.walk(visitor),
            Expr::IsBool { expr, .. } => expr.walk(visitor),
            Expr::AnyOp { expr, right, .. } | Expr::AllOp { expr, right, .. } => {
                expr.walk(visitor);
                right.walk(visitor);
            }
            Expr::Like { expr, pattern, .. }
            | Expr::ILike { expr, pattern, .. }
            | Expr::SimilarTo { expr, pattern, .. } => {
                expr.walk(visitor);
                pattern.walk(visitor);
            }
            Expr::Case {
                operand,
                when_clauses,
                else_clause,
            } => {
                if let Some(op) = operand {
                    op.walk(visitor);
                }
                for (cond, result) in when_clauses {
                    cond.walk(visitor);
                    result.walk(visitor);
                }
                if let Some(el) = else_clause {
                    el.walk(visitor);
                }
            }
            Expr::Nested(inner) => inner.walk(visitor),
            Expr::Cast { expr, .. } | Expr::TryCast { expr, .. } => expr.walk(visitor),
            Expr::Extract { expr, .. } => expr.walk(visitor),
            Expr::Interval { value, .. } => value.walk(visitor),
            Expr::ArrayLiteral(items)
            | Expr::SqliteArrayLiteral(items)
            | Expr::Tuple(items)
            | Expr::Coalesce { items, .. } => {
                for item in items {
                    item.walk(visitor);
                }
            }
            Expr::If {
                condition,
                true_val,
                false_val,
            } => {
                condition.walk(visitor);
                true_val.walk(visitor);
                if let Some(fv) = false_val {
                    fv.walk(visitor);
                }
            }
            Expr::NullIf { expr, r#else } => {
                expr.walk(visitor);
                r#else.walk(visitor);
            }
            Expr::Collate { expr, .. } => expr.walk(visitor),
            Expr::Alias { expr, .. } => expr.walk(visitor),
            Expr::ArrayIndex { expr, index } => {
                expr.walk(visitor);
                index.walk(visitor);
            }
            Expr::PostgresArrayIndex { expr, index } => {
                expr.walk(visitor);
                index.walk(visitor);
            }
            Expr::JsonAccess { expr, path, .. } => {
                expr.walk(visitor);
                path.walk(visitor);
            }
            Expr::Lambda { body, .. } => body.walk(visitor),
            Expr::TypedFunction { func, filter, .. } => {
                func.walk_children(visitor);
                if let Some(f) = filter {
                    f.walk(visitor);
                }
            }
            Expr::Cube { exprs } | Expr::Rollup { exprs } => {
                for item in exprs {
                    item.walk(visitor);
                }
            }
            Expr::GroupingSets { sets } => {
                for item in sets {
                    item.walk(visitor);
                }
            }
            Expr::Commented { expr, .. } => expr.walk(visitor),
            // Leaf nodes
            Expr::Column { .. }
            | Expr::Number(_)
            | Expr::HexString(_)
            | Expr::StringLiteral(_)
            | Expr::JsonKey(_)
            | Expr::EscapedStringLiteral(_)
            | Expr::Boolean(_)
            | Expr::Null
            | Expr::Wildcard
            | Expr::Star
            | Expr::Parameter(_)
            | Expr::PostgresParameter(_)
            | Expr::TypeExpr(_)
            | Expr::QualifiedWildcard { .. }
            | Expr::Default
            | Expr::Subquery(_)
            | Expr::Exists { .. } => {}
        }
    }

    /// Find the first expression matching the predicate.
    #[must_use]
    pub fn find<F>(&self, predicate: &F) -> Option<&Expr>
    where
        F: Fn(&Expr) -> bool,
    {
        let mut result = None;
        self.walk(&mut |expr| {
            if result.is_some() {
                return false;
            }
            if predicate(expr) {
                result = Some(expr as *const Expr);
                false
            } else {
                true
            }
        });
        // SAFETY: the pointer is valid as long as self is alive
        result.map(|p| unsafe { &*p })
    }

    /// Find all expressions matching the predicate.
    #[must_use]
    pub fn find_all<F>(&self, predicate: &F) -> Vec<&Expr>
    where
        F: Fn(&Expr) -> bool,
    {
        let mut results: Vec<*const Expr> = Vec::new();
        self.walk(&mut |expr| {
            if predicate(expr) {
                results.push(expr as *const Expr);
            }
            true
        });
        results.into_iter().map(|p| unsafe { &*p }).collect()
    }

    /// Transform this expression tree by applying a function to each node.
    /// The function can return a new expression to replace the current one.
    #[must_use]
    pub fn transform<F>(self, func: &F) -> Expr
    where
        F: Fn(Expr) -> Expr,
    {
        let transformed = match self {
            Expr::BinaryOp { left, op, right } => Expr::BinaryOp {
                left: Box::new(left.transform(func)),
                op,
                right: Box::new(right.transform(func)),
            },
            Expr::UnaryOp { op, expr } => Expr::UnaryOp {
                op,
                expr: Box::new(expr.transform(func)),
            },
            Expr::Function {
                name,
                args,
                distinct,
                raw_order_nulls,
                arg_order_by,
                arg_limit,
                filter,
                over,
            } => Expr::Function {
                name,
                args: args.into_iter().map(|a| a.transform(func)).collect(),
                distinct,
                raw_order_nulls,
                arg_order_by: arg_order_by
                    .into_iter()
                    .map(|mut item| {
                        item.expr = item.expr.transform(func);
                        item
                    })
                    .collect(),
                arg_limit: arg_limit.map(|limit| Box::new(limit.transform(func))),
                filter: filter.map(|f| Box::new(f.transform(func))),
                over,
            },
            Expr::WithinGroup {
                expr,
                order_by,
                filter,
                over,
            } => Expr::WithinGroup {
                expr: Box::new(expr.transform(func)),
                order_by: order_by
                    .into_iter()
                    .map(|mut item| {
                        item.expr = item.expr.transform(func);
                        item
                    })
                    .collect(),
                filter: filter.map(|f| Box::new(f.transform(func))),
                over,
            },
            Expr::Nested(inner) => Expr::Nested(Box::new(inner.transform(func))),
            Expr::Cast { expr, data_type } => Expr::Cast {
                expr: Box::new(expr.transform(func)),
                data_type,
            },
            Expr::Between {
                expr,
                low,
                high,
                negated,
            } => Expr::Between {
                expr: Box::new(expr.transform(func)),
                low: Box::new(low.transform(func)),
                high: Box::new(high.transform(func)),
                negated,
            },
            Expr::Case {
                operand,
                when_clauses,
                else_clause,
            } => Expr::Case {
                operand: operand.map(|o| Box::new(o.transform(func))),
                when_clauses: when_clauses
                    .into_iter()
                    .map(|(c, r)| (c.transform(func), r.transform(func)))
                    .collect(),
                else_clause: else_clause.map(|e| Box::new(e.transform(func))),
            },
            Expr::IsBool {
                expr,
                value,
                negated,
            } => Expr::IsBool {
                expr: Box::new(expr.transform(func)),
                value,
                negated,
            },
            Expr::AnyOp { expr, op, right } => Expr::AnyOp {
                expr: Box::new(expr.transform(func)),
                op,
                right: Box::new(right.transform(func)),
            },
            Expr::AllOp { expr, op, right } => Expr::AllOp {
                expr: Box::new(expr.transform(func)),
                op,
                right: Box::new(right.transform(func)),
            },
            Expr::TypedFunction {
                func: tf,
                filter,
                over,
            } => Expr::TypedFunction {
                func: tf.transform_children(func),
                filter: filter.map(|f| Box::new(f.transform(func))),
                over,
            },
            Expr::InList {
                expr,
                list,
                negated,
            } => Expr::InList {
                expr: Box::new(expr.transform(func)),
                list: list.into_iter().map(|e| e.transform(func)).collect(),
                negated,
            },
            Expr::InSubquery {
                expr,
                subquery,
                negated,
            } => Expr::InSubquery {
                expr: Box::new(expr.transform(func)),
                subquery, // Statement — not transformable via Expr func
                negated,
            },
            Expr::IsNull { expr, negated } => Expr::IsNull {
                expr: Box::new(expr.transform(func)),
                negated,
            },
            Expr::Like {
                expr,
                pattern,
                negated,
                escape,
            } => Expr::Like {
                expr: Box::new(expr.transform(func)),
                pattern: Box::new(pattern.transform(func)),
                negated,
                escape: escape.map(|e| Box::new(e.transform(func))),
            },
            Expr::ILike {
                expr,
                pattern,
                negated,
                lower_on_ansi,
                escape,
            } => Expr::ILike {
                expr: Box::new(expr.transform(func)),
                pattern: Box::new(pattern.transform(func)),
                negated,
                lower_on_ansi,
                escape: escape.map(|e| Box::new(e.transform(func))),
            },
            Expr::SimilarTo {
                expr,
                pattern,
                escape,
            } => Expr::SimilarTo {
                expr: Box::new(expr.transform(func)),
                pattern: Box::new(pattern.transform(func)),
                escape: escape.map(|e| Box::new(e.transform(func))),
            },
            Expr::TryCast { expr, data_type } => Expr::TryCast {
                expr: Box::new(expr.transform(func)),
                data_type,
            },
            Expr::Extract { field, expr } => Expr::Extract {
                field,
                expr: Box::new(expr.transform(func)),
            },
            Expr::Interval {
                value,
                unit,
                unit_text,
            } => Expr::Interval {
                value: Box::new(value.transform(func)),
                unit,
                unit_text,
            },
            Expr::ArrayLiteral(elems) => {
                Expr::ArrayLiteral(elems.into_iter().map(|e| e.transform(func)).collect())
            }
            Expr::SqliteArrayLiteral(elems) => {
                Expr::SqliteArrayLiteral(elems.into_iter().map(|e| e.transform(func)).collect())
            }
            Expr::Tuple(elems) => {
                Expr::Tuple(elems.into_iter().map(|e| e.transform(func)).collect())
            }
            Expr::Coalesce {
                items,
                is_nvl,
                is_null,
                source_name,
            } => Expr::Coalesce {
                items: items.into_iter().map(|e| e.transform(func)).collect(),
                is_nvl,
                is_null,
                source_name,
            },
            Expr::If {
                condition,
                true_val,
                false_val,
            } => Expr::If {
                condition: Box::new(condition.transform(func)),
                true_val: Box::new(true_val.transform(func)),
                false_val: false_val.map(|f| Box::new(f.transform(func))),
            },
            Expr::NullIf { expr, r#else } => Expr::NullIf {
                expr: Box::new(expr.transform(func)),
                r#else: Box::new(r#else.transform(func)),
            },
            Expr::Collate { expr, collation } => Expr::Collate {
                expr: Box::new(expr.transform(func)),
                collation,
            },
            Expr::Alias { expr, name } => Expr::Alias {
                expr: Box::new(expr.transform(func)),
                name,
            },
            Expr::ArrayIndex { expr, index } => Expr::ArrayIndex {
                expr: Box::new(expr.transform(func)),
                index: Box::new(index.transform(func)),
            },
            Expr::PostgresArrayIndex { expr, index } => Expr::PostgresArrayIndex {
                expr: Box::new(expr.transform(func)),
                index: Box::new(index.transform(func)),
            },
            Expr::JsonAccess {
                expr,
                path,
                as_text,
            } => Expr::JsonAccess {
                expr: Box::new(expr.transform(func)),
                path: Box::new(path.transform(func)),
                as_text,
            },
            Expr::Lambda { params, body } => Expr::Lambda {
                params,
                body: Box::new(body.transform(func)),
            },
            Expr::Cube { exprs } => Expr::Cube {
                exprs: exprs.into_iter().map(|e| e.transform(func)).collect(),
            },
            Expr::Rollup { exprs } => Expr::Rollup {
                exprs: exprs.into_iter().map(|e| e.transform(func)).collect(),
            },
            Expr::GroupingSets { sets } => Expr::GroupingSets {
                sets: sets.into_iter().map(|e| e.transform(func)).collect(),
            },
            Expr::Commented { expr, comments } => Expr::Commented {
                expr: Box::new(expr.transform(func)),
                comments,
            },
            other => other,
        };
        func(transformed)
    }

    /// Check whether this expression is a column reference.
    #[must_use]
    pub fn is_column(&self) -> bool {
        matches!(self, Expr::Column { .. })
    }

    /// Check whether this expression is a literal value (number, string, bool, null).
    #[must_use]
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Expr::Number(_)
                | Expr::HexString(_)
                | Expr::StringLiteral(_)
                | Expr::EscapedStringLiteral(_)
                | Expr::Boolean(_)
                | Expr::Null
        )
    }

    /// Get the SQL representation of this expression for display purposes.
    /// For full generation, use the Generator.
    #[must_use]
    pub fn sql(&self) -> String {
        use crate::generator::Generator;
        Generator::expr_to_sql(self)
    }
}

/// Helper: collect all column references from an expression.
#[must_use]
pub fn find_columns(expr: &Expr) -> Vec<&Expr> {
    expr.find_all(&|e| matches!(e, Expr::Column { .. }))
}

/// Helper: collect all table references from a statement.
#[must_use]
pub fn find_tables(statement: &Statement) -> Vec<&TableRef> {
    match statement {
        Statement::Select(sel) => {
            let mut tables = Vec::new();
            if let Some(from) = &sel.from {
                collect_table_refs_from_source(&from.source, &mut tables);
            }
            for join in &sel.joins {
                collect_table_refs_from_source(&join.table, &mut tables);
            }
            tables
        }
        Statement::Insert(ins) => vec![&ins.table],
        Statement::Update(upd) => vec![&upd.table],
        Statement::Delete(del) => vec![&del.table],
        Statement::CreateTable(ct) => vec![&ct.table],
        Statement::DropTable(dt) => vec![&dt.table],
        Statement::CreateIndex(idx) => vec![&idx.table],
        Statement::DropIndex(idx) => idx.table.iter().collect(),
        _ => vec![],
    }
}

fn collect_table_refs_from_source<'a>(source: &'a TableSource, tables: &mut Vec<&'a TableRef>) {
    match source {
        TableSource::Table(table_ref) => tables.push(table_ref),
        TableSource::Subquery { .. } => {}
        TableSource::TableFunction { .. } => {}
        TableSource::RawTableFunction { .. } => {}
        TableSource::TableWithTails { table, .. } => tables.push(table),
        TableSource::Raw { .. } => {}
        TableSource::RowsFrom { .. } => {}
        TableSource::Values { .. } => {}
        TableSource::Lateral { source } => collect_table_refs_from_source(source, tables),
        TableSource::Pivot { source, .. } | TableSource::Unpivot { source, .. } => {
            collect_table_refs_from_source(source, tables);
        }
        TableSource::Unnest { .. } => {}
    }
}
